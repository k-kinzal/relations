use crate::configs::{AdditionalRelation, Config};
use crate::databases::{
    mysql, relations, rule_ends_with, rule_ends_with_excepting_the_prefixes, rule_same_data_type,
    Rule as DetectRule,
};
use anyhow::Result;
use clap::Subcommand;
use itertools::Itertools;
use sqlx::ConnectOptions;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use url::Url;

#[derive(PartialEq, Eq, Debug)]
pub enum Rule {
    EndsWith,
    EndsWithExceptingThePrefixes,
    SomeDataType,
}

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ends-with" => Ok(Rule::EndsWith),
            "ends-with-excepting-the-prefixes" => Ok(Rule::EndsWithExceptingThePrefixes),
            "some-data-type" => Ok(Rule::SomeDataType),
            _ => Err(Self::Err::from(format!("unsupported rule: `{}`", s))),
        }
    }
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum GenerateSubcommands {
    #[clap(about = "Generate configs file from database")]
    #[clap(arg_required_else_help = true)]
    Config {
        /// Database URL
        #[clap(long)]
        database_url: String,

        /// Rules for detecting relations. (ends-with, ends-with-excepting-the-prefixes, some-data-type)
        /// By default, column names that end with the table_name_column_name of the parent table are detected as relations.
        #[clap(short, long, default_value = "ends-with")]
        rules: Vec<String>,

        /// Prefixes to be specified for detection by the ends-with-excepting-the-prefixes rule
        #[clap(long)]
        ends_with_excepting_prefixes: Vec<String>,

        /// Output file path
        /// By default, .tbl.yml is output to the current directory.
        #[clap(short, long, default_value = ".tbl.yml")]
        output: String,
    },
}

pub async fn run_generate(command: GenerateSubcommands) -> Result<()> {
    match command {
        GenerateSubcommands::Config {
            database_url,
            rules,
            ends_with_excepting_prefixes,
            output,
        } => {
            let database_url = Url::parse(database_url.as_str())?;
            let database_names = database_url
                .path_segments()
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            let ends_with_excepting_prefixes = ends_with_excepting_prefixes
                .into_iter()
                .unique()
                .collect::<HashSet<String>>();
            let rules = rules
                .into_iter()
                .unique()
                .map(|s| {
                    Rule::from_str(&s)
                        .map(|rule| match rule {
                            Rule::EndsWith => rule_ends_with(),
                            Rule::EndsWithExceptingThePrefixes => {
                                rule_ends_with_excepting_the_prefixes(
                                    ends_with_excepting_prefixes.clone(),
                                )
                            }
                            Rule::SomeDataType => rule_same_data_type(),
                        })
                        .map_err(|e| anyhow::Error::msg(e.to_string()))
                })
                .collect::<Result<Vec<DetectRule>>>()?;

            run_generate_config(database_url, database_names, rules, output).await
        }
    }
}

async fn run_generate_config(
    database_url: Url,
    database_names: HashSet<String>,
    rules: Vec<DetectRule>,
    output: String,
) -> Result<()> {
    let tables = match database_url.scheme() {
        "mysql" => {
            use sqlx::mysql::MySqlConnectOptions;
            use sqlx::mysql::MySqlPoolOptions;

            let opt = MySqlConnectOptions::from_str(database_url.as_str())?
                .disable_statement_logging()
                .clone();
            let conn = MySqlPoolOptions::new().connect_with(opt).await?;
            mysql::tables(&conn, database_names).await?
        }
        _ => unimplemented!("unsupported database: `{}`", database_url.as_str()),
    };

    let relations = relations(tables, rules);

    let config = Config {
        name: database_url
            .path_segments()
            .unwrap()
            .last()
            .unwrap()
            .to_string(),
        dsn: database_url.to_string(),
        doc_path: "dbdoc".to_string(),
        relations: Some(
            relations
                .into_iter()
                .sorted_by(|a, b| a.table.name.cmp(&b.table.name))
                .map(|r| AdditionalRelation {
                    def: Some(format!("{}->{}", r.table.name, r.parent_table.name)),
                    table: r.table.name,
                    columns: r.columns.into_iter().map(|c| c.name).collect(),
                    parent_table: r.parent_table.name,
                    parent_columns: r.parent_columns.into_iter().map(|c| c.name).collect(),
                })
                .collect(),
        ),
        ..Default::default()
    };

    fs::write(output, serde_yaml::to_string(&config)?)?;

    Ok(())
}
