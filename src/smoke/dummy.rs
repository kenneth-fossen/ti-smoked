use crate::core::SmokeTest;
use crate::core::TestResult;
use std::time::Duration;

pub struct DummyTest { 
    pub name: String 
}

impl SmokeTest for DummyTest {
    fn run(&self) -> TestResult {
        let time = Duration::from_millis(1337);

        TestResult {
            name: self.name.clone(),
            smoke: false,
            details: String::from("Good"),
            duration: time,
        }
    }
}