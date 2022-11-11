// Copyright 2022 Erik Howard <erikhoward@pm.me>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

use ureq::{AgentBuilder};
use serde::{Deserialize, Serialize};

const USER_AGENT: &str = concat!("dali-rs/", env!("CARGO_PKG_VERSION"));

#[derive(Debug)]
pub enum RequestMethod {
    GET,
    PATCH,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurrealHTTPResponse {
    pub time: String,
    pub status: String,
    pub result: Vec<serde_json::Value>,
}

// TODO: Create connection options for TLS and Timeouts

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Client {
    #[builder(default = "String::from(\"localhost\")")]
    host: String,
    #[builder(default = "8000")]
    port: i32,
    #[builder(default = "String::from(\"root\")")]
    username: String,
    #[builder(default = "String::from(\"root\")")]
    password: String,
    #[builder(default = "String::from(\"test\")")]
    namespace: String,
    #[builder(default = "String::from(\"test\")")]
    database: String,
}

impl Client {
    pub fn new() -> ClientBuilder {
        ClientBuilder::default()
    }

    fn auth(&self) -> String {
        let auth = format!("{}:{}", self.username, self.password);
        let auth = base64::encode(auth);
        format!("Basic {}", auth)
    }

    pub fn execute(&self, uri: &str, data: &str) -> Result<serde_json::Value, ureq::Error> {
        let agent = AgentBuilder::new()
            .user_agent(USER_AGENT)
            .build();

        let response = agent.post(uri)
            .set("Accept", "application/json")
            .set("Authorization",&self.auth())
            .set("DB", &self.database)
            .set("NS", &self.namespace)
            .send_string(data)?;

        // return serde json
        Ok(response.into_json()?)
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_client() {
        let client = ClientBuilder::default()
            .username("test")
            .password("test")
            .build()
            .unwrap();
        assert_eq!(client.username, "test");
    }

    #[test]
    fn can_build_client_with_defaults() {
        let client = ClientBuilder::default().build().unwrap();
        assert_eq!(client.host, "localhost");
        assert_eq!(client.port, 8000);
        assert_eq!(client.username, "root");
        assert_eq!(client.password, "root");
        assert_eq!(client.namespace, "test");
        assert_eq!(client.database, "test");
    }

    #[test]
    fn can_auth() {
        let client = ClientBuilder::default()
            .username("test")
            .password("test")
            .build()
            .unwrap();
        let auth = client.auth();
        assert_eq!(auth, "Basic dGVzdDp0ZXN0");
    }

    // test Client new impl
    #[test]
    fn can_build_client_with_new() {
        let client = Client::new()
            .username("test")
            .password("test")
            .build()
            .unwrap();
        assert_eq!(client.username, "test");
    }

    #[test]
    // test execute method
    fn can_execute_query() {
        let client = Client::new()
            .username("root")
            .password("root")
            .build()
            .unwrap();
        match client.execute("http://20.163.28.54:8000/sql", "select * from account;") {
            Ok(response) => {
                assert_eq!(response[0]["status"], "OK");
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}