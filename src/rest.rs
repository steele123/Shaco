use serde::{Serialize, Deserialize};

use crate::utils::{process_info, request::build_reqwest_client};

#[derive(Clone)]
/// A client for the League-Client(LCU) REST API
pub struct RESTClient {
    reqwest_client: reqwest::Client,
    pub lcu_client_info: LCUClientInfo,
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
    pub fn new(lcu_info: LCUClientInfo) -> Result<Self, Error> {
        let reqwest_client = build_reqwest_client(Some(&lcu_info.token));

        Ok(Self {
            reqwest_client,
            lcu_client_info: lcu_info,
        })
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: String) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .get(format!(
                "https://127.0.0.1:{}{}",
                self.lcu_client_info.port, endpoint
            ))
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
            .post(format!(
                "https://127.0.0.1:{}{}",
                self.lcu_client_info.port, endpoint
            ))
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
            .put(format!(
                "https://127.0.0.1:{}{}",
                self.lcu_client_info.port, endpoint
            ))
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
            .delete(format!(
                "https://127.0.0.1:{}{}",
                self.lcu_client_info.port, endpoint
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }
}
