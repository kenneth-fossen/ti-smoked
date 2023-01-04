use std::time::Instant;
use async_trait::async_trait;
use crate::smoke::{LibrariesTest, ResultBuilder, TestResult, TestResultBuilder};


#[async_trait]
impl SmokeTest for LibrariesTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());

        test_result.failed()
    }
}