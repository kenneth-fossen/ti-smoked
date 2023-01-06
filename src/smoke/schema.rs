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
        // // "CableCode", "CommonLibrary", "AHA"
        let schema_name = self.config.get_config_value("GetSchemaDetector:SchemaName");
        let schema_options = SchemaOptions { schema_name };
        let res = self.client.get_schema(schema_options).await;

        test_result.failed()
    }
}
