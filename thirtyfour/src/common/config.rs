use crate::error::WebDriverError;
use crate::{
    extensions::query::{ElementPollerWithTimeout, IntoElementPoller},
    prelude::WebDriverResult,
};
use const_format::formatcp;
use http::HeaderValue;
use std::sync::Arc;

/// Configuration options used by a `WebDriver` instance and the related `SessionHandle`.
///
/// The configuration of a `WebDriver` will be shared by all elements found via that instance.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct WebDriverConfig {
    /// If true, send the "Connection: keep-alive" header with all requests.
    pub keep_alive: bool,
    /// The default poller to use when performing element queries or waits.
    pub poller: Arc<dyn IntoElementPoller + Send + Sync>,
    /// The user agent to use when sending commands to the webdriver server.
    pub user_agent: HeaderValue,
}

impl Default for WebDriverConfig {
    fn default() -> Self {
        Self::builder().build().expect("default values failed")
    }
}

impl WebDriverConfig {
    /// Create new `WebDriverConfigBuilder`.
    pub fn builder() -> WebDriverConfigBuilder {
        WebDriverConfigBuilder::new()
    }

    /// The default user agent.
    pub const DEFAULT_USER_AGENT: HeaderValue = {
        //noinspection RsReplaceMatchExpr
        const RUST_VER: &str = match option_env!("RUSTC_VERSION") {
            Some(ver) => ver,
            None => "unknown",
        };

        //noinspection RsCompileErrorMacro
        const HEADER: &str = formatcp!(
            "thirtyfour/{} (rust/{}; {})",
            crate::VERSION,
            RUST_VER,
            std::env::consts::OS
        );

        HeaderValue::from_static(HEADER)
    };

    /// Get the default user agent.
    #[deprecated(
        since = "0.34.1",
        note = "This associated function is now a constant `WebDriverConfig::DEFAULT_USER_AGENT`"
    )]
    pub fn default_user_agent() -> HeaderValue {
        Self::DEFAULT_USER_AGENT
    }
}

/// Builder for `WebDriverConfig`.
#[derive(Debug)]
pub struct WebDriverConfigBuilder {
    keep_alive: bool,
    poller: Option<Arc<dyn IntoElementPoller + Send + Sync>>,
    user_agent: Option<WebDriverResult<HeaderValue>>,
}

impl Default for WebDriverConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WebDriverConfigBuilder {
    /// Create a new `WebDriverConfigBuilder`.
    pub fn new() -> Self {
        Self {
            keep_alive: true,
            poller: None,
            user_agent: None,
        }
    }

    /// Set the keep_alive option.
    pub fn keep_alive(mut self, keep_alive: bool) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    /// Set the specified element poller.
    pub fn poller(mut self, poller: Arc<dyn IntoElementPoller + Send + Sync>) -> Self {
        self.poller = Some(poller);
        self
    }

    /// Set the user agent.
    pub fn user_agent<V>(mut self, user_agent: V) -> Self
    where
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<WebDriverError>,
    {
        self.user_agent = Some(user_agent.try_into().map_err(Into::into));
        self
    }

    /// Build `WebDriverConfig` using builder options.
    pub fn build(self) -> WebDriverResult<WebDriverConfig> {
        Ok(WebDriverConfig {
            keep_alive: self.keep_alive,
            poller: self.poller.unwrap_or_else(|| Arc::new(ElementPollerWithTimeout::default())),
            user_agent: self.user_agent.transpose()?.unwrap_or(WebDriverConfig::DEFAULT_USER_AGENT),
        })
    }
}
