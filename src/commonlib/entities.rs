#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

fn str_to_time<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
{
    let mut timestr = String::deserialize(d)?;
    if !timestr.ends_with('Z') {
        timestr.push('Z');
    }
    let date = DateTime::parse_from_rfc3339(&timestr).unwrap();
    Ok(date.with_timezone(&Utc))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub name: String,
    pub references: Option<String>,
    pub mapping: Option<String>,
    pub versions: Vec<SchemaVersion>,
    pub description: Option<String>,
    pub interfaces: Vec<SchemaInterface>,
    pub classes: Vec<SchemaClass>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaVersion {
    version_number: String,
    description: Option<String>,
    #[serde(deserialize_with = "str_to_time")]
    release_date: DateTime<Utc>,
    change_log: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaClass {
    name: String,
    description: Option<String>,
    schema: Option<String>,
    comments: Option<String>,
    references: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaInterface {
    name: Option<String>,
    description: Option<String>,
    comments: Option<String>,
    reference: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaOptions {
    pub(crate) schema_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewDefinition {
    library: String,
    pub(crate) columns: Vec<DatabaseInformationSchemaColumn>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct DatabaseInformationSchemaColumn {
    table_catalog: Option<String>,
    table_schema: Option<String>,
    table_name: Option<String>,
    pub column_name: Option<String>,
    ordinal_position: Option<i32>,
    is_nullable: Option<String>,
    data_type: Option<String>,
    character_maximum_length: Option<i32>,
    character_octet_length: Option<i32>,
    numeric_precision: Option<i32>,
    numeric_precision_radix: Option<i32>,
    numeric_scale: Option<i32>,
    datetime_precision: Option<i32>,
    character_set_catalog: Option<String>,
    character_set_schema: Option<String>,
    character_set_name: Option<String>,
    collation_catalog: Option<String>,
    collation_schema: Option<String>,
    collation_name: Option<String>,
    domain_catalog: Option<String>,
    domain_schema: Option<String>,
    domain_name: Option<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    id: Uuid,
    timestamp: DateTime<Utc>,
    schema: Option<Schema>,
    metadata: Vec<MessageAttribute>,
    pub objects: Vec<MessagingObject>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttribute;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingAttribute {
    name: String,
    behavior: Option<String>,
    value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingSubObject;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingRelationship;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingObject {
    name: String,
    attributes: Vec<MessagingAttribute>,
    sub_objects: Vec<MessagingSubObject>,
    relationships: Vec<MessagingRelationship>,
}

#[derive(Deserialize, Debug)]
pub struct Attachment {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub(crate) name: String,
    definition: Option<String>,
    alias: Option<String>,
    is_global: bool,
    is_scope_specific: bool,
    is_case_sensitive: bool,
    are_names_upper_case: bool,
    name_in_identity: bool,
    name_may_change: bool,
    is_foreign_object: bool,
    foreign_app_name: Option<String>,
    // display name Regex
    code_name_regex: Option<String>,
    scope_type: Option<String>,
    code_sets: Vec<CodeSet>,
    tags: Vec<String>,
    access_groups: Vec<String>,
    attribute_definition: Option<Vec<AttributeDefinition>>,
    // Ignore null value
    attachment: Option<Attachment>,
    attachment_key: Option<Uuid>,
    // displayname Created / Ignore null
    #[serde(deserialize_with = "str_to_time")]
    date_created: DateTime<Utc>,
    // displayname Updated / Ignore null
    #[serde(deserialize_with = "str_to_time")]
    date_updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
enum AttributeType {
    String,
    Int,
    Float,
    Bool,
    Date,
    DateTime,
    CodeRef,
    LibraryRef,
    Uri,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDefinition {
    name: String,
    description: String,
    display_as: String,
    // StringEnumConverter
    attribute_type: Option<AttributeType>,
    sequence_number: i32,
    required: bool,
    include_identity: bool,
    reference_library_name: Option<String>,
    reference_display_mode: CodeRefDisplayMode,
}

#[derive(Deserialize, Debug)]
pub enum CodeRefDisplayMode {
    Identity = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodeSet {
    codes: Vec<Code>,
    library: Option<Library>,
    code_name_regex: Option<String>,
    scopes: Vec<String>,
    // default value TRUE
    locked_for_delete: bool,
    // displayname Created / Ignore null
    #[serde(deserialize_with = "str_to_time")]
    date_created: DateTime<Utc>,
    // displayname Updated / Ignore null
    #[serde(deserialize_with = "str_to_time")]
    date_updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub name: String,
    pub is_valid: bool,
    pub description: Option<String>,
    pub project: Option<String>,
    // Display name Code Set, ignore null
    pub code_set: Option<CodeSet>,
    pub code_set_name: Option<String>,
    pub attributes: Vec<CodeAttribute>,
    pub identity: String,
    pub iri: Option<String>,
    pub attachment: Option<Attachment>,
    pub attachment_key: Option<Uuid>,
    #[serde(deserialize_with = "str_to_time")]
    pub date_created: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_time")]
    pub date_updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodeAttribute {
    pub definition_name: String,
    pub display_value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceCode {
    pub id: i32,
    pub identity: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeValue;
