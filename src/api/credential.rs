use super::{error::ApiError, Options};
use anyhow::{bail, ensure, Ok};
use base64::{prelude::BASE64_STANDARD, Engine};
use hmac::{Hmac, Mac};
use reqwest::{Method, Url};
use sha2::Sha256;
use std::convert::TryFrom;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug)]
pub struct Credential {
    api_key: String,
    secret_key: String,
}

impl Credential {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub(crate) fn signature(
        &self,
        method: Method,
        timestamp: &str,
        url: &Url,
        body: &str,
    ) -> (&str, String) {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("hmac can take key of any size");

        let msg = match url.query() {
            Some(query) => format!(
                "{}{}{}?{}{}",
                timestamp,
                method.as_str(),
                url.path(),
                query,
                body
            ),
            None => format!("{}{}{}{}", timestamp, method.as_str(), url.path(), body),
        };

        mac.update(msg.as_bytes());
        let result = mac.finalize();
        let result_bytes = result.into_bytes();
        let signed = BASE64_STANDARD.encode::<&[u8]>(result_bytes.as_ref());

        (self.api_key.as_str(), signed)
    }

    pub(crate) fn signature_websocket(
        &self,
        method: Method,
        timestamp: &str,
        url: &str,
    ) -> (&str, String) {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("hmac can take key of any size");

        let msg = format!("{}{}{}", timestamp, method.as_str(), url);

        mac.update(msg.as_bytes());
        let result = mac.finalize();
        let result_bytes = result.into_bytes();
        let signed = BASE64_STANDARD.encode::<&[u8]>(result_bytes.as_ref());

        (self.api_key.as_str(), signed)
    }
}

impl TryFrom<&Options> for Credential {
    type Error = anyhow::Error;

    fn try_from(opt: &Options) -> Result<Self, Self::Error> {
        ensure!(opt.api_key.is_some(), "api key is not set");
        ensure!(opt.secret_key.is_some(), "secret key is not set");

        if let (Some(api_key), Some(secret_key)) = (&opt.api_key, &opt.secret_key) {
            Ok(Self {
                api_key: api_key.to_owned(),
                secret_key: secret_key.to_owned(),
            })
        } else {
            bail!("not enough credentials from Options")
        }
    }
}
