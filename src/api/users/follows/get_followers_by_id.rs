use std::{collections::HashMap, fmt::Error};
use crate::authentication::Client;
use serde_json::value::Value;
use super::params_structs::{PGetFollowersById, PGetFollowersByIdBuilder, QPGetFollowersById};

impl Client {
    pub async fn get_followers_by_id(&self, params: &PGetFollowersById) -> Result<Value, reqwest::Error> {
        let bearer_header = format!("Bearer {}", &self.bearer_token[..]);
        let url = format!("https://api.twitter.com/2/users/{}/followers", params.id);
        let reqwest_client = reqwest::Client::new();
        // optional parameters handling
        let empty_vec: Vec<String> = vec![String::from("")];

        let expansions = params.expansions.as_ref().unwrap_or_else(|| &empty_vec);

        let tweet_fields = params.tweet_fields.as_ref().unwrap_or_else(|| &empty_vec);

        let user_fields = params.user_fields.as_ref().unwrap_or_else(|| &empty_vec);

        let max_results = &params.max_results.unwrap_or_else(|| 1);

        // TODO fix pagination token
        // let pagination_token = params.pagination_token.as_ref().unwrap_or_else(|| &empty_string);
        // ----------------------------

        let query_params = QPGetFollowersById {
            max_results: *max_results,
            expansions: expansions.join(","),
        };

        let mut query_params_2: HashMap<String, String> = HashMap::new();
        query_params_2.insert("user.fields".to_string(), user_fields.join(","));
        query_params_2.insert("tweet.fields".to_string(), tweet_fields.join(","));

        let user_request = match reqwest_client.get(url)
            .header(reqwest::header::AUTHORIZATION, bearer_header)
            .query(&query_params)
            .query(&query_params_2)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(e),
            };
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
        println!("{:?}", resp);
        assert!(resp.is_ok());
        Ok(())
    }
}
