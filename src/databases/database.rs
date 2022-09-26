use itertools::Itertools;
use std::collections::HashSet;

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Table {
    pub(crate) name: String,
    pub(crate) database: String,
    pub(crate) columns: Vec<Column>,
    pub(crate) indexes: Vec<Index>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Column {
    pub(crate) name: String,
    pub(crate) data_type: String,
    pub(crate) is_auto_increment: bool,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Index {
    pub(crate) name: String,
    pub(crate) columns: Vec<Column>,
    pub(crate) is_unique: bool,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Relation {
    pub(crate) table: Table,
    pub(crate) columns: Vec<Column>,
    pub(crate) parent_table: Table,
    pub(crate) parent_columns: Vec<Column>,
}

pub(crate) fn relations(tables: Vec<Table>, rules: Vec<Rule>) -> Vec<Relation> {
    let mut relations = Vec::new();
    for parent_table in tables.iter() {
        for index in parent_table.indexes.iter() {
            for child_table in tables.iter() {
                for child_columns in child_table.columns.iter().combinations(index.columns.len()) {
                    let is_detect_relation =
                        child_columns
                            .iter()
                            .enumerate()
                            .fold(true, |acc, (i, child_column)| {
                                acc && index
                                    .columns
                                    .get(i)
                                    .and_then(|parent_column| {
                                        rules
                                            .iter()
                                            .all(|rule| {
                                                rule(
                                                    parent_table,
                                                    parent_column,
                                                    child_table,
                                                    child_column,
                                                )
                                            })
                                            .then(|| ())
                                    })
                                    .is_some()
                            });
                    if is_detect_relation {
                        relations.push(Relation {
                            table: child_table.clone(),
                            columns: child_columns.into_iter().cloned().collect(),
                            parent_table: parent_table.clone(),
                            parent_columns: index.columns.clone(),
                        });
                    }
                }
            }
        }
    }

    relations
}

pub(crate) type Rule = Box<dyn Fn(&Table, &Column, &Table, &Column) -> bool>;

pub(crate) fn rule_ends_with_excepting_the_prefixes(prefixes: HashSet<String>) -> Rule {
    Box::new(move |parent_table, parent_column, _, child_column| {
        prefixes.iter().fold(false, |acc, prefix| {
            let s = format!(
                "{}_{}",
                parent_table.name.trim_start_matches(prefix),
                parent_column.name
            );
            acc || child_column.name.ends_with(&s)
        })
    })
}

pub(crate) fn rule_ends_with() -> Rule {
    Box::new(|parent_table, parent_column, _, child_column| {
        let s = format!("{}_{}", parent_table.name, parent_column.name);
        child_column.name.ends_with(&s)
    })
}

pub(crate) fn rule_same_data_type() -> Rule {
    Box::new(|_, parent_column, _, child_column| parent_column.data_type == child_column.data_type)
}
