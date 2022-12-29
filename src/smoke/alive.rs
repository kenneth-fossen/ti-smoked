use crate::commonlib::TestResult;
use crate::commonlib::TestTarget;
use crate::commonlib::{ResultBuilder, SmokeTest, TestResultBuilder};
use std::time::Instant;
use async_trait::async_trait;

pub struct AliveTest {
    pub name: String,
    pub config: TestTarget,
    pub webclient: reqwest::Client,
}

#[async_trait]
impl SmokeTest for AliveTest {
    async fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
            .set_name(self.name.clone())
            .set_duration(Instant::now());

        let baseurl = self.config.get_config_value("CommonLibraryApiBaseAddress");
        if baseurl.is_empty() {
            return test_result.failed();
        }

        let res = self
            .webclient
            .get(format!("{baseurl}/alive"))
            .send()
            .await
            .expect("Failed to get alive signal");

        let test_result = if res.status().is_success() {
            test_result.success()
        } else {
            test_result
                .set_details(format!(
                    "HTTP Request was unsuccessful: {}",
                    res.status()
                ))
                .failed()
        };
        test_result
    }
}
