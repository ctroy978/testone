use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    /*  let data = "CREATE account SET
    name = 'ACME Inc',
    created_at = time::now();"
        .to_string();


    let data = "CREATE author:john SET
    name.first = 'John',
    name.last = 'Adams',
    name.full = string::join(' ', name.first, name.last),
    age = 29,
    admin = true,
    signup_at = time::now();";
    */

    let data = "SELECT * FROM author";

    let res = reqwest::Client::new()
        .post("http://localhost:8000/sql")
        .header("Content-Type", "application/json")
        .header("ns", "test")
        .header("db", "test")
        .basic_auth("root", Some("root"))
        .body(data.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await;

    println!("{}", res.unwrap());
}
