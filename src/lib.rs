extern crate core;

use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use async_trait::async_trait;
use azure_identity::ClientSecretCredential;
use azure_core::auth::TokenCredential;
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use uuid::Uuid;
use crate::commonlib::TestTarget;

pub mod commonlib;
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

pub struct ClientFactory {
    appkey: String,
    baseurl: String,
    tokenprovider: TokenProvider,
}
pub struct Client {
    appkey: String,
    webclient: reqwest::Client,
    azure_client: ClientSecretCredential,
}

trait CommonLibraryApi {
    fn get_library(group: String) -> Vec<Library>;
    fn get_code(group: String) -> Vec<Code>;
    fn get_schema(schema_options: SchemaOptions) -> Schema;
    fn get_code_mapped(library: String, schema: String, facility: String) -> Message;
    fn get_genericview_definition(library: String) -> ViewDefinition;
}
#[async_trait]
trait Configure {
    fn configure(&self, test_target: TestTarget) -> ClientFactory;
    fn build(&self) -> Client;

}

#[async_trait]
trait CommonLibClient {
    async fn get_request<'de, T: Clone + Deserialize<'de>>(&self, url: String) -> T;
}



#[async_trait]
impl Configure for ClientFactory {
    fn configure(&self, test_target: TestTarget) -> ClientFactory {
        let appkey = test_target.get_config_value("CommonLibraryAppId");
        let baseurl = test_target.get_config_value("CommonLibraryApiBaseAddress");
        let tokenprovider = TokenProvider::from_connectionstring(test_target.get_config_value("TokenProviderConnectionString"));

        ClientFactory {
            appkey,
            baseurl,
            tokenprovider,
        }
    }

    fn build(&self) -> Client {
        let webclient = reqwest::Client::new();
        let azure_cli = azure_identity::ClientSecretCredential::new(
            Arc::new(webclient.clone()),
            self.tokenprovider.tenant.clone(),
            self.tokenprovider.appid.clone(),
            self.tokenprovider.secret.clone(),
            Default::default()
        );

        Client {
            appkey: self.appkey.clone(),
            webclient: webclient.clone(),
            azure_client: azure_cli,
        }
    }
}

#[async_trait]
impl CommonLibClient for Client {
    async fn get_request<'de, T: Clone + Deserialize<'de>>(&self, url: String) -> T {
        // unsure what flow to use with AppId.

        if let Ok(tokenresponse) = self.azure_client.get_token(self.appkey.as_str()).await {
            let resp= self.webclient
                .get(url)
                .header("Authorization", format!("Bearer {}", tokenresponse.token.secret()))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let item: T =  serde_json::from_str(*resp).unwrap();
            item
        } else {
            panic!("Unable to auth against Azure");
        }
    }
}


struct TokenProvider {
    pub runas: String,
    pub tenant: String,
    pub appid: String,
    pub secret: String,
}

impl TokenProvider {
    fn from_connectionstring(connectionstring: String) -> TokenProvider {
        let mut pairs: Vec<&str> = connectionstring.split(';').collect();
        let runas: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        let appid: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        let tenantid: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        let secret: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        TokenProvider {
            tenant: tenantid.last().unwrap().to_string(),
            runas: runas.last().unwrap().to_string(),
            appid: appid.last().unwrap().to_string(),
            secret: secret.last().unwrap().to_string(),
        }
    }
}

impl CommonLibraryApi for Client {

    // /api/Library?name={name}&group={group}&scope={scope}&name={name}&isValid={isValid}"
    fn get_library(group: String) -> Vec<Library>{
        todo!()
    }
    fn get_code(library: String) -> Vec<Code> {
        todo!()
    }

    fn get_schema(schema_options: SchemaOptions) -> Schema {
        todo!()
    }

    fn get_code_mapped(library: String, schema: String, facility: String) -> Message {
        todo!()
    }

    fn get_genericview_definition(library: String) -> ViewDefinition {
        todo!()
    }
}

#[derive(Deserialize)]
struct Schema;
struct SchemaOptions;

#[derive(Deserialize)]
struct ViewDefinition;

#[derive(Deserialize)]
struct Message;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
struct Attachment {

}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
enum CodeRefDisplayMode {
    Identity = 0,
    Name = 1,
    Description = 2,
    NameAndDescription =3 ,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
struct Code {
    project: String,
    // Display name Code Set, ignore null
    code_set: CodeSet,
    code_set_name: String,
    attributes: Vec<CodeAttribute>,
    identity: String,
    iri: String,
    attachment: Attachment,
    attachment_key: Uuid,
}

#[derive(Deserialize)]
struct CodeAttribute {

}


#[cfg(test)]
mod test {

}