#![feature(async_closure)]
//#![feature(async_iterator)]

//#![feature(async_fn_in_trait)]
//#![allow(incomplete_features)]
// unable to use Async fn in trait due to the use of Box<dyn Trait>
#![allow(dead_code, unused_variables, unreachable_code)]

use ti_smoked::commonlib::{Configure, ClientFactory};
use ti_smoked::commonlib::entities::Code;
use ti_smoked::open;

use ti_smoked::smoke::{SmokeTest, AliveTest, DummyTest, MappedCodeTest, TestTarget, CodesTest};

#[tokio::main]
async fn main() {
    println!("Hello, world!\n");

    let file_content = open("dev.json").expect("Failed to open the file");
    let test_target: TestTarget =
        serde_json::from_str(file_content.as_str()).expect("Failed to parse JSON");
    println!();

    let http_client = reqwest::Client::new();
    let client = ClientFactory::configure(test_target.clone()).build();

    let mut commands: Vec<Box<dyn SmokeTest>> = vec![];
    commands.push(Box::new(AliveTest {
        name: "Alive Test".to_string(),
        config: test_target.clone(),
        client: http_client.clone(),
    }));
    commands.push(Box::new(DummyTest {
        name:"Dummy Test".to_string(),
    }));
    // commands.push(Box::new(CodesTest {
    //     name: "Codes Test".to_string(),
    //     config: test_target.clone(),
    //     client,
    // }));
    commands.push(Box::new(MappedCodeTest {
        name: "MappedCode".to_string(),
        config: test_target.clone(),
        client,
    }));

    println!("Test Target: {}\n", &test_target.name);

    run(commands, test_target).await;
}

async fn run(mut commands: Vec<Box<dyn SmokeTest>>, _target: TestTarget) {
    commands.reverse();
    println!("| Detector\t | Failure\t | Duration | Details \t |");
    println!("----------------------------------------------------------");

    while let Some(cmd) = commands.pop() {
        println!("{}", cmd.run().await)
    }

    println!("----------------------------------------------------------\n");
    println!("Total tests:\t{}", commands.len());
    println!("\tPassed:\t{}", commands.len());
}