use headers::{Header, HeaderName, HeaderValue};

pub struct XForwardedFor(String);

impl XForwardedFor {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ToString for XForwardedFor {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

static X_FORWARDED_FOR_HEADER: HeaderName = HeaderName::from_static("x-forwarded-for");

impl Header for XForwardedFor {
    fn name() -> &'static HeaderName {
        &X_FORWARDED_FOR_HEADER
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
