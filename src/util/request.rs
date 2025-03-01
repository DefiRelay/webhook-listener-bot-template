use reqwest::header::HeaderMap;
use reqwest::Error;
use reqwest::IntoUrl;
use serde_json::json;

pub async fn post_request<U: IntoUrl>(url: U, data: serde_json::Value) -> Result<(), Error> {
    // Create a client instance
    let client = reqwest::Client::new();

    // Define the URL
    //   let url = "http://httpbin.org/post";

    // JSON data to send
    /*   let data = json!({
        "name": "John Doe",
        "age": 30
    });*/

    /*let new_post: Post = reqwest::Client::new()
    .post("https://jsonplaceholder.typicode.com/posts")
    .json(&new_post)
    .send()
    .await?
    .json()
    .await?;*/

    // Send a POST request
    let res = client.post(url).json(&data).send().await?;

    // Optionally, handle the response e.g., check status, parse body, etc.
    if res.status().is_success() {
        println!("Successfully sent the POST request");
    } else {
        println!("Failed to send POST request: {}", res.status());
    }

    Ok(())
}

pub async fn patch_request<U: IntoUrl>(
    url: U,
    data: serde_json::Value,
    header_map: HeaderMap,
) -> Result<(), Error> {
    // Create a client instance
    //let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
    .use_rustls_tls() // Force Rust TLS backend
    .build()
    .expect("Failed to create reqwest client");

    // Send a POST request
    let res = client
        .patch(url)
        .headers(header_map)
        .json(&data)
        .send()
        .await?;

    // Optionally, handle the response e.g., check status, parse body, etc.
    if res.status().is_success() {
        println!("Successfully sent the POST request");
    } else {
        println!("Failed to send POST request: {:?}", res);
    }

    Ok(())
}
