pub mod entities;

use std::sync::Arc;
use async_trait::async_trait;
use azure_identity::ClientSecretCredential;
use azure_core::auth::TokenCredential;
use serde::de::DeserializeOwned;
use entities::{Code, Library, Message, Schema, SchemaOptions, ViewDefinition};
use crate::smoke::TestTarget;



pub struct ClientFactory {
    appkey: String,
    baseurl: String,
    tokenprovider: TokenProvider,
}
pub struct Client {
    appkey: String,
    baseurl: String,
    webclient: reqwest::Client,
    azure_client: ClientSecretCredential,
}

#[async_trait]
trait CommonLibraryApi {
    async fn get_library(&self, group: String) -> Vec<Library>;
    async fn get_code(&self,group: String) -> Vec<Code>;
    fn get_schema(&self,schema_options: SchemaOptions) -> Schema;
    async fn get_code_mapped(&self,library: String, schema: String, facility: String) -> Message;
    fn get_genericview_definition(&self,library: String) -> ViewDefinition;
}

#[async_trait]
pub trait Configure {
    fn configure(test_target: TestTarget) -> ClientFactory;
    fn build(&self) -> Client;
}

#[async_trait]
pub trait CommonLibClient {
    async fn do_request(&self, url: String) -> String;
    async fn get_request<T: DeserializeOwned>(&self, url: String) -> T;
}

#[async_trait]
impl Configure for ClientFactory {
    fn configure(test_target: TestTarget) -> ClientFactory {
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
            baseurl: self.baseurl.clone(),
            webclient: webclient.clone(),
            azure_client: azure_cli,
        }
    }
}

#[async_trait]
impl CommonLibClient for Client {
    async fn do_request(&self, url: String) -> String {
        if let Ok(tokenresponse) = self.azure_client.get_token(self.appkey.as_str()).await {
            self.webclient
                .get(url)
                .header("Authorization", format!("Bearer {}", tokenresponse.token.secret()))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        } else {
            panic!("Unable to auth against Azure");
        }
    }

    async fn get_request<T: DeserializeOwned>(&self, url: String) -> T {
        let resp = self.do_request(url).await;
        //println!("Response: {:?}",resp);
        let item: T = serde_json::from_str(resp.as_str()).unwrap();
        item
    }
}



#[derive(Debug)]
struct TokenProvider {
    pub runas: String,
    pub tenant: String,
    pub appid: String,
    pub secret: String,
}

impl TokenProvider {
    fn from_connectionstring(connectionstring: String) -> TokenProvider {
        let mut pairs: Vec<&str> = connectionstring.split(';').collect();
        let secret: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        let tenantid: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        let appid: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        let runas: Vec<&str> = pairs.pop().unwrap().split('=').collect();
        TokenProvider {
            tenant: tenantid.last().unwrap().to_string(),
            runas: runas.last().unwrap().to_string(),
            appid: appid.last().unwrap().to_string(),
            secret: secret.last().unwrap().to_string(),
        }
    }
}

#[async_trait]
impl CommonLibraryApi for Client {


    async fn get_library(&self,group: String) -> Vec<Library>{
        // /api/Library?name={name}&group={group}&scope={scope}&name={name}&isValid={isValid}"
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/Library?group={group}");
        self.get_request::<Vec<Library>>(url).await
    }
    async fn get_code(&self,library: String) -> Vec<Code> {
        // "/api/Code/{library}?scope={scope}&name={name}&description={description}&isValid={isValid}&$filter={filter}");
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/Code/{library}");
        self.get_request::<Vec<Code>>(url).await
    }

    fn get_schema(&self,schema_options: SchemaOptions) -> Schema {
        todo!()
    }

    async fn get_code_mapped(&self,library: String, schema: String, facility: String) -> Message {
        // $"/api/Code/Mapped/{library}?schema={schema}&scope={scope}");
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/code/Mapped/{library}?schema={schema}&scope={facility}");
        self.get_request::<Message>(url).await
    }


    fn get_genericview_definition(&self,library: String) -> ViewDefinition {
        todo!()
    }
}



#[cfg(test)]
mod test {
    use crate::open;
    use super::*;

    fn get_config() -> TestTarget {
        let file_content = open("dev.json").expect("Failed to open the file");
        let test_target: TestTarget =
            serde_json::from_str(file_content.as_str()).expect("Failed to parse JSON");
        test_target
    }

    #[tokio::test]
    async fn authenticate() {
        let test_target = get_config();
        let client = ClientFactory::configure(test_target).build();
        let token = client.azure_client.get_token(client.appkey.as_str()).await;
        let token: String = token.unwrap().token.secret().chars().take(2).collect();
        assert_eq!(token, "ey")
    }

    #[tokio::test]
    async fn get_library() {
        let test_target = get_config();
        let client = ClientFactory::configure(test_target).build();
        let resp = client.get_library("Facility and Project".to_string()).await;
    }

    #[tokio::test]
    async fn get_code() {
        let test_target = get_config();

        let client = ClientFactory::configure(test_target).build();
        let resp = client.get_code("Facility".to_string()).await;
    }

    #[tokio::test]
    async fn get_mapped_code() {
        let test_target = get_config();
        let client = ClientFactory::configure(test_target).build();
        let resp = client.get_code_mapped("CableCode".to_string(), "CommonLibrary".to_string(), "AHA".to_string() ).await;
    }
}