use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AuthBody {
    pub grant_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    token_type: String,
    pub access_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};
    #[test]
    fn check_auth_body() {
        let grant_type = String::from("extraordinary");
        let a_body = AuthBody {
            grant_type: grant_type.clone()
        };
        assert_eq!(a_body.grant_type, grant_type);
    }
    #[test]
    fn check_auth_response() {
        let token_type = String::from("Bearer");
        let access_token = String::from(rand::thread_rng().gen_range(13213..32323132).to_string());
        let a_resp = AuthResponse {
            token_type: token_type.clone(),
            access_token: access_token.clone(),
        };
        assert_eq!(token_type, a_resp.token_type);
        assert_eq!(access_token, a_resp.access_token);
    }
}
