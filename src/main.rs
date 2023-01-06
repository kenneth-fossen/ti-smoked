//#![feature(async_closure)]
//#![feature(async_iterator)]

//#![feature(async_fn_in_trait)]
//#![allow(incomplete_features)]
// unable to use Async fn in trait due to the use of Box<dyn Trait>
#![allow(dead_code, unused_variables, unreachable_code)]

use ti_smoked::commonlib::{ClientFactory, Configure};
use ti_smoked::open;

use ti_smoked::smoke::{AliveTest, CodesTest, DummyTest, LibrariesTest, MappedCodeTest, SchemaTest, SmokeTest, TestTarget, ViewsTest};

#[tokio::main]
async fn main() {
    println!("Hello, world!\n");

    let file_content = open("prod.json").expect("Failed to open the file");
    let test_target: TestTarget =
        serde_json::from_str(file_content.as_str()).expect("Failed to parse JSON");
    println!();

    let http_client = reqwest::Client::builder().build().unwrap();

    let mut commands: Vec<Box<dyn SmokeTest>> = vec![];
    let azure_client = ClientFactory::configure(test_target.clone()).build().await;

    /*
    -----------------------------------------------------------------------------------------------
Get schema                    |           |      3468 |
Smokey dummy detector         |           |      1337 |
Alive check                   |           |       292 |
Get Codes                     |           |       190 |
Get Libraries                 |           |       410 |
Get view definition           |           |        66 |
Mapped Codes                  |           |       129 |
Query                         |           |       161 |
---------------------------------------------------------------
     */
    commands.push(Box::new(SchemaTest {
        name: "Get Schema".to_string(),
        config: test_target.clone(),
        client: azure_client.clone(),
    }));
    commands.push(Box::new(DummyTest {
        name: "Dummy Test".to_string(),
    }));
    commands.push(Box::new(AliveTest {
        name: "Alive Test".to_string(),
        config: test_target.clone(),
        client: http_client.clone(),
    }));
    commands.push(Box::new(CodesTest {
        name: "Codes Test".to_string(),
        config: test_target.clone(),
        client: azure_client.clone(),
    }));
    commands.push(Box::new(LibrariesTest {
        name: "Get Libraries".to_string(),
        config: test_target.clone(),
        client: azure_client.clone(),
    }));
    commands.push(Box::new(ViewsTest {
        name: "Get View Def".to_string(),
        config: test_target.clone(),
        client: azure_client.clone(),
    }));
    commands.push(Box::new(MappedCodeTest {
        name: "MappedCode".to_string(),
        config: test_target.clone(),
        client: azure_client.clone(),
    }));
    // query test
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
}
