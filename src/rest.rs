use std::time::Duration;

use serde::{Serialize, Deserialize};

use crate::utils::{process_info, request::build_reqwest_client};

#[derive(Clone)]
/// A client for the League-Client(LCU) REST API
pub struct RESTClient {
    client: reqwest::Client,
    remoting: bool,
    pub lcu_client_info: LCUClientInfo
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LCUClientInfo {
    pub port: u16,
    pub token: String,
    pub remoting_port: u16,
    pub remoting_token: String,
}

type Error = Box<dyn std::error::Error>;

impl RESTClient {
    /// Create a new instance of the LCU REST wrapper
    pub fn new(lcu_info: LCUClientInfo, remoting: bool) -> Result<Self, Error> {
        let client = if remoting {
            build_reqwest_client(Some(&lcu_info.remoting_token))
        } else {
            build_reqwest_client(Some(&lcu_info.token))
        };

        Ok(Self {
            client,
            lcu_client_info: lcu_info,
            remoting,
        })
    }

    fn get_port(&self) -> u16 {
        if self.remoting {
            self.lcu_client_info.remoting_port
        } else {
            self.lcu_client_info.port
        }
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        let port = self.get_port();
        let req: serde_json::Value = self
            .client
            .get(format!(
                "https://127.0.0.1:{}{}",
                port, endpoint
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a get request that may take longer than the client's default timeout, e.g. one the
    /// League Client answers by asking Riot's servers (match history)
    pub async fn get_with_timeout(
        &self,
        endpoint: String,
        timeout: Duration,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let port = self.get_port();
        let req: serde_json::Value = self
            .client
            .get(format!(
                "https://127.0.0.1:{}{}",
                port, endpoint
            ))
            .timeout(timeout)
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a post request to the specified endpoint
    pub async fn post<T: Serialize>(
        &self,
        endpoint: String,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let port = self.get_port();
        let response = self
            .client
            .post(format!(
                "https://127.0.0.1:{}{}",
                port, endpoint
            ))
            .json(&body)
            .send()
            .await?;
        let req = action_response(response).await?;

        Ok(req)
    }

    pub async fn post_no_body(
        &self,
        endpoint: String
    ) -> Result<serde_json::Value, reqwest::Error> {
        let port = self.get_port();
        let response = self
            .client
            .post(format!(
                "https://127.0.0.1:{}{}",
                port, endpoint
            ))
            .send()
            .await?;
        let req = action_response(response).await?;

        Ok(req)
    }

    /// Make a put request to the specified endpoint
    pub async fn put<T: Serialize>(
        &self,
        endpoint: String,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let port = self.get_port();
        let response = self
            .client
            .put(format!(
                "https://127.0.0.1:{}{}",
                port, endpoint
            ))
            .json(&body)
            .send()
            .await?;
        let req = action_response(response).await?;

        Ok(req)
    }

    /// Make a delete request to the specified endpoint
    pub async fn delete(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        let port = self.get_port();
        let response = self
            .client
            .delete(format!(
                "https://127.0.0.1:{}{}",
                port, endpoint
            ))
            .send()
            .await?;
        let req = action_response(response).await?;

        Ok(req)
    }
}

/// Fails on an error status, and reads an empty answer (League often replies 204 No Content to
/// actions like accepting a ready check) as null instead of a JSON parse error.
async fn action_response(response: reqwest::Response) -> Result<serde_json::Value, reqwest::Error> {
    let response = response.error_for_status()?;
    if response.status() == reqwest::StatusCode::NO_CONTENT || response.content_length() == Some(0) {
        return Ok(serde_json::Value::Null);
    }
    response.json().await
}
