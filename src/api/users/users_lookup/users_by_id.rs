use crate::authentication::Client;
use serde_json::value::Value;
use std::collections::HashMap;
use super::params_structs::{PGetUsersByIds, PGetUsersByIdsBuilder, QPGetUsersByIds};
const USERS_BY_ID_ENDPOINT: &str  = "https://api.twitter.com/2/users";

impl Client {
    pub async fn get_users_by_ids(&self, params: &PGetUsersByIds) -> Result<Value, reqwest::Error> {
        let bearer_header = format!("Bearer {}", &self.bearer_token[..]);
        let reqwest_client = reqwest::Client::new();
        let empty_vec: Vec<String> = vec![String::from("")];
        let expansions = params.expansions.as_ref().unwrap_or_else(|| &empty_vec);
        let tweet_fields = params.tweet_fields.as_ref().unwrap_or_else(|| &empty_vec);
        let user_fields = params.user_fields.as_ref().unwrap_or_else(|| &empty_vec);
        let query_params = QPGetUsersByIds {
            ids: params.ids.join(" "),
            expansions: expansions.join(",")
        };
        let mut query_params_2 = HashMap::new();
        query_params_2.insert("user.fields", user_fields.join(","));
        query_params_2.insert("tweet.fields", tweet_fields.join(","));
        let user_request = match reqwest_client.get(USERS_BY_ID_ENDPOINT)
            .header(reqwest::header::AUTHORIZATION, bearer_header)
            .query(&query_params)
            .query(&query_params_2)
            .send().await {
                Ok(resp) => resp,
                Err(e) => return Err(e),
        };
        let resp_textified = user_request.text().await.expect("Error while trying to textify response");
        let resp_jsonified: Value = serde_json::from_str(&resp_textified).expect("Error while trying to jsonify response");
        Ok(resp_jsonified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::get_api_credentials;
    #[tokio::test]
    async fn check_user_by_ids() -> Result<(), Box<dyn std::error::Error>>{
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client {api_key, api_secret, bearer_token: String::from("")};
        let id = String::from("1504478166248611840");
        client.authenticate().await;
        let params = PGetUsersByIdsBuilder::default()
            .ids(vec![id.clone()]).build()?;
        let resp = client.get_users_by_ids(&params).await?;
        assert_eq!(id, resp["data"][0]["id"]);
        Ok(())
    }
    #[tokio::test]
    async fn check_users_by_ids_with_expansions() -> Result<(), Box<dyn std::error::Error>>{
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client {api_key, api_secret, bearer_token: String::from("")};
        let id = String::from("1504478166248611840");
        let expansions = vec!["pinned_tweet_id".to_string()];
        client.authenticate().await;
        let params = PGetUsersByIdsBuilder::default()
            .ids(vec![id.clone()]).expansions(expansions).build()?;
        let resp = client.get_users_by_ids(&params).await?;
        assert_eq!(id, resp["data"][0]["id"]);
        Ok(())
    }
    #[tokio::test]
    async fn check_users_by_ids_all() -> Result<(), Box<dyn std::error::Error>>{
        let (api_key, api_secret) = get_api_credentials()?;
        let mut client = Client {api_key, api_secret, bearer_token: String::from("")};
        let id = String::from("1504478166248611840");
        let expansions = vec!["pinned_tweet_id".to_string()];
        let tweet_fields = vec!["created_at".to_string()];
        let user_fields = vec!["created_at".to_string(), "profile_image_url".to_string()];
        client.authenticate().await;
        let params = PGetUsersByIdsBuilder::default()
            .ids(vec![id.clone()])
            .expansions(expansions)
            .tweet_fields(tweet_fields)
            .user_fields(user_fields)
            .build()?;
        let resp = client.get_users_by_ids(&params).await?;
        assert_eq!(id, resp["data"][0]["id"]);
        Ok(())
    }
}
