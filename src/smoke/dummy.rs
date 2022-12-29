use crate::commonlib::TestResult;
use crate::commonlib::{ResultBuilder, SmokeTest, TestResultBuilder};
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
            .set_details("Good".to_string())
            .set_duration(Instant::now());

        test_result.success()
    }
}
