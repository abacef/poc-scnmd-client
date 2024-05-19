use serde::{Serialize};

#[derive(Serialize)]
struct Tang {
    cpu_used: f64,
}

#[tokio::main]
async fn main() {
    let tang = Tang{cpu_used: 3.4};
    let json_string = serde_json::to_string(&tang).unwrap();
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8000/metric_data/cpu_usage")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_string.to_owned())
        .send()
        .await;
    println!("Response status: {:?}", response);
}
