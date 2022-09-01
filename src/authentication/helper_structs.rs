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
    #[test]
    fn check_auth_body() {
        let grant_type = String::from("pam");
        let a_body = AuthBody {
            grant_type: grant_type.clone()
        };
        assert_eq!(a_body.grant_type, grant_type);
    }
}
