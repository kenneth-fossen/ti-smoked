use serde::Deserialize;
use std::{fmt::Display, time::Duration};
pub mod cl_client;

pub trait SmokeTest {
    fn run(&self) -> TestResult;
}

pub struct TestResult {
    pub details: String,
    pub smoke: bool,
    pub duration: Duration,
    pub name: String,
}

impl Display for TestResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = if !self.smoke { "" } else { "X" };
        write!(
            formatter,
            "| {}\t | {}\t\t | {} ms \t| {} \t\t |",
            self.name,
            result,
            self.duration.as_millis(),
            self.details
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TestTarget {
    pub name: String,
    config: Vec<ConfigItem>,
}

impl TestTarget {
    pub fn get_config_value(&self, key: &str) -> String {
        let found: Vec<&ConfigItem> = self
            .config
            .iter()
            .filter(|config| config.key == key)
            .collect();

        found.first().unwrap().value.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct ConfigItem {
    key: String,
    value: String,
}
