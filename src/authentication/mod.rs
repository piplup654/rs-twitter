use base64;
use dotenv::dotenv;
pub mod helper_structs;
const AUTHORIZE_ENDPOINT: &str = "https://api.twitter.com/oauth2/token";

pub struct Client {
    pub api_key: String,
    pub api_secret: String,
    pub bearer_token: String,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(api_key: {}, api_secret: {}, bearer_token: {})", self.api_key, self.api_secret, self.bearer_token)
    }
}

pub fn get_api_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = match std::env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => panic!("No .env file found or no API_KEY field specified"),
    };
    let api_secret = match std::env::var("API_SECRET") {
        Ok(api_secret) => api_secret,
        Err(_) => panic!("No .env file found or no API_SECRET field specified"),
    };
    Ok((api_key, api_secret))
}

impl Client {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {api_key, api_secret, bearer_token: String::from("")}
    }
    pub async fn authenticate(&mut self){
        let client_string = base64::encode(
            format!("{}:{}",self.api_key, self.api_secret));
        let auth_body = helper_structs::AuthBody {
            grant_type: "client_credentials".to_string()
        };
        let auth_body_encoded = serde_urlencoded::to_string(&auth_body).unwrap();
        let auth_request = match reqwest::Client::new().post(AUTHORIZE_ENDPOINT)
            .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
            .header("Authorization", format!("Basic {}", client_string))
            .body(auth_body_encoded)
            .send()
            .await
            .unwrap().json::<helper_structs::AuthResponse>().await {
                Ok(resp) => resp,
                Err(_) => panic!("Error while trying to request bearer token")
            };
        let bearer: String = auth_request.access_token;
        self.bearer_token = bearer;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};
    #[test]
    fn check_client_creation() {
        let api_key = String::from(rand::thread_rng().gen_range(2312323..3232313).to_string());
        let api_secret = String::from(rand::thread_rng().gen_range(2312323..3232313).to_string());
        let client: Client = Client::new(api_key.clone(), api_secret.clone());
        assert_eq!(client.api_key, api_key);
    }
}
