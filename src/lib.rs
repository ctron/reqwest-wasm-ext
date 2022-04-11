use core::fmt;
use reqwest::RequestBuilder;
use std::fmt::Display;

/// Allow using `basic_auth` on both wasm and non-wasm targets
pub trait ReqwestExt: Sized {
    /// Drop in replacement for [`reqwest::RequestBuilder::basic_auth`].
    ///
    /// This is a drop in replacement. However, it requires that the trait is in the context. In
    /// the case of a non-wasm target, the function is present, the trait unused and a warning
    /// will be issued.
    ///
    /// To fix this warning, you can either:
    ///   * Restrict the trait import using `#[cfg(target_arch = "wasm32")]`
    ///   * Use the [`basic_auth_ext`] function
    ///
    /// On non-wasm32 targets, the function directly calls [`reqwest::RequestBuilder::basic_auth`].
    fn basic_auth<U, P>(self, username: U, password: Option<P>) -> Self
    where
        U: fmt::Display,
        P: fmt::Display;

    #[inline]
    fn basic_auth_ext<U, P>(self, username: U, password: Option<P>) -> Self
    where
        U: fmt::Display,
        P: fmt::Display,
    {
        self.basic_auth(username, password)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ReqwestExt for RequestBuilder {
    fn basic_auth<U, P>(self, username: U, password: Option<P>) -> RequestBuilder
    where
        U: Display,
        P: Display,
    {
        self.basic_auth(username, password)
    }
}

#[cfg(target_arch = "wasm32")]
impl ReqwestExt for RequestBuilder {
    fn basic_auth<U, P>(self, username: U, password: Option<P>) -> RequestBuilder
    where
        U: Display,
        P: Display,
    {
        use base64::write::EncoderWriter as Base64Encoder;
        use std::io::Write;

        let mut header_value = b"Basic ".to_vec();
        {
            let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
            // The unwraps here are fine because Vec::write* is infallible.
            write!(encoder, "{}:", username).unwrap();
            if let Some(password) = password {
                write!(encoder, "{}", password).unwrap();
            }
        }

        self.header(reqwest::header::AUTHORIZATION, header_value)
    }
}

#[cfg(test)]
mod test {

    use reqwest::Client;

    #[test]
    fn test() {
        #[cfg(target_arch = "wasm32")]
        use super::ReqwestExt;

        let client = Client::new();
        // the following line should compile on wasm and non-wasm targets
        let _req = client
            .get("http://localhost")
            .basic_auth("foo", Some("bar"));
    }

    #[test]
    fn test_ext() {
        use super::ReqwestExt;

        let client = Client::new();
        // the following line should compile on wasm and non-wasm targets
        let _req = client
            .get("http://localhost")
            .basic_auth_ext("foo", Some("bar"));
    }
}
