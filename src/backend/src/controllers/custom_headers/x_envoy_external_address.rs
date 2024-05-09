use std::fmt::Display;

use headers::{Header, HeaderName, HeaderValue};

pub struct XEnvoyExternalAddress(String);

impl XEnvoyExternalAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for XEnvoyExternalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

static X_ENVOY_EXTERNAL_ADDRESS_HEADER: HeaderName =
    HeaderName::from_static("x-envoy-external-address");

impl Header for XEnvoyExternalAddress {
    fn name() -> &'static HeaderName {
        &X_ENVOY_EXTERNAL_ADDRESS_HEADER
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;

        let s = value.to_str().map_err(|_| headers::Error::invalid())?;

        Ok(Self(s.to_string()))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        values.extend(std::iter::once(HeaderValue::from_str(&self.0).unwrap()));
    }
}
