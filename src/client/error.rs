/// `Result` type-alias
pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    /// Error enum for vault-rs
    #[derive(Debug)]
    pub enum Error {
        /// `isahc::Error` errors
        Isahc(err: ::isahc::Error) {
            from()
            description("isahc error")
            display("isahc error: {}", err)
            cause(err)
        }
        /// `serde_json::Error`
        SerdeJson(err: ::serde_json::Error) {
            from()
            description("serde_json Error")
            display("serde_json Error: {}", err)
            cause(err)
        }
        /// Vault errors
        Vault(err: String) {
            description("vault error")
            display("vault error: {}", err)
        }
        /// Response from Vault errors
        /// This is for when the response is not successful.
        VaultResponse(response: isahc::Response<String>) {
            description("vault response error")
            display("Error in vault response: {:?}", response)
        }
        /// IO errors
        Io(err: ::std::io::Error) {
            from()
            description("io error")
            display("io error: {}", err)
            cause(err)
        }
        /// `Url` parsing error
        Url(err: ::url::ParseError) {
            from()
            description("url parse error")
            display("url parse error: {}", err)
            cause(err)
        }
        /// `BuildRequest` parsing error
        BuildRequest(err: ::http::Error) {
            description("url parse error")
            display("url parse error: {}", err)
            cause(err)
        }
        /// `Base64` decode error
        Base64(err: ::base64::DecodeError) {
            from()
                description("base64 decode error")
                display("base64 decode error: {}", err)
                cause(err)
        }
        /// `Utf8` decode error
        Utf8(err: ::std::string::FromUtf8Error) {
            from()
                description("base64 decode error")
                display("base64 decode error: {}", err)
                cause(err)
        }
    }
}
