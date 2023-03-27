async fn make_get_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = make_get_request(url).await.unwrap();
    println!("{}", response);
}
