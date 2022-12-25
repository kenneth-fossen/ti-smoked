use std::time::Instant;
use crate::core::{ResultBuilder, SmokeTest, TestResult, TestResultBuilder, TestTarget};
use crate::ClClient;
use async_trait::async_trait;

struct LibrariesTest {
    pub name: String,
    pub config: TestTarget,
    pub webclient: ClClient,
}

#[async_trait]
impl SmokeTest for LibrariesTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());

        test_result.failed()
    }
}