use crate::core::SmokeTest;
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
        let timer = Instant::now();
        let baseurl = self.config.get_config_value("CommonLibraryApiBaseAddress");
        if baseurl.is_empty() {
            return TestResult {
                name: self.name.clone(),
                smoke: true,
                details: String::from("Missing fields in config file"),
                duration: timer.elapsed(),
            };
        }
        let res = self
            .webclient
            .get(format!("{}/alive", baseurl))
            .send()
            .expect("Failed to get alive signal");

        TestResult {
            name: self.name.clone(),
            smoke: !res.status().is_success(),
            details: String::from(""),
            duration: timer.elapsed(),
        }
    }
}
