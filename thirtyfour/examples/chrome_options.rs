//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//! Run as follows:
//!
//!     cargo run --example chrome_options

use std::{thread, time::Duration};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut caps = DesiredCapabilities::chrome();
    caps.insert_browser_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    )?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;

    thread::sleep(Duration::from_millis(10000));thread::sleep(Duration::from_millis(10000));

    // Always explicitly close the browser. This prevents the executor from being blocked
    driver.quit().await?;

    Ok(())
}
