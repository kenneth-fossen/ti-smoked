use std::time::Instant;
use crate::commonlib::{ResultBuilder, SmokeTest, TestResult, TestResultBuilder, TestTarget};
use crate::Client;
use async_trait::async_trait;

struct LibrariesTest {
    pub name: String,
    pub config: TestTarget,
    pub webclient: Client,
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