use crate::commonlib::entities::SchemaOptions;
use crate::commonlib::CommonLibraryApi;
use crate::smoke::{ResultBuilder, SchemaTest, SmokeTest, TestResult, TestResultBuilder};
use async_trait::async_trait;
use std::time::Instant;

#[async_trait]
impl SmokeTest for SchemaTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());

        let schema_name = self.config.get_config_value("GetSchemaDetector:SchemaName");
        let schema_options = SchemaOptions { schema_name };
        let schema = self.client.get_schema(schema_options).await;
        // let schema = schemadto..first().unwrap();

        assert!(!schema.name.is_empty(), "It has a name");
        assert!(schema.description.is_some(), "There shall be a description");
        assert!(!schema.interfaces.is_empty());
        assert!(!schema.classes.is_empty());

        test_result.success()
    }
}
