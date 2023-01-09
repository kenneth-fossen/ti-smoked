pub mod entities;

use crate::commonlib::entities::Schema;
use crate::smoke::TestTarget;
use async_trait::async_trait;
use azure_core::auth::TokenCredential;
use entities::{Code, Library, Message, SchemaOptions, ViewDefinition};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

pub struct ClientFactory {
    appkey: String,
    baseurl: String,
    tokenprovider: TokenProvider,
}

#[derive(Clone)]
pub struct Client {
    #[allow(dead_code)]
    appkey: String,
    baseurl: String,
    webclient: reqwest::Client,
    token: String,
}

#[async_trait]
pub trait CommonLibraryApi {
    async fn get_library(&self, group: String) -> Vec<Library>;
    async fn get_code(&self, group: String) -> Vec<Code>;
    async fn get_schema(&self, _schema_options: SchemaOptions) -> Schema;
    async fn get_code_mapped(&self, library: String, schema: String, facility: String) -> Message;
    async fn get_genericview_definition(&self, _library: String) -> ViewDefinition;
}

#[async_trait]
pub trait Configure {
    fn configure(test_target: TestTarget) -> ClientFactory;
    async fn build(&self) -> Client;
}

#[async_trait]
pub trait CommonLibClient {
    async fn do_request(&self, url: String) -> String;
    async fn do_post_request(&self, url: String, body: String) -> String;
    async fn get_request<T: DeserializeOwned>(&self, url: String) -> T;
    async fn post_request<T, U>(&self, url: String, body: U) -> T
    where
        T: DeserializeOwned + Send,
        U: Serialize + Send;
}

#[async_trait]
impl Configure for ClientFactory {
    fn configure(test_target: TestTarget) -> ClientFactory {
        let appkey = test_target.get_config_value("CommonLibraryAppId");
        let baseurl = test_target.get_config_value("CommonLibraryApiBaseAddress");
        let tokenprovider = TokenProvider::from_connectionstring(
            test_target.get_config_value("TokenProviderConnectionString"),
        );

        ClientFactory {
            appkey,
            baseurl,
            tokenprovider,
        }
    }

    async fn build(&self) -> Client {
        let webclient = reqwest::Client::builder().build().unwrap();
        let azure_cli = azure_identity::ClientSecretCredential::new(
            Arc::new(webclient.clone()),
            self.tokenprovider.tenant.clone(),
            self.tokenprovider.appid.clone(),
            self.tokenprovider.secret.clone(),
            Default::default(),
        );
        let token = if let Ok(token_response) = azure_cli.get_token(self.appkey.as_str()).await {
            token_response.token.secret().to_string()
        } else {
            "".to_string()
        };

        Client {
            appkey: self.appkey.clone(),
            baseurl: self.baseurl.clone(),
            webclient: webclient.clone(),
            token,
        }
    }
}

#[async_trait]
impl CommonLibClient for Client {
    async fn do_request(&self, url: String) -> String {
        self.webclient
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    async fn do_post_request(&self, url: String, body: String) -> String {
        self.webclient
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    async fn get_request<T: DeserializeOwned>(&self, url: String) -> T {
        let resp = self.do_request(url).await;
        // println!("Response: {:?}",resp);
        let item: T = serde_json::from_str(resp.as_str()).unwrap();
        item
    }

    async fn post_request<T, U>(&self, url: String, body: U) -> T
    where
        T: DeserializeOwned + Send,
        U: Serialize + Send,
    {
        let json = if let Ok(json) = serde_json::to_string(&body) {
            json
        } else {
            "".to_string()
        };
        let resp = self.do_post_request(url, json.clone()).await;
        let item: T = serde_json::from_str(resp.as_str()).unwrap();
        item
    }
}

#[derive(Debug)]
struct TokenProvider {
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
        TokenProvider {
            tenant: tenantid.last().unwrap().to_string(),
            appid: appid.last().unwrap().to_string(),
            secret: secret.last().unwrap().to_string(),
        }
    }
}

#[async_trait]
impl CommonLibraryApi for Client {
    async fn get_library(&self, group: String) -> Vec<Library> {
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/Library?group={group}");
        self.get_request::<Vec<Library>>(url).await
    }
    async fn get_code(&self, library: String) -> Vec<Code> {
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/Code/{library}");
        self.get_request::<Vec<Code>>(url).await
    }

    async fn get_schema(&self, schema_options: SchemaOptions) -> Schema {
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/Schema");
        self.post_request::<Schema, SchemaOptions>(url, schema_options)
            .await
    }

    async fn get_code_mapped(&self, library: String, schema: String, facility: String) -> Message {
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/code/Mapped/{library}?schema={schema}&scope={facility}");
        self.get_request::<Message>(url).await
    }

    async fn get_genericview_definition(&self, library: String) -> ViewDefinition {
        let baseurl = &self.baseurl;
        let url = format!("{baseurl}/api/GenericViews/library/{library}/definition");
        self.get_request::<ViewDefinition>(url).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::open;

    fn get_config(config: Option<String>) -> TestTarget {
        let filename = if config.is_some() {
            config.unwrap()
        } else {
            "dev.json".to_string()
        };
        let file_content = open(&filename).expect("Failed to open the file");
        let test_target: TestTarget =
            serde_json::from_str(file_content.as_str()).expect("Failed to parse JSON");
        test_target
    }

    #[tokio::test]
    async fn authenticate() {
        let test_target = get_config(None);
        let client = ClientFactory::configure(test_target).build().await;
        let token: String = client.token.chars().take(2).collect();
        assert_eq!(token, "ey")
    }

    #[tokio::test]
    async fn get_library() {
        let test_target = get_config(None);
        let client = ClientFactory::configure(test_target).build().await;
        let resp = client.get_library("Facility and Project".to_string()).await;
        assert_eq!(resp.len() > 0, true)
    }

    #[tokio::test]
    async fn get_code() {
        let test_target = get_config(None);

        let client = ClientFactory::configure(test_target).build().await;
        let resp = client.get_code("Facility".to_string()).await;
        assert_eq!(resp.len() > 0, true)
    }

    #[tokio::test]
    async fn get_mapped_code() {
        let test_target = get_config(None);
        let client = ClientFactory::configure(test_target).build().await;
        let resp = client
            .get_code_mapped(
                "CableCode".to_string(),
                "CommonLibrary".to_string(),
                "AHA".to_string(),
            )
            .await;
        assert_eq!(resp.objects.len() > 0, true)
    }

    #[tokio::test]
    async fn get_schema() {
        let test_targets = get_config(Some("local.json".to_string()));
        let client = ClientFactory::configure(test_targets).build().await;

        let schema_options = SchemaOptions {
            schema_name: "TR3111".to_string(),
        };
        let _resp = client.get_schema(schema_options).await;
    }

    #[tokio::test]
    async fn get_viewdefinition() {
        let test_target = get_config(None);
        let client = ClientFactory::configure(test_target).build().await;
        let library_name = "Facility".to_string();
        let _resp = client.get_genericview_definition(library_name).await;
    }
}
