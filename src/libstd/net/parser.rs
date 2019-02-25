use error::Error;
use net::AddrParseError;

#[stable(feature = "addr_parse_error_error", since = "1.4.0")]
impl Error for AddrParseError {
    fn description(&self) -> &str {
        "invalid IP address syntax"
    }
}
