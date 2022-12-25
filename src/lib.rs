use std::fs::File;
use std::io::Read;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::core::TestTarget;

pub mod core;
pub mod smoke;

pub fn open(filename: &str) -> Result<String, std::io::Error> {
    let basepath = concat!(env!("CARGO_MANIFEST_DIR"), "/local");
    println!("BasePath: {basepath}");
    let filepath = format!("{basepath}/{filename}");
    println!("Opening file: {filepath}");
    let mut file = File::open(filepath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content)
}


pub struct ClClient {
    appkey: String,
    baseurl: String,
    tokenprovider: String,
    //webclient: reqwest::Client,
}

impl ClClient {
    fn build(&self, test_target: TestTarget) -> ClClient {
        let appkey = test_target.get_config_value("CommonLibraryAppId");
        let baseurl = test_target.get_config_value("CommonLibraryApiBaseAddress");
        let tokenprovider = test_target.get_config_value("TokenProviderConnectionString");

        ClClient {
            appkey,
            baseurl,
            tokenprovider,
        }
    }

    fn get_library(group: String) -> Vec<Library>{
        todo!()
    }
}

struct Library {
    definition: String,
    alias: String,
    is_global: bool,
    is_scope_specific: bool,
    is_case_sensitive: bool,
    are_names_uppercase: bool,
    name_in_identity: bool,
    name_may_change: bool,
    is_foreign_object: bool,
    // display name Regex
    code_name_regex: String,
    code_sets: Vec<CodeSet>,
    tags: Vec<String>,
    access_groups: Vec<String>,
    attribute_definition: Vec<AttributeDefinition>,
    // Ignore null value
    attachment: Attachment,
    attachment_key: Uuid,
    // displayname Created / Ignore null
    date_created: DateTime<Utc>,
    // displayname Updated / Ignore null
    date_updated: DateTime<Utc>,
}

struct Attachment;
struct AttributeDefinition;

struct CodeSet {
    codes: Vec<Code>,
    library: Library,
    code_name_regex: String,
    scopes: Vec<String>,
    // default value TRUE
    locked_for_delete: bool,
    // displayname Created / Ignore null
    date_created: DateTime<Utc>,
    // displayname Updated / Ignore null
    date_updated: DateTime<Utc>,
}

struct Code;