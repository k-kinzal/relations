use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration file compatible with .tbls.yml
/// ```yaml
/// ---
/// dsn: "mysql://root:password@localhost:3306/database"
/// relations:
///   - table: logs
///     columns:
///       - user_id
///     parentTable: users
///     parentColumns:
///       - id
///     # Relation definition
///     # Default is `Additional Relation`
///     def: logs->users
/// ```

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) labels: Option<Vec<String>>,
    pub(crate) dsn: String,
    pub(crate) doc_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) er: Option<ER>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub(crate) include: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub(crate) exclude: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) lint: Option<Lint>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub(crate) lint_exclude: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) relations: Option<Vec<AdditionalRelation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) comments: Option<Vec<AdditionalComment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) dict: Option<serde_yaml::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) templates: Option<Templates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) detect_virtual_relations: Option<DetectVirtualRelations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) required_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Format {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) adjust: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) sort: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) show_only_first_paragraph: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) hide_columns_without_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ER {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) skip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) comment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) distance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) font: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Lint {
    pub(crate) require_table_comment: RequireTableComment,
    pub(crate) require_column_comment: RequireColumnComment,
    pub(crate) require_index_comment: RequireIndexComment,
    pub(crate) require_constraint_comment: RequireConstraintComment,
    pub(crate) require_trigger_comment: RequireTriggerComment,
    pub(crate) unrelated_table: UnrelatedTable,
    pub(crate) column_count: ColumnCount,
    pub(crate) require_columns: RequireColumns,
    pub(crate) duplicate_relations: DuplicateRelations,
    pub(crate) require_foreign_key_index: RequireForeignKeyIndex,
    pub(crate) label_style_big_query: LabelStyleBigQuery,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireTableComment {
    pub(crate) enabled: bool,
    pub(crate) all_or_nothing: bool,
    pub(crate) exclude: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireColumnComment {
    pub(crate) enabled: bool,
    pub(crate) all_or_nothing: bool,
    pub(crate) exclude: Vec<String>,
    pub(crate) exclude_tables: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireIndexComment {
    pub(crate) enabled: bool,
    pub(crate) all_or_nothing: bool,
    pub(crate) exclude: Vec<String>,
    pub(crate) exclude_tables: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireConstraintComment {
    pub(crate) enabled: bool,
    pub(crate) all_or_nothing: bool,
    pub(crate) exclude: Vec<String>,
    pub(crate) exclude_tables: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireTriggerComment {
    pub(crate) enabled: bool,
    pub(crate) all_or_nothing: bool,
    pub(crate) exclude: Vec<String>,
    pub(crate) exclude_tables: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UnrelatedTable {
    pub(crate) enabled: bool,
    pub(crate) all_or_nothing: bool,
    pub(crate) exclude: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ColumnCount {
    pub(crate) enabled: bool,
    pub(crate) max: i32,
    pub(crate) exclude: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireColumns {
    pub(crate) enabled: bool,
    pub(crate) columns: Vec<RequireColumnsColumn>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireColumnsColumn {
    pub(crate) name: String,
    pub(crate) exclude: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DuplicateRelations {
    pub(crate) enabled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequireForeignKeyIndex {
    pub(crate) enabled: bool,
    pub(crate) exclude: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LabelStyleBigQuery {
    pub(crate) enabled: bool,
    pub(crate) exclude: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AdditionalRelation {
    pub(crate) table: String,
    pub(crate) columns: Vec<String>,
    pub(crate) parent_table: String,
    pub(crate) parent_columns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) def: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AdditionalComment {
    pub(crate) table: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) table_comment: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub(crate) column_comments: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub(crate) column_labels: HashMap<String, Vec<String>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub(crate) index_comments: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub(crate) constraint_comments: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub(crate) trigger_comments: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub(crate) labels: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Templates {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) md: Option<Md>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) dot: Option<Dot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) puml: Option<Puml>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Md {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) table: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Dot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) table: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Puml {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) table: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DetectVirtualRelations {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) strategy: Option<String>,
}
