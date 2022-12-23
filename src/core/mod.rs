use serde::Deserialize;
use std::{fmt::Display, time::Duration, time::Instant};

pub trait SmokeTest {
    fn run(&self) -> TestResult;
}

pub struct TestResult {
    pub details: String,
    pub smoke: bool,
    pub duration: Duration,
    pub name: String,
}

pub trait ResultBuilder {
    fn set_details(self, details: String) -> TestResultBuilder;
    fn set_name(self, name: String) -> TestResultBuilder;
    fn set_duration(self, duration: Instant) -> TestResultBuilder;
    fn failed(self) -> TestResult;
    fn success(self) -> TestResult;
    fn build(self, smoke: bool) -> TestResult;
}

#[derive(Default)]
pub struct TestResultBuilder {
    details: Option<String>,
    duration: Option<Instant>,
    name: Option<String>,
}

impl ResultBuilder for TestResultBuilder {
    //type OutputType = TestResult;

    fn set_details(mut self, details: String) -> TestResultBuilder {
        self.details = Some(details);
        self
    }

    fn set_name(mut self, name: String) -> TestResultBuilder {
        self.name = Some(name);
        self
    }

    fn set_duration(mut self, duration: Instant) -> TestResultBuilder {
        self.duration = Some(duration);
        self
    }

    fn failed(self) -> TestResult {
        TestResult {
            details: self.details.unwrap_or(format!("")),
            smoke: true,
            duration: self.duration.expect("Please set duration").elapsed(),
            name: self.name.unwrap(),
        }
    }

    fn success(self) -> TestResult {
        TestResult {
            details: self.details.unwrap_or(format!("")),
            smoke: false,
            duration: self.duration.expect("Please set duration").elapsed(),
            name: self.name.unwrap(),
        }
    }

    fn build(self, smoke: bool) -> TestResult {
        TestResult {
            details: self.details.unwrap_or(format!("")),
            smoke,
            duration: self.duration.expect("Please set duration").elapsed(),
            name: self.name.unwrap(),
        }
    }
}

impl Display for TestResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = if !self.smoke { "" } else { "X" };
        let details = if self.details.is_empty() {
            format!("\t")
        } else {
            self.details.clone()
        };
        write!(
            formatter,
            "| {}\t | {}\t\t | {} ms \t| {} \t\t |",
            self.name,
            result,
            self.duration.as_millis(),
            details
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
