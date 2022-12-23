#![allow(dead_code, unused_variables, unreachable_code)]
use serde_json;
use ti_smoked::core::{SmokeTest, TestTarget};
use ti_smoked::open;

use ti_smoked::smoke::{
    alive::AliveTest,
    dummy::DummyTest
};

fn main() {
    println!("Hello, world!\n");

    let file_content = open("dev.json").expect("Failed to open the file");
    let test_target: TestTarget =
        serde_json::from_str(file_content.as_str()).expect("Failed to parse JSON");
    println!();

    let http_client = reqwest::blocking::Client::new();

    let mut commands: Vec<Box<dyn SmokeTest>> = vec![];
    commands.push(Box::new(AliveTest {
        name: format!("Alive Test"),
        config: test_target.clone(),
        webclient: http_client,
    }));
    commands.push(Box::new(DummyTest { name: format!("Dummy Test") }));

    println!("Test Target: {}\n", &test_target.name);

    run(commands, test_target);
}

fn run(commands: Vec<Box<dyn SmokeTest>>, _target: TestTarget) {
    println!("| Detector\t | Failure\t | Duration | Details \t |");
    println!("---");
    commands.iter().for_each(|cmd| println!("{}", cmd.run()));
    println!("---");

    println!("Total tests:\t{}", commands.len());
    println!("\tPassed:\t{}", commands.len());
}


pub struct ClClient {
    appkey: String,
    baseurl: String,
    tokenprovider: String,
}

impl ClClient {
    fn build(&self, test_target: TestTarget) -> ClClient {
        let appkey = test_target.get_config_value("CommonLibraryAppId");
        let baseurl = test_target.get_config_value("CommonLibraryApiBaseAddress");
        let tokenprovider = test_target.get_config_value("TokenProviderConnectionString");
        ClClient {
            appkey,
            baseurl,
            tokenprovider,
        }
    }
}