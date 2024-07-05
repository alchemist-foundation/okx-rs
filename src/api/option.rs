use std::sync::Arc;

pub trait OKXEnv {
    fn rest(&self) -> &str;
    fn public_websocket(&self) -> &str;
    fn private_websocket(&self) -> &str;
    fn business_websocket(&self) -> &str;
    fn headers(&self) -> Option<&[(&str, &str)]> {
        None
    }
}

#[derive(Clone)]
pub struct LiveTrading;

impl OKXEnv for LiveTrading {
    fn rest(&self) -> &str {
        "https://www.okx.com/api/v5"
    }

    fn public_websocket(&self) -> &str {
        "wss://ws.okx.com:8443/ws/v5/public"
    }

    fn private_websocket(&self) -> &str {
        "wss://ws.okx.com:8443/ws/v5/private"
    }

    fn business_websocket(&self) -> &str {
        "wss://ws.okx.com:8443/ws/v5/business"
    }
}

#[derive(Clone)]
pub struct DemoTrading;

impl OKXEnv for DemoTrading {
    fn rest(&self) -> &str {
        "https://www.okx.com/api/v5"
    }

    fn public_websocket(&self) -> &str {
        "wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999"
    }

    fn private_websocket(&self) -> &str {
        "wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999"
    }

    fn business_websocket(&self) -> &str {
        "wss://wspap.okx.com:8443/ws/v5/business?brokerId=9999"
    }

    fn headers(&self) -> Option<&[(&str, &str)]> {
        Some(&[("x-simulated-trading", "1")])
    }
}

#[derive(Clone)]
pub struct Options {
    pub env: Arc<dyn OKXEnv>,
    pub api_key: Option<String>,
    pub secret_key: Option<String>,
    pub passphrase: Option<String>,
}

impl Options {
    pub fn new(env: impl OKXEnv + 'static) -> Options {
        Self {
            env: Arc::new(env),
            api_key: None,
            secret_key: None,
            passphrase: None,
        }
    }

    pub fn new_with_credential(
        env: impl OKXEnv + 'static,
        api_key: impl AsRef<str>,
        secret_key: impl AsRef<str>,
        passphrase: impl AsRef<str>,
    ) -> Self {
        Self {
            env: Arc::new(env),
            api_key: Some(api_key.as_ref().to_string()),
            secret_key: Some(secret_key.as_ref().to_string()),
            passphrase: Some(passphrase.as_ref().to_string()),
        }
    }
}

impl Options {
    pub fn rest(&self) -> &str {
        self.env.rest()
    }

    pub fn public_websocket(&self) -> &str {
        self.env.public_websocket()
    }

    pub fn private_websocket(&self) -> &str {
        self.env.private_websocket()
    }

    pub fn business_websocket(&self) -> &str {
        self.env.business_websocket()
    }
}
