use serde::Deserialize;
use serde::Serialize;
use reqwest::Error;
// use reqwest::Body;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TokenizeRequest {
    pub text: String,
    pub words_only: bool,
}

impl TokenizeRequest {
    pub fn new(text_: String) -> TokenizeRequest {
        TokenizeRequest {
            text: text_,
            words_only: false
        }
    }
}

// impl From<TokenizeRequest> for Body {
//     fn from(r: TokenizeRequest) -> Body {
//         let json_str = serde_json::to_string(&r).unwrap();
//         Body::from(json_str)
//     }
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TokenizeResponse {
    pub notes: Option<String>,
    pub tokens: Vec<Vec<String>>,
}


#[tokio::main]
async 
fn main() -> Result<(), Error> {
    let request_url = format!("http://localhost:8080/tokenize");
    println!("{}", request_url);

    let txt = "а де йому";
    let req = TokenizeRequest::new(txt.into());

    let json_str = serde_json::to_string(&req).unwrap();
    println!("{:?}", json_str);

    let client = reqwest::Client::new();
    let response = client
        .post(&request_url)
        .header("Content-Type", "application/json")
        .body(json_str) // req
        // .body(req) // req
        .send()
        .await?;

    let txt_resp = response.text().await.unwrap();

    println!("{:?}", txt_resp);

    let resp: TokenizeResponse = serde_json::from_str(&txt_resp).unwrap();
    println!("{:?}", resp);

    Ok(())
}
