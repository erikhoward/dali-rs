// Copyright 2022 Erik Howard <erikhoward@pm.me>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

const USER_AGENT: &str = concat!("dali-rs/", env!("CARGO_PKG_VERSION"));

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

    // return base64 encoded auth string
    fn auth(&self) -> String {
        let auth = format!("{}:{}", self.username, self.password);
        let auth = base64::encode(auth);
        format!("Basic {}", auth)
    }
}

#[cfg(test)]
mod tests {

    fn can_build_client() {
        use crate::client::ClientBuilder;
        let client = ClientBuilder::default()
            .username("test")
            .password("test")
            .build()
            .unwrap();
        assert_eq!(client.username, "test");
    }

    #[test]
    fn can_build_client_with_defaults() {
        use crate::client::ClientBuilder;
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
        use crate::client::ClientBuilder;
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
        use crate::client::Client;
        let client = Client::new()
            .username("test")
            .password("test")
            .build()
            .unwrap();
        assert_eq!(client.username, "test");
    }
}