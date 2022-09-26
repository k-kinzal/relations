use anyhow::Result;
use sqlx::mysql::MySqlRow;
use sqlx::{Executor, MySql, Row};

pub async fn show_tables<'a, E>(executor: E, database_name: &str) -> Result<Vec<String>>
where
    E: Executor<'a, Database = MySql>,
{
    let query = format!("SHOW TABLES FROM `{}`", database_name);
    let rows = sqlx::query(&query)
        .try_map(|row: MySqlRow| row.try_get(0))
        .fetch_all(executor)
        .await?;
    Ok(rows)
}

pub async fn show_indexes<'a, E>(
    executor: E,
    database_name: &str,
    table_name: &str,
) -> Result<
    Vec<(
        String,         // Table
        bool,           // Non_unique
        String,         // Key_name
        i64,            // Seq_in_index
        String,         // Column_name
        String,         // Collation
        i64,            // Cardinality
        Option<i64>,    // Sub_part
        Option<String>, // Packed
        bool,           // Null
        String,         // Index_type
        String,         // Comment
        String,         // Index_comment
    )>,
>
where
    E: Executor<'a, Database = MySql>,
{
    let query = format!("SHOW INDEXES FROM `{}`.`{}`", database_name, table_name);
    let rows = sqlx::query(&query)
        .try_map(|row: MySqlRow| {
            Ok((
                row.try_get("Table")?,
                row.try_get::<i32, &str>("Non_unique")? == 1,
                row.try_get("Key_name")?,
                row.try_get("Seq_in_index")?,
                row.try_get("Column_name")?,
                row.try_get("Collation")?,
                row.try_get("Cardinality")?,
                row.try_get("Sub_part")?,
                row.try_get("Packed")?,
                row.try_get::<&str, &str>("Null")? == "YES",
                row.try_get("Index_type")?,
                row.try_get("Comment")?,
                row.try_get("Index_comment")?,
            ))
        })
        .fetch_all(executor)
        .await?;
    Ok(rows)
}

pub async fn describe_table<'a, E>(
    executor: E,
    database_name: &str,
    table_name: &str,
) -> Result<
    Vec<(
        String,         // Field
        String,         // Type
        bool,           // Null
        Option<String>, // Key
        Option<String>, // Default
        Option<String>, // Extra
    )>,
>
where
    E: Executor<'a, Database = MySql>,
{
    let query = format!("DESCRIBE `{}`.`{}`", database_name, table_name);
    let rows = sqlx::query(&query)
        .try_map(|row: MySqlRow| {
            Ok((
                row.try_get("Field")?,
                row.try_get("Type")?,
                row.try_get::<&str, &str>("Null")? == "YES",
                row.try_get("Key")?,
                row.try_get("Default")?,
                row.try_get("Extra")?,
            ))
        })
        .fetch_all(executor)
        .await?;
    Ok(rows)
}
