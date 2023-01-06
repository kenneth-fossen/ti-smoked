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

        assert_eq!(!schema.name.is_empty(), true, "It has a name");
        assert_eq!(schema.description.is_none(), false, "There shall be a description");
        assert_eq!(schema.interfaces.len() >0, true);
        assert_eq!(schema.classes.len() > 0, true);

        test_result.success()
    }
}
