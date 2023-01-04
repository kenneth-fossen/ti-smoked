use std::time::Instant;
use async_trait::async_trait;
use crate::smoke::{SmokeTest, MappedCodeTest, TestResult, TestResultBuilder, ResultBuilder};
use crate::commonlib::CommonLibraryApi;

#[async_trait]
impl SmokeTest for MappedCodeTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());
        // // "CableCode", "CommonLibrary", "AHA"
        let res = self.client.get_code_mapped("CableCode".to_string(), "CommonLibrary".to_string(), "AHA".to_string()).await;

        if res.objects.len() > 0 {
            return test_result.success();
        }

        test_result.failed()
    }
}