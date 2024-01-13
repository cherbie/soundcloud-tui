use std::{io::ErrorKind, result::Result, str::FromStr};

#[allow(dead_code)]
pub struct UserInfo {
    id: String,
    full_name: String,
    last_name: String,
    username: String,
}

impl TryFrom<serde_json::Value> for UserInfo {
    type Error = std::io::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if !value.is_object() {
            return Err(Self::Error::new(
                ErrorKind::InvalidData,
                "value is not valid JSON",
            ));
        }

        let id = match value["id"].as_str() {
            Some(v) => v,
            None => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "id not defined",
                ))
            }
        };
        let full_name = match value["full_name"].as_str() {
            Some(v) => v,
            None => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "full_name not defined",
                ))
            }
        };
        let last_name = match value["last_name"].as_str() {
            Some(v) => v,
            None => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "last_name not defined",
                ))
            }
        };
        let username = match value["username"].as_str() {
            Some(v) => v,
            None => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "username not defined",
                ))
            }
        };

        Ok(Self {
            id: String::from(id),
            full_name: String::from(full_name),
            last_name: String::from(last_name),
            username: String::from(username),
        })
    }
}

pub(crate) struct SoundCloudClientConfig {
    base_url: String,
    #[allow(dead_code)]
    token: String,
}

#[allow(dead_code)]
pub async fn get_user_info<'a>(config: &SoundCloudClientConfig) -> std::io::Result<UserInfo> {
    let me_api_url = config.base_url.clone() + "/me";
    let response = reqwest::get(me_api_url)
        .await
        .map_err(|err| std::io::Error::new(ErrorKind::Other, err.to_string()))?;

    if let Err(err) = response.error_for_status_ref() {
        return Err(std::io::Error::new(ErrorKind::Other, err.to_string()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|err| std::io::Error::new(ErrorKind::InvalidData, err.to_string()))?;
    let response_body = serde_json::Value::from_str(&response_text)?;
    let user_info = UserInfo::try_from(response_body)?;

    Ok(user_info)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::{self, Mock, ServerGuard};
    use serde_json::{self, json};

    async fn mock_soundcloud_me_api(server: &mut ServerGuard, body: &serde_json::Value) -> Mock {
        server
            .mock("GET", "/me")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .create_async()
            .await
    }

    #[tokio::test]
    async fn test_get_user_info() {
        let mut server = mockito::Server::new_async().await;
        let url = mockito::Server::url(&server);

        let client_config = SoundCloudClientConfig {
            base_url: url,
            token: String::from("deadbeef"),
        };

        let expected_id = "user_id";
        let expected_full_name = "full_name";
        let expected_last_name = "last_name";
        let expected_username = "username";
        let expected_request_body = json!({"id": expected_id, "full_name": expected_full_name, "last_name": expected_last_name, "username": expected_username});

        let soundcloud_me_mock = mock_soundcloud_me_api(&mut server, &expected_request_body).await;

        let user_info = get_user_info(&client_config).await.unwrap();

        soundcloud_me_mock.assert();

        assert_eq!(user_info.id, expected_id);
        assert_eq!(user_info.full_name, expected_full_name);
        assert_eq!(user_info.last_name, expected_last_name);
        assert_eq!(user_info.username, expected_username);
    }
}
