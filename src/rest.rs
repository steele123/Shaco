use serde::Serialize;

use crate::utils::{process_info, request::build_reqwest_client};

#[derive(Clone)]
/// A client for the League-Client(LCU) REST API
pub struct RESTClient {
    reqwest_client: reqwest::Client,
    pub lcu_client_info: LCUClientInfo,
}

#[derive(Clone)]
pub struct LCUClientInfo {
    pub port: u16,
    pub token: String,
    pub remoting_port: u16,
    pub remoting_token: String,
}

type Error = Box<dyn std::error::Error>;

impl RESTClient {
    /// Create a new instance of the LCU REST wrapper
    pub fn new() -> Result<Self, Error> {
        let (auth_token, port, remoting_token, remoting_port) = process_info::get_auth_info()?;
        let reqwest_client = build_reqwest_client(Some(&auth_token));
        Ok(Self {
            reqwest_client,
            lcu_client_info: LCUClientInfo {
                port: port.parse::<u16>().unwrap(),
                token: auth_token,
                remoting_port: remoting_port.parse::<u16>().unwrap(),
                remoting_token,
            },
        })
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .get(format!("https://127.0.0.1:{}{}", self.lcu_client_info.port, endpoint))
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
        let req: serde_json::Value = self
            .reqwest_client
            .post(format!("https://127.0.0.1:{}{}", self.lcu_client_info.port, endpoint))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a put request to the specified endpoint
    pub async fn put<T: Serialize>(
        &self,
        endpoint: String,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .put(format!("https://127.0.0.1:{}{}", self.lcu_client_info.port, endpoint))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a delete request to the specified endpoint
    pub async fn delete(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .delete(format!("https://127.0.0.1:{}{}", self.lcu_client_info.port, endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }
}
