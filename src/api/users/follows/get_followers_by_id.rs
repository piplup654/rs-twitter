use crate::authentication::Client;
use serde_json::value::Value;
use super::params_structs::{PGetFollowersById, PGetFollowersByIdBuilder};

impl Client {
    pub async fn get_followers_by_id(&self, params: &PGetFollowersById) -> Result<Value, Box<dyn std::error::Error>> {
        let bearer_header = format!("Bearer {}", &self.bearer_token[..]);
        let reqwest_client = reqwest::Client::new();
        let url = format!("https://api.twitter.com/2/users/{}/followers", params.id);
        let user_request = reqwest_client.get(url)
            .header(reqwest::header::AUTHORIZATION, bearer_header)
            .send()
            .await
            .expect("Error while trying to response from get_followers_by_id endpoint");
        let resp_textified = user_request.text().await.expect("Error while trying to textify response");
        let resp_jsonified: Value = serde_json::from_str(&resp_textified).expect("Error while trying to jsonify response(already textified)");
        Ok(resp_jsonified)
    }
}

#[cfg(test)]
mod tests {
    use crate::authentication::get_api_credentials;
    use super::*;
    #[tokio::test]
    async fn check_followers_by_id() -> Result<(), Box<dyn std::error::Error>> {
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client::new(api_key, api_secret);
        let id = String::from("1504478166248611840");
        client.authenticate().await;
        let params = PGetFollowersByIdBuilder::default().id(id.clone()).build()?;
        let resp = client.get_followers_by_id(&params).await;
        assert!(resp.is_ok());
        Ok(())
    }
}
