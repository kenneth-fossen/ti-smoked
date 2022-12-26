#![feature(async_closure)]
//#![feature(async_iterator)]

//#![feature(async_fn_in_trait)]
//#![allow(incomplete_features)]
// unable to use Async fn in trait due to the use of Box<dyn Trait>
#![allow(dead_code, unused_variables, unreachable_code)]

use ti_smoked::core::{SmokeTest, TestTarget};
use ti_smoked::open;

use ti_smoked::smoke::{alive::AliveTest, dummy::DummyTest};

#[tokio::main]
async fn main() {
    println!("Hello, world!\n");

    let file_content = open("dev.json").expect("Failed to open the file");
    let test_target: TestTarget =
        serde_json::from_str(file_content.as_str()).expect("Failed to parse JSON");
    println!();

    let http_client = reqwest::Client::new();

    let mut commands: Vec<Box<dyn SmokeTest>> = vec![];
    commands.push(Box::new(AliveTest {
        name: "Alive Test".to_string(),
        config: test_target.clone(),
        webclient: http_client,
    }));
    commands.push(Box::new(DummyTest {
        name:"Dummy Test".to_string(),
    }));

    println!("Test Target: {}\n", &test_target.name);

    run(commands, test_target).await;
}

async fn run(mut commands: Vec<Box<dyn SmokeTest>>, _target: TestTarget) {
    commands.reverse();
    println!("| Detector\t | Failure\t | Duration | Details \t |");
    println!("--------------------------------------------------");

    while let Some(cmd) = commands.pop() {
        println!("{}", cmd.run().await)
    }
    //commands
    //    .iter()
    //    .for_each(async |cmd| println!("{}", cmd.run().await));

    println!("--------------------------------------------------\n");
    println!("Total tests:\t{}", commands.len());
    println!("\tPassed:\t{}", commands.len());
}