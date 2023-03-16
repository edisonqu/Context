use serde_json::Value;
use reqwest::Client;
use reqwest::header;


pub async fn question_and_context(
    question: &str, context: &str, api_key: &str
) -> Result<(Value), Box<dyn std::error::Error>>{
    let client = Client::new();

    const URL: &str = "https://api-inference.huggingface.co/models/distilbert-base-cased-distilled-squad";
    let payload = serde_json::json!({
        "inputs": {
            "question": question,
            "context": context
        }
    });

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization",{"Bearer ".to_owned()+api_key}.parse().unwrap());

    let resp = client
        .post(URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;
    println!("{:?}",resp);
    Ok(resp)
}