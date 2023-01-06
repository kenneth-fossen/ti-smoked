use crate::commonlib::CommonLibraryApi;
use crate::smoke::{MappedCodeTest, ResultBuilder, SmokeTest, TestResult, TestResultBuilder};
use async_trait::async_trait;
use std::time::Instant;

#[async_trait]
impl SmokeTest for MappedCodeTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());
        // // "CableCode", "CommonLibrary", "AHA"
        let res = self
            .client
            .get_code_mapped(
                "CableCode".to_string(),
                "CommonLibrary".to_string(),
                "AHA".to_string(),
            )
            .await;

        if !res.objects.is_empty() {
            return test_result.success();
        }

        test_result.failed()
    }
}
