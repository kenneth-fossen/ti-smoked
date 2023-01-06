use crate::smoke::{DummyTest, ResultBuilder, SmokeTest, TestResult, TestResultBuilder};
use async_trait::async_trait;
use std::time::Instant;

#[async_trait]
impl SmokeTest for DummyTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_details("Good".to_string())
            .set_duration(Instant::now());

        test_result.success()
    }
}
