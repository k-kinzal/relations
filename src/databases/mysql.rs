use crate::databases::{Column, Index, Table};
use anyhow::Result;
use sqlx::{Executor, MySql};
use std::collections::HashSet;

mod query;

pub(crate) use query::*;

pub(crate) async fn tables<'a, E>(
    executor: E,
    database_names: HashSet<String>,
) -> Result<Vec<Table>>
where
    E: Executor<'a, Database = MySql> + Clone,
{
    let mut table_names = HashSet::new();
    for database_name in database_names.iter() {
        let tables = show_tables(executor.clone(), database_name).await?;
        for table_name in tables.into_iter() {
            table_names.insert((database_name.clone(), table_name));
        }
    }

    let mut tables = Vec::new();
    for (database_name, table_name) in table_names.into_iter() {
        let mut columns = Vec::new();
        for (field, data_type, _, _, _, extra) in
            describe_table(executor.clone(), &database_name, &table_name).await?
        {
            columns.push(Column {
                name: field,
                data_type,
                is_auto_increment: extra == Some("auto_increment".to_string()),
            });
        }
        let mut indexes: Vec<Index> = Vec::new();
        for (_, non_unique, key_name, _, column_name, _, _, _, _, _, _, _, _) in
            show_indexes(executor.clone(), &database_name, &table_name).await?
        {
            let column = columns
                .iter()
                .find(|column| column.name == column_name)
                .unwrap()
                .clone();
            if let Some(i) = indexes.iter().position(|index| index.name == key_name) {
                let index = indexes[i].clone();
                indexes.insert(
                    i,
                    Index {
                        columns: index.columns.into_iter().chain(vec![column]).collect(),
                        ..index
                    },
                );
            } else {
                indexes.push(Index {
                    name: key_name.clone(),
                    columns: vec![column],
                    is_unique: !non_unique,
                });
            }
        }
        tables.push(Table {
            name: table_name,
            database: database_name,
            columns,
            indexes,
        });
    }

    Ok(tables)
}
