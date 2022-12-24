use crate::core::TestResult;
use crate::core::{ResultBuilder, SmokeTest, TestResultBuilder};
use std::time::Instant;
use async_trait::async_trait;

pub struct DummyTest {
    pub name: String,
}

#[async_trait]
impl SmokeTest for DummyTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_details(format!("Good"))
            .set_duration(Instant::now());

        test_result.success()
    }
}
