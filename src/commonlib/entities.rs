use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Schema;
pub(crate) struct SchemaOptions;

#[derive(Deserialize)]
pub(crate) struct ViewDefinition;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Message {
    id: Uuid,
    timestamp: DateTime<Utc>,
    schema: Option<Schema>,
    metadata: Vec<MessageAttribute>,
    objects: Vec<MessagingObject>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageAttribute;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessagingAttribute {
    name: String,
    behavior: Option<String>,
    value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessagingSubObject;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessagingRelationship;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessagingObject {
    name: String,
    attributes: Vec<MessagingAttribute>,
    sub_objects: Vec<MessagingSubObject>,
    relationships: Vec<MessagingRelationship>,
}

#[derive(Deserialize, Debug)]
struct Attachment {

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Library {
    definition: String,
    alias: String,
    is_global: bool,
    is_scope_specific: bool,
    is_case_sensitive: bool,
    are_names_uppercase: bool,
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
    attribute_definition: Vec<AttributeDefinition>,
    // Ignore null value
    attachment: Attachment,
    attachment_key: Uuid,
    // displayname Created / Ignore null
    #[serde(default)]
    date_created: DateTime<Utc>,
    #[serde(default)]
    // displayname Updated / Ignore null
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

#[derive(Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct AttributeDefinition {
    name: String,
    description: String,
    display_as: String,
    // StringEnumConverter
    attribute_type: AttributeType,
    sequence_number: i32,
    required: bool,
    include_identity: bool,
    reference_library_name: String,
    reference_display_mode: CodeRefDisplayMode,
}

#[derive(Deserialize, Debug)]
enum CodeRefDisplayMode {
    Identity = 0,
    Name = 1,
    Description = 2,
    NameAndDescription =3 ,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CodeSet {
    codes: Vec<Code>,
    library: Library,
    code_name_regex: Option<String>,
    scopes: Vec<String>,
    // default value TRUE
    locked_for_delete: bool,
    // displayname Created / Ignore null
    date_created: DateTime<Utc>,
    // displayname Updated / Ignore null
    date_updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Code {
    project: Option<String>,
    // Display name Code Set, ignore null
    code_set: Option<CodeSet>,
    code_set_name: Option<String>,
    attributes: Vec<CodeAttribute>,
    identity: String,
    iri: Option<String>,
    attachment: Option<Attachment>,
    attachment_key: Option<Uuid>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CodeAttribute {}