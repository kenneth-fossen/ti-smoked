use crate::core::{ResultBuilder, SmokeTest, TestResultBuilder};
use crate::core::TestResult;
use crate::core::TestTarget;
use std::time::Instant;

pub struct AliveTest {
    pub name: String,
    pub config: TestTarget,
    pub webclient: reqwest::blocking::Client,
}

impl SmokeTest for AliveTest {
    fn run(&self) -> TestResult {
        let test_result = TestResultBuilder::default()
                .set_name(self.name.clone())
                .set_duration(Instant::now());

        let baseurl = self.config.get_config_value("CommonLibraryApiBaseAddress");
        if baseurl.is_empty() {
            return test_result.failed();
        }
        
        let res = self
            .webclient
            .get(format!("{}/alive", baseurl))
            .send()
            .expect("Failed to get alive signal");

        let test_result = if res.status().is_success() {
            test_result.success()
        } else {
            test_result
                .set_details(format!("HTTP Request was unsuccessful: {}", res.status().to_string()))
                .failed()
        };
        test_result
    }
}
