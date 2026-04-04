// web.rs - Advanced Web Automation Tool for Pentesting AI Agent
// This provides complete web interaction capabilities including authentication,
// form handling, scraping, bypass techniques, and advanced browser automation.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Utc};
use fantoccini::{Client as WebDriver, ClientBuilder as WdClientBuilder, Locator};
use futures::future::join_all;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};
use rand::distr::Alphanumeric;
use rand::prelude::*;
use regex::Regex;
use reqwest::multipart::{Form, Part};
use reqwest::{
    cookie::Jar,
    header::{
        HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION,
        CONNECTION, CONTENT_TYPE, COOKIE, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
    },
    redirect::Policy,
    Client, ClientBuilder, Proxy, Response,
};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ffi::OsString;
use tokio::sync::Mutex;
use tokio::time::sleep;
use url::Url;
use urlencoding;

// ============================================================
// Error Types
// ============================================================

#[derive(Debug)]
pub enum WebError {
    RequestError(String),
    ParseError(String),
    AuthError(String),
    TimeoutError(String),
    JavascriptError(String),
    CaptchaError(String),
    NavigationError(String),
    ConfigError(String),
    IoError(std::io::Error),
    BrowserError(String),
    NetworkError(String),
    RateLimitError(String),
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebError::RequestError(e) => write!(f, "Request error: {}", e),
            WebError::ParseError(e) => write!(f, "Parse error: {}", e),
            WebError::AuthError(e) => write!(f, "Authentication error: {}", e),
            WebError::TimeoutError(e) => write!(f, "Timeout error: {}", e),
            WebError::JavascriptError(e) => write!(f, "JavaScript error: {}", e),
            WebError::CaptchaError(e) => write!(f, "Captcha error: {}", e),
            WebError::NavigationError(e) => write!(f, "Navigation error: {}", e),
            WebError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            WebError::IoError(e) => write!(f, "IO error: {}", e),
            WebError::BrowserError(e) => write!(f, "Browser error: {}", e),
            WebError::NetworkError(e) => write!(f, "Network error: {}", e),
            WebError::RateLimitError(e) => write!(f, "Rate limit error: {}", e),
        }
    }
}

impl Error for WebError {}

impl From<reqwest::Error> for WebError {
    fn from(err: reqwest::Error) -> Self {
        WebError::RequestError(err.to_string())
    }
}

impl From<std::io::Error> for WebError {
    fn from(err: std::io::Error) -> Self {
        WebError::IoError(err)
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for WebError {
    fn from(err: scraper::error::SelectorErrorKind) -> Self {
        WebError::ParseError(err.to_string())
    }
}

impl From<serde_json::Error> for WebError {
    fn from(err: serde_json::Error) -> Self {
        WebError::ParseError(err.to_string())
    }
}

// ============================================================
// Configuration Types
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub max_redirects: usize,
    pub timeout_seconds: u64,
    pub connect_timeout_seconds: u64,
    pub pool_idle_timeout_seconds: u64,
    pub max_concurrent_requests: usize,
    pub retry_attempts: usize,
    pub retry_delay_ms: u64,
    pub user_agents: Vec<String>,
    pub proxy_list: Vec<String>,
    pub cookies_file: Option<String>,
    pub storage_dir: String,
    pub headless_browser: bool,
    pub browser_binary: Option<String>,
    pub chrome_driver_path: Option<String>,
    pub captcha_services: CaptchaConfig,
    pub rate_limit_requests_per_second: Option<u64>,
    pub stealth_mode: bool,
    pub fingerprint_rotation: bool,
    pub all_report_path: Option<String>,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            max_redirects: 10,
            timeout_seconds: 60,
            connect_timeout_seconds: 15,
            pool_idle_timeout_seconds: 90,
            max_concurrent_requests: 10,
            retry_attempts: 3,
            retry_delay_ms: 1000,
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            ],
            proxy_list: Vec::new(),
            cookies_file: None,
            storage_dir: "./web_storage".to_string(),
            headless_browser: true,
            browser_binary: None,
            chrome_driver_path: None,
            captcha_services: CaptchaConfig::default(),
            rate_limit_requests_per_second: None,
            stealth_mode: true,
            fingerprint_rotation: false,
            all_report_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaConfig {
    pub two_captcha_api_key: Option<String>,
    pub anti_captcha_api_key: Option<String>,
    pub capsolver_api_key: Option<String>,
    pub auto_solve: bool,
    pub solve_timeout_seconds: u64,
}

impl Default for CaptchaConfig {
    fn default() -> Self {
        Self {
            two_captcha_api_key: None,
            anti_captcha_api_key: None,
            capsolver_api_key: None,
            auto_solve: false,
            solve_timeout_seconds: 120,
        }
    }
}

// ============================================================
// Authentication & Session Management
// ============================================================

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub cookies: HashMap<String, String>,
    pub headers: HeaderMap,
    pub last_used: DateTime<Utc>,
    pub login_url: String,
    pub username: String,
    pub session_token: Option<String>,
    pub csrf_token: Option<String>,
    pub jwt_token: Option<String>,
}

impl AuthSession {
    pub fn new(login_url: String, username: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            ),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));

        Self {
            cookies: HashMap::new(),
            headers,
            last_used: Utc::now(),
            login_url,
            username,
            session_token: None,
            csrf_token: None,
            jwt_token: None,
        }
    }

    pub fn to_cookie_header(&self) -> String {
        self.cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn add_cookie(&mut self, name: String, value: String) {
        self.cookies.insert(name, value);
    }

    pub fn extract_csrf(&mut self, html: &str) -> Option<String> {
        let csrf_patterns = [
            r#"name=["']csrf_token["'] value=["']([^"']+)["']"#,
            r#"name=["']_csrf["'] value=["']([^"']+)["']"#,
            r#"name=["']csrfmiddlewaretoken["'] value=["']([^"']+)["']"#,
            r#"name=["']authenticity_token["'] value=["']([^"']+)["']"#,
            r#"'csrf-token' content=['"]([^'"]+)['"]"#,
            r#"'csrf-param' content=['"]([^'"]+)['"]"#,
        ];

        for pattern in &csrf_patterns {
            let re = Regex::new(pattern).ok()?;
            if let Some(cap) = re.captures(html) {
                let token = cap[1].to_string();
                self.csrf_token = Some(token.clone());
                return Some(token);
            }
        }
        None
    }

    pub fn extract_jwt(&mut self, html: &str) -> Option<String> {
        let jwt_pattern = r#"eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*"#;
        let re = Regex::new(jwt_pattern).ok()?;
        if let Some(mat) = re.find(html) {
            let token = mat.as_str().to_string();
            self.jwt_token = Some(token.clone());
            return Some(token);
        }
        None
    }
}

// ============================================================
// Form Data & Submission
// ============================================================

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub value: String,
    pub field_type: String,
    pub required: bool,
    pub options: Vec<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WebForm {
    pub action: String,
    pub method: String,
    pub enctype: String,
    pub fields: Vec<FormField>,
    pub csrf_token: Option<String>,
}

impl WebForm {
    pub fn from_html(html: &str, form_selector: Option<&str>) -> Result<Self, WebError> {
        let document = Html::parse_document(html);
        let selector = match form_selector {
            Some(s) => Selector::parse(s).map_err(|e| WebError::ParseError(e.to_string()))?,
            None => Selector::parse("form").map_err(|e| WebError::ParseError(e.to_string()))?,
        };

        let form_element = document
            .select(&selector)
            .next()
            .ok_or_else(|| WebError::ParseError("No form found".to_string()))?;

        let action = form_element
            .value()
            .attr("action")
            .unwrap_or("")
            .to_string();
        let method = form_element
            .value()
            .attr("method")
            .unwrap_or("get")
            .to_string()
            .to_uppercase();
        let enctype = form_element
            .value()
            .attr("enctype")
            .unwrap_or("application/x-www-form-urlencoded")
            .to_string();

        let mut fields = Vec::new();
        let input_selector = Selector::parse("input, select, textarea, button").unwrap();

        for element in form_element.select(&input_selector) {
            let name = element.value().attr("name").unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }

            let field_type = element.value().attr("type").unwrap_or("text").to_string();
            let value = element.value().attr("value").unwrap_or("").to_string();
            let required = element.value().attr("required").is_some();

            let mut options = Vec::new();
            if element.value().name() == "select" {
                let option_selector = Selector::parse("option").unwrap();
                for option in element.select(&option_selector) {
                    if let Some(opt_value) = option.value().attr("value") {
                        options.push(opt_value.to_string());
                    }
                }
            }

            let max_length = element
                .value()
                .attr("maxlength")
                .and_then(|v| v.parse().ok());
            let min_length = element
                .value()
                .attr("minlength")
                .and_then(|v| v.parse().ok());
            let pattern = element.value().attr("pattern").map(|s| s.to_string());

            fields.push(FormField {
                name,
                value,
                field_type,
                required,
                options,
                max_length,
                min_length,
                pattern,
            });
        }

        Ok(WebForm {
            action,
            method,
            enctype,
            fields,
            csrf_token: None,
        })
    }

    pub fn to_form_data(
        &self,
        data: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, WebError> {
        let mut form_data = HashMap::new();

        for field in &self.fields {
            if field.required {
                if let Some(value) = data.get(&field.name) {
                    form_data.insert(field.name.clone(), value.clone());
                } else {
                    return Err(WebError::ParseError(format!(
                        "Missing required field: {}",
                        field.name
                    )));
                }
            } else if let Some(value) = data.get(&field.name) {
                form_data.insert(field.name.clone(), value.clone());
            }
        }

        if let Some(csrf) = &self.csrf_token {
            form_data.insert("csrf_token".to_string(), csrf.clone());
        }

        Ok(form_data)
    }

    pub fn to_multipart(&self, data: &HashMap<String, String>) -> Result<Form, WebError> {
        let mut form = Form::new();

        for (name, value) in data {
            form = form.text(name.clone(), value.clone());
        }

        Ok(form)
    }
}

// ============================================================
// Browser Fingerprint & Stealth
// ============================================================

#[derive(Debug, Clone)]
pub struct BrowserFingerprint {
    pub user_agent: String,
    pub accept_language: String,
    pub platform: String,
    pub screen_resolution: (u32, u32),
    pub timezone: String,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub navigator_plugins: Vec<String>,
    pub do_not_track: Option<String>,
    pub hardware_concurrency: u32,
    pub device_memory: f32,
    pub touch_support: bool,
}

impl BrowserFingerprint {
    pub fn random() -> Self {
        let mut rng = rand::rng();

        let user_agents = [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
        ];

        let platforms = ["Win32", "MacIntel", "Linux x86_64", "Win64"];
        let timezones = [
            "America/New_York",
            "Europe/London",
            "Asia/Tokyo",
            "Australia/Sydney",
        ];
        let resolutions = [(1920, 1080), (2560, 1440), (1366, 768), (1536, 864)];

        BrowserFingerprint {
            user_agent: user_agents[rng.random_range(0..user_agents.len())].to_string(),
            accept_language: "en-US,en;q=0.9".to_string(),
            platform: platforms[rng.random_range(0..platforms.len())].to_string(),
            screen_resolution: resolutions[rng.random_range(0..resolutions.len())],
            timezone: timezones[rng.random_range(0..timezones.len())].to_string(),
            webgl_vendor: "Intel Inc.".to_string(),
            webgl_renderer: "Intel Iris OpenGL Engine".to_string(),
            navigator_plugins: vec![
                "Chrome PDF Plugin".to_string(),
                "Chrome PDF Viewer".to_string(),
            ],
            do_not_track: None,
            hardware_concurrency: rng.random_range(4..16),
            device_memory: rng.random_range(4..16) as f32,
            touch_support: false,
        }
    }

    pub fn to_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_str(&self.accept_language).unwrap(),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"));
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));

        if let Some(dnt) = &self.do_not_track {
            headers.insert("DNT", HeaderValue::from_str(dnt).unwrap());
        }

        headers.insert(
            "Sec-Ch-Ua",
            HeaderValue::from_static("\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\""),
        );
        headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
        headers.insert(
            "Sec-Ch-Ua-Platform",
            HeaderValue::from_str(&self.platform).unwrap(),
        );
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
        headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));

        headers
    }
}

// ============================================================
// Rate Limiting & Throttling
// ============================================================

pub struct RateLimiter {
    requests_per_second: f64,
    last_request: Mutex<Instant>,
    tokens: Mutex<f64>,
}

impl RateLimiter {
    pub fn new(requests_per_second: u64) -> Self {
        Self {
            requests_per_second: requests_per_second as f64,
            last_request: Mutex::new(Instant::now()),
            tokens: Mutex::new(requests_per_second as f64),
        }
    }

    pub async fn acquire(&self) {
        let mut last = self.last_request.lock().await;
        let mut tokens = self.tokens.lock().await;

        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs_f64();

        *tokens = (*tokens + elapsed * self.requests_per_second).min(self.requests_per_second);
        *last = now;

        if *tokens < 1.0 {
            let wait_time = Duration::from_secs_f64((1.0 - *tokens) / self.requests_per_second);
            drop(tokens);
            drop(last);
            sleep(wait_time).await;

            let mut last = self.last_request.lock().await;
            let mut tokens = self.tokens.lock().await;
            *last = Instant::now();
            *tokens = self.requests_per_second - 1.0;
        } else {
            *tokens -= 1.0;
        }
    }
}

// ============================================================
// Web Automation Core
// ============================================================

#[derive(Debug, Clone)]
pub struct LoginRequest<'a> {
    pub url: &'a str,
    pub user_field: &'a str,
    pub pass_field: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub session_name: Option<&'a str>,
    pub additional_data: Option<HashMap<String, String>>,
}

pub struct WebAutomation {
    client: Client,
    config: WebConfig,
    sessions: Arc<Mutex<HashMap<String, AuthSession>>>,
    _cookies_jar: Arc<Jar>,
    fingerprints: Arc<Mutex<VecDeque<BrowserFingerprint>>>,
    rate_limiter: Option<RateLimiter>,
    browser: Option<Browser>,
    webdriver_client: Option<WebDriver>,
    _storage_path: PathBuf,
    is_lightpanda: bool,
}

impl WebAutomation {
    pub async fn new(config: WebConfig) -> Result<Self, WebError> {
        // Setup storage directory
        let storage_path = PathBuf::from(&config.storage_dir);
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path)?;
        }

        // Setup cookie jar
        let cookies_jar = Arc::new(Jar::default());

        // Load cookies if exists
        if let Some(cookies_file) = &config.cookies_file {
            let cookies_path = Path::new(cookies_file);
            if cookies_path.exists() {
                Self::load_cookies(&cookies_jar, cookies_path).await?;
            }
        }

        // Build HTTP client
        let mut client_builder = ClientBuilder::new()
            .cookie_provider(cookies_jar.clone())
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(config.connect_timeout_seconds))
            .pool_idle_timeout(Some(Duration::from_secs(config.pool_idle_timeout_seconds)))
            .pool_max_idle_per_host(config.max_concurrent_requests)
            .redirect(Policy::limited(config.max_redirects))
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .brotli(true)
            .deflate(true)
            .gzip(true);

        // Add proxies if configured
        if !config.proxy_list.is_empty() {
            let proxy_url = config.proxy_list[0].clone(); // Use first proxy for now
            let proxy = Proxy::all(&proxy_url)
                .map_err(|e| WebError::ConfigError(format!("Invalid proxy: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }

        // Set default headers
        let fingerprint = BrowserFingerprint::random();
        let headers = fingerprint.to_headers();
        client_builder = client_builder.default_headers(headers);

        let client = client_builder.build()?;

        // Initialize rate limiter
        let rate_limiter = config.rate_limit_requests_per_second.map(RateLimiter::new);

        // Initialize fingerprints pool
        let mut fingerprints = VecDeque::new();
        for _ in 0..10 {
            fingerprints.push_back(BrowserFingerprint::random());
        }

        // Initialize browser with Lightpanda-first strategy
        let (browser, is_lightpanda) = if config.headless_browser {
            match Self::init_hybrid_browser(&config).await {
                Ok((b, lp)) => (Some(b), lp),
                Err(e) => {
                    eprintln!("Warning: Failed to initialize hybrid browser: {}", e);
                    (None, false)
                }
            }
        } else {
            (None, false)
        };

        // Initialize WebDriver client if needed
        let webdriver_client = if let Some(_driver_path) = &config.chrome_driver_path {
            match Self::setup_webdriver(&config).await {
                Ok(c) => Some(c),
                Err(e) => {
                    eprintln!("Warning: Failed to initialize WebDriver: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            client,
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            _cookies_jar: cookies_jar,
            fingerprints: Arc::new(Mutex::new(fingerprints)),
            rate_limiter,
            browser,
            webdriver_client,
            _storage_path: storage_path,
            is_lightpanda,
        })
    }

    async fn init_hybrid_browser(config: &WebConfig) -> Result<(Browser, bool), WebError> {
        tracing::info!("Initializing Elite Hybrid Browser System...");

        // Step 1: Attempt to use Lightpanda via CDP
        match Self::init_lightpanda_cdp(9222).await {
            Ok(lp_browser) => {
                tracing::info!("Lightpanda Engine [ELITE] successfully engaged via CDP.");
                return Ok((lp_browser, true));
            }
            Err(e) => {
                tracing::warn!(
                    "Lightpanda engagement failed: {}. Falling back to native Chromium...",
                    e
                );
            }
        }

        // Step 2: Fallback to native headless chrome
        let native_browser = Self::init_headless_browser(config).await?;
        tracing::info!("Native Chromium Engine [COMPATIBILITY] successfully engaged.");
        Ok((native_browser, false))
    }

    async fn init_lightpanda_cdp(port: u16) -> Result<Browser, WebError> {
        let cdp_url = format!("http://127.0.0.1:{}", port);

        // Check if something is listening on the port
        if std::net::TcpStream::connect_timeout(
            &format!("127.0.0.1:{}", port).parse().unwrap(),
            Duration::from_millis(200),
        )
        .is_err()
        {
            tracing::debug!("Provisioning Lightpanda via binary execution...");

            // Attempt to launch lightpanda in background with tactical flags
            let mut cmd = std::process::Command::new("lightpanda");
            cmd.arg("--cdp")
                .arg(port.to_string())
                .arg("--cdp-host")
                .arg("127.0.0.1")
                .arg("--log-level")
                .arg("error");

            let _lp_process = cmd.spawn().map_err(|e| {
                WebError::BrowserError(format!(
                    "Autonomous provisioning failed. Engine binary not found or non-executable: {}",
                    e
                ))
            })?;

            // Wait for it to settle with a smarter polling loop
            let start = std::time::Instant::now();
            let timeout = Duration::from_secs(5);
            let mut ready = false;

            while start.elapsed() < timeout {
                if std::net::TcpStream::connect_timeout(
                    &format!("127.0.0.1:{}", port).parse().unwrap(),
                    Duration::from_millis(100),
                )
                .is_ok()
                {
                    // Quick check if /json/version responds
                    if let Ok(response) = reqwest::get(format!("{}/json/version", cdp_url)).await {
                        if response.status().is_success() {
                            ready = true;
                            break;
                        }
                    }
                }
                sleep(Duration::from_millis(200)).await;
            }

            if !ready {
                return Err(WebError::BrowserError(
                    "Lightpanda CDP synchronization timeout".to_string(),
                ));
            }
        }

        // Connect headless_chrome to the CDP endpoint
        Browser::connect(cdp_url)
            .map_err(|e| WebError::BrowserError(format!("CDP Handshake failed: {}", e)))
    }

    async fn init_headless_browser(config: &WebConfig) -> Result<Browser, WebError> {
        let mut builder = LaunchOptions::default_builder();

        // Tactical Stealth & Performance Flags
        let browser_args = vec![
            OsString::from("--disable-gpu"),
            OsString::from("--disable-dev-shm-usage"),
            OsString::from("--disable-extensions"),
            OsString::from("--disable-component-extensions-with-background-pages"),
            OsString::from("--disable-default-apps"),
            OsString::from("--mute-audio"),
            OsString::from("--no-pings"),
            OsString::from("--password-store=basic"),
            OsString::from("--use-mock-keychain"),
        ];

        builder
            .headless(true)
            .sandbox(false)
            .args(browser_args.iter().map(|s| s.as_os_str()).collect());

        if let Some(binary_path) = &config.browser_binary {
            builder.path(Some(PathBuf::from(binary_path)));
        }

        let options = builder
            .build()
            .map_err(|e| WebError::BrowserError(e.to_string()))?;
        Browser::new(options)
            .map_err(|e| WebError::BrowserError(format!("Native engine launch failed: {}", e)))
    }

    async fn setup_webdriver(_config: &WebConfig) -> Result<WebDriver, WebError> {
        let mut caps = serde_json::Map::new();
        let mut chrome_options = serde_json::Map::new();
        chrome_options.insert(
            "args".to_string(),
            serde_json::json!([
                "--headless",
                "--disable-gpu",
                "--no-sandbox",
                "--disable-dev-shm-usage"
            ]),
        );
        caps.insert(
            "goog:chromeOptions".to_string(),
            serde_json::Value::Object(chrome_options),
        );

        WdClientBuilder::native()
            .capabilities(caps)
            .connect("http://localhost:9515")
            .await
            .map_err(|e| WebError::BrowserError(e.to_string()))
    }

    async fn load_cookies(jar: &Jar, path: &Path) -> Result<(), WebError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for cookie_line in reader.lines().map_while(Result::ok) {
            if !cookie_line.trim().is_empty() && !cookie_line.starts_with('#') {
                if let Some((url, cookie)) = Self::parse_cookie_line(&cookie_line) {
                    jar.add_cookie_str(&cookie, &url.parse().unwrap());
                }
            }
        }
        Ok(())
    }

    fn parse_cookie_line(line: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 7 {
            let _flag = parts[1];
            let domain = parts[0];
            let path = parts[2];
            let secure = parts[3];
            let _expiration = parts[4];
            let name = parts[5];
            let value = parts[6];

            let cookie_str = format!(
                "{}={}; domain={}; path={}; {}",
                name,
                value,
                domain,
                path,
                if secure == "TRUE" { "secure;" } else { "" }
            );

            Some((format!("http://{}", domain), cookie_str))
        } else {
            None
        }
    }

    pub async fn save_cookies(&self) -> Result<(), WebError> {
        if let Some(cookies_file) = &self.config.cookies_file {
            let path = Path::new(cookies_file);
            let mut file = File::create(path)?;

            // Write cookies in Netscape format
            writeln!(file, "# Netscape HTTP Cookie File")?;
            writeln!(file, "# This file was generated by WebAutomation")?;

            // This is simplified - real implementation would need to extract cookies from jar
            Ok(())
        } else {
            Ok(())
        }
    }

    // ============================================================
    // Core HTTP Methods
    // ============================================================

    pub async fn get(&self, url: &str, session_name: Option<&str>) -> Result<String, WebError> {
        self.request_with_retry("GET", url, None, None, session_name)
            .await
    }

    pub async fn post(
        &self,
        url: &str,
        data: &HashMap<String, String>,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        self.request_with_retry("POST", url, Some(data), None, session_name)
            .await
    }

    pub async fn post_json(
        &self,
        url: &str,
        json: &serde_json::Value,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        let json_str = json.to_string();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        self.request_with_retry("POST", url, None, Some((&json_str, headers)), session_name)
            .await
    }

    pub async fn put(
        &self,
        url: &str,
        data: &HashMap<String, String>,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        self.request_with_retry("PUT", url, Some(data), None, session_name)
            .await
    }

    pub async fn delete(&self, url: &str, session_name: Option<&str>) -> Result<String, WebError> {
        self.request_with_retry("DELETE", url, None, None, session_name)
            .await
    }

    pub async fn head(&self, url: &str, session_name: Option<&str>) -> Result<HeaderMap, WebError> {
        if let Some(limiter) = &self.rate_limiter {
            limiter.acquire().await;
        }

        let mut request_builder = self.client.head(url);

        // Add session headers if available
        if let Some(session_name) = session_name {
            let sessions = self.sessions.lock().await;
            if let Some(session) = sessions.get(session_name) {
                if let Some(cookie_str) = session.cookies.get("session") {
                    request_builder = request_builder.header(COOKIE, cookie_str);
                }
                if let Some(token) = &session.csrf_token {
                    request_builder = request_builder.header("X-CSRF-Token", token);
                }
                if let Some(jwt) = &session.jwt_token {
                    request_builder =
                        request_builder.header(AUTHORIZATION, format!("Bearer {}", jwt));
                }
            }
        }

        let response = request_builder.send().await?;

        if response.status().is_success() {
            Ok(response.headers().clone())
        } else {
            Err(WebError::RequestError(format!(
                "HTTP {}: {}",
                response.status(),
                url
            )))
        }
    }

    async fn request_with_retry(
        &self,
        method: &str,
        url: &str,
        form_data: Option<&HashMap<String, String>>,
        custom_data: Option<(&str, HeaderMap)>,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        let mut last_error = None;

        for attempt in 0..self.config.retry_attempts {
            // Apply rate limiting
            if let Some(limiter) = &self.rate_limiter {
                limiter.acquire().await;
            }

            // Rotate fingerprint if configured
            if self.config.fingerprint_rotation && attempt > 0 {
                self.rotate_fingerprint().await;
            }

            match self
                .execute_request(
                    method,
                    url,
                    form_data,
                    custom_data.as_ref().map(|(s, h)| (*s, h.clone())),
                    session_name,
                )
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response.text().await?);
                    } else if response.status().as_u16() == 429 {
                        // Rate limited - wait longer
                        let wait_time = Duration::from_secs(2u64.pow(attempt as u32) * 5);
                        sleep(wait_time).await;
                        last_error = Some(WebError::RateLimitError(format!(
                            "HTTP {}",
                            response.status()
                        )));
                    } else if response.status().is_server_error() {
                        // Server error - retry
                        let wait_time = Duration::from_millis(
                            self.config.retry_delay_ms * (attempt + 1) as u64,
                        );
                        sleep(wait_time).await;
                        last_error = Some(WebError::RequestError(format!(
                            "HTTP {}",
                            response.status()
                        )));
                    } else {
                        // Client error - don't retry
                        return Err(WebError::RequestError(format!(
                            "HTTP {}: {}",
                            response.status(),
                            url
                        )));
                    }
                }
                Err(e) => {
                    last_error = Some(e);
                    let wait_time =
                        Duration::from_millis(self.config.retry_delay_ms * (attempt + 1) as u64);
                    sleep(wait_time).await;
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| WebError::RequestError("Max retries exceeded".to_string())))
    }

    async fn execute_request(
        &self,
        method: &str,
        url: &str,
        form_data: Option<&HashMap<String, String>>,
        custom_data: Option<(&str, HeaderMap)>,
        session_name: Option<&str>,
    ) -> Result<Response, WebError> {
        let mut request_builder = match method {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            "PATCH" => self.client.patch(url),
            _ => {
                return Err(WebError::RequestError(format!(
                    "Unsupported method: {}",
                    method
                )))
            }
        };

        // Add session headers
        if let Some(session_name) = session_name {
            let sessions = self.sessions.lock().await;
            if let Some(session) = sessions.get(session_name) {
                // Add cookies
                if !session.cookies.is_empty() {
                    let cookie_str = session.to_cookie_header();
                    request_builder = request_builder.header(COOKIE, cookie_str);
                }

                // Add CSRF token
                if let Some(token) = &session.csrf_token {
                    request_builder = request_builder.header("X-CSRF-Token", token);
                }

                // Add JWT token
                if let Some(jwt) = &session.jwt_token {
                    request_builder =
                        request_builder.header(AUTHORIZATION, format!("Bearer {}", jwt));
                }

                // Add other headers
                for (key, value) in session.headers.iter() {
                    request_builder = request_builder.header(key, value);
                }
            }
        }

        // Add form data or custom data
        if let Some(data) = form_data {
            request_builder = request_builder.form(data);
        } else if let Some((body, headers)) = custom_data {
            for (key, value) in headers.iter() {
                request_builder = request_builder.header(key, value);
            }
            request_builder = request_builder.body(body.to_string());
        }

        Ok(request_builder.send().await?)
    }

    async fn rotate_fingerprint(&self) {
        let mut fingerprints = self.fingerprints.lock().await;
        let new_fingerprint = BrowserFingerprint::random();
        fingerprints.push_back(new_fingerprint);
        let _ = fingerprints.pop_front();
    }

    // ============================================================
    // Authentication Methods
    // ============================================================

    pub async fn login_form(&self, request: LoginRequest<'_>) -> Result<AuthSession, WebError> {
        // First, GET the login page to extract CSRF token and form details
        let login_page = self.get(request.url, None).await?;

        // Parse the login form
        let form = WebForm::from_html(&login_page, Some("form"))
            .map_err(|e| WebError::AuthError(format!("Failed to parse login form: {}", e)))?;

        // Extract CSRF token
        let mut form_data = HashMap::new();
        form_data.insert(request.user_field.to_string(), request.username.to_string());
        form_data.insert(request.pass_field.to_string(), request.password.to_string());

        if let Some(additional) = request.additional_data {
            form_data.extend(additional);
        }

        // Add CSRF token if found
        {
            let document = Html::parse_document(&login_page);
            let csrf_selectors = [
                "input[name='csrf_token']",
                "input[name='_csrf']",
                "input[name='csrfmiddlewaretoken']",
                "input[name='authenticity_token']",
            ];

            for selector_str in &csrf_selectors {
                if let Ok(selector) = Selector::parse(selector_str) {
                    if let Some(element) = document.select(&selector).next() {
                        if let Some(token) = element.value().attr("value") {
                            form_data.insert(
                                selector_str
                                    .split("='")
                                    .last()
                                    .unwrap_or("csrf_token")
                                    .to_string(),
                                token.to_string(),
                            );
                            break;
                        }
                    }
                }
            }
        }

        // Submit login form
        let response_text = self.post(&form.action, &form_data, None).await?;

        // Create session
        let mut session = AuthSession::new(request.url.to_string(), request.username.to_string());

        // Extract session cookies from response
        if let Some(set_cookie) = response_text.lines().find(|l| l.contains("Set-Cookie")) {
            // Parse cookies - simplified
            let cookie_parts: Vec<&str> = set_cookie.split(';').collect();
            for part in cookie_parts {
                if part.contains('=') {
                    let kv: Vec<&str> = part.splitn(2, '=').collect();
                    if kv.len() == 2 {
                        session.add_cookie(kv[0].trim().to_string(), kv[1].to_string());
                    }
                }
            }
        }

        // Extract CSRF token from response
        session.extract_csrf(&response_text);

        // Extract JWT if present
        session.extract_jwt(&response_text);

        // Store session
        {
            let mut sessions = self.sessions.lock().await;
            let key = request.session_name.unwrap_or(request.username);
            sessions.insert(key.to_string(), session.clone());
        }

        Ok(session)
    }

    pub async fn login_basic_auth(
        &self,
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<AuthSession, WebError> {
        let auth_value = format!("{}:{}", username, password);
        let encoded = BASE64.encode(auth_value.as_bytes());

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded))
                .map_err(|e| WebError::AuthError(e.to_string()))?,
        );

        let response = self.client.get(url).headers(headers).send().await?;

        if response.status().is_success() {
            let mut session = AuthSession::new(url.to_string(), username.to_string());

            // Extract any cookies
            for cookie in response.cookies() {
                session.add_cookie(cookie.name().to_string(), cookie.value().to_string());
            }

            // Store session
            let mut sessions = self.sessions.lock().await;
            sessions.insert(username.to_string(), session.clone());

            Ok(session)
        } else {
            Err(WebError::AuthError(format!(
                "Basic auth failed: {}",
                response.status()
            )))
        }
    }

    pub async fn login_oauth2(
        &self,
        auth_url: &str,
        token_url: &str,
        client_id: &str,
        client_secret: &str,
        username: &str,
        password: &str,
    ) -> Result<AuthSession, WebError> {
        // This is a simplified OAuth2 flow - real implementation would need to handle redirects
        let mut form_data = HashMap::new();
        form_data.insert("grant_type".to_string(), "password".to_string());
        form_data.insert("client_id".to_string(), client_id.to_string());
        form_data.insert("client_secret".to_string(), client_secret.to_string());
        form_data.insert("username".to_string(), username.to_string());
        form_data.insert("password".to_string(), password.to_string());

        let response_text = self.post(token_url, &form_data, None).await?;

        // Parse JSON response
        let json: serde_json::Value = serde_json::from_str(&response_text)?;

        let mut session = AuthSession::new(auth_url.to_string(), username.to_string());

        // Extract tokens
        if let Some(access_token) = json.get("access_token").and_then(|v| v.as_str()) {
            session.session_token = Some(access_token.to_string());
            session.jwt_token = Some(access_token.to_string());
        }

        if let Some(refresh_token) = json.get("refresh_token").and_then(|v| v.as_str()) {
            session.add_cookie("refresh_token".to_string(), refresh_token.to_string());
        }

        // Store session
        let mut sessions = self.sessions.lock().await;
        sessions.insert(username.to_string(), session.clone());

        Ok(session)
    }

    pub async fn logout(&self, session_name: &str) -> Result<(), WebError> {
        let mut sessions = self.sessions.lock().await;

        if let Some(session) = sessions.remove(session_name) {
            // Try to hit logout endpoint if exists
            let logout_url = format!("{}/logout", session.login_url.trim_end_matches('/'));
            let _ = self.get(&logout_url, Some(session_name)).await;
        }

        Ok(())
    }

    // ============================================================
    // Form Handling
    // ============================================================

    pub async fn fill_and_submit_form(
        &self,
        url: &str,
        form_selector: Option<&str>,
        field_data: &HashMap<String, String>,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        // Get the page
        let page_content = self.get(url, session_name).await?;

        // Parse form
        let mut form = WebForm::from_html(&page_content, form_selector)?;

        // Extract CSRF token
        let sessions = self.sessions.lock().await;
        if let Some(session_name) = session_name {
            if let Some(session) = sessions.get(session_name) {
                form.csrf_token = session.csrf_token.clone();
            }
        }

        // Build form data
        let mut form_data = HashMap::new();
        for (name, value) in field_data {
            form_data.insert(name.clone(), value.clone());
        }

        // Prepare and submit form
        let response = match form.method.as_str() {
            "GET" => {
                let mut url_parts: Vec<String> = Vec::new();
                for (key, value) in &form_data {
                    url_parts.push(format!("{}={}", key, urlencoding::encode(value)));
                }
                let query_string = url_parts.join("&");
                let full_url = if form.action.contains('?') {
                    format!("{}&{}", form.action, query_string)
                } else {
                    format!("{}?{}", form.action, query_string)
                };
                self.get(&full_url, session_name).await?
            }
            "POST" => {
                if form.enctype.contains("multipart/form-data") {
                    let multipart = form.to_multipart(&form_data)?;
                    let response = self
                        .client
                        .post(&form.action)
                        .multipart(multipart)
                        .send()
                        .await?;
                    response.text().await?
                } else {
                    self.post(&form.action, &form_data, session_name).await?
                }
            }
            _ => {
                return Err(WebError::ParseError(format!(
                    "Unsupported form method: {}",
                    form.method
                )))
            }
        };

        Ok(response)
    }

    pub async fn upload_file(
        &self,
        url: &str,
        file_field: &str,
        file_path: &str,
        additional_data: Option<HashMap<String, String>>,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        let path = Path::new(file_path);
        let file_name = path
            .file_name()
            .ok_or_else(|| {
                WebError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Invalid file path",
                ))
            })?
            .to_str()
            .unwrap_or("file");

        let file_content = fs::read(path)?;

        let mut form = Form::new();

        // Add file part
        let file_part = Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("application/octet-stream")?;
        form = form.part(file_field.to_string(), file_part);

        // Add additional data
        if let Some(data) = additional_data {
            for (key, value) in data {
                form = form.text(key, value);
            }
        }

        // Add session cookies if available
        let mut request_builder = self.client.post(url).multipart(form);

        if let Some(session_name) = session_name {
            let sessions = self.sessions.lock().await;
            if let Some(session) = sessions.get(session_name) {
                if !session.cookies.is_empty() {
                    let cookie_str = session.to_cookie_header();
                    request_builder = request_builder.header(COOKIE, cookie_str);
                }
            }
        }

        let response = request_builder.send().await?;

        Ok(response.text().await?)
    }

    // ============================================================
    // Scraping & Information Gathering
    // ============================================================

    pub async fn scrape_links(
        &self,
        url: &str,
        pattern: Option<&str>,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let content = self.get(url, session_name).await?;
        let document = Html::parse_document(&content);

        let selector = Selector::parse("a").map_err(|e| WebError::ParseError(e.to_string()))?;
        let mut links = Vec::new();

        let url_regex = if let Some(p) = pattern {
            Some(Regex::new(p).map_err(|e| WebError::ParseError(e.to_string()))?)
        } else {
            None
        };

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                // Resolve relative URLs
                let base = Url::parse(url).map_err(|e| WebError::ParseError(e.to_string()))?;
                if let Ok(full_url) = base.join(href) {
                    let url_str = full_url.to_string();

                    // Filter by regex pattern if provided
                    if let Some(regex) = &url_regex {
                        if regex.is_match(&url_str) {
                            links.push(url_str);
                        }
                    } else {
                        links.push(url_str);
                    }
                }
            }
        }

        Ok(links)
    }

    pub async fn scrape_images(
        &self,
        url: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let content = self.get(url, session_name).await?;
        let document = Html::parse_document(&content);

        let selector = Selector::parse("img").map_err(|e| WebError::ParseError(e.to_string()))?;
        let mut images = Vec::new();

        let base = Url::parse(url).map_err(|e| WebError::ParseError(e.to_string()))?;

        for element in document.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(full_url) = base.join(src) {
                    images.push(full_url.to_string());
                }
            }
        }

        Ok(images)
    }

    pub async fn scrape_scripts(
        &self,
        url: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let content = self.get(url, session_name).await?;
        let document = Html::parse_document(&content);

        let selector =
            Selector::parse("script[src]").map_err(|e| WebError::ParseError(e.to_string()))?;
        let mut scripts = Vec::new();

        let base = Url::parse(url).map_err(|e| WebError::ParseError(e.to_string()))?;

        for element in document.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(full_url) = base.join(src) {
                    scripts.push(full_url.to_string());
                }
            }
        }

        Ok(scripts)
    }

    pub async fn scrape_forms(
        &self,
        url: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<WebForm>, WebError> {
        let content = self.get(url, session_name).await?;
        let document = Html::parse_document(&content);

        let selector = Selector::parse("form").map_err(|e| WebError::ParseError(e.to_string()))?;
        let mut forms = Vec::new();

        let base = Url::parse(url).map_err(|e| WebError::ParseError(e.to_string()))?;

        for form_element in document.select(&selector) {
            let action = form_element
                .value()
                .attr("action")
                .unwrap_or("")
                .to_string();
            let method = form_element
                .value()
                .attr("method")
                .unwrap_or("get")
                .to_string();
            let enctype = form_element
                .value()
                .attr("enctype")
                .unwrap_or("application/x-www-form-urlencoded")
                .to_string();

            // Resolve action URL
            let full_action = if action.is_empty() {
                url.to_string()
            } else {
                base.join(&action)
                    .map(|u| u.to_string())
                    .unwrap_or_else(|_| action)
            };

            let mut fields = Vec::new();
            let input_selector = Selector::parse("input, select, textarea").unwrap();

            for input_element in form_element.select(&input_selector) {
                if let Some(name) = input_element.value().attr("name") {
                    let field_type = input_element
                        .value()
                        .attr("type")
                        .unwrap_or("text")
                        .to_string();
                    let value = input_element
                        .value()
                        .attr("value")
                        .unwrap_or("")
                        .to_string();
                    let required = input_element.value().attr("required").is_some();

                    fields.push(FormField {
                        name: name.to_string(),
                        value,
                        field_type,
                        required,
                        options: Vec::new(),
                        max_length: None,
                        min_length: None,
                        pattern: None,
                    });
                }
            }

            forms.push(WebForm {
                action: full_action,
                method: method.to_uppercase(),
                enctype,
                fields,
                csrf_token: None,
            });
        }

        Ok(forms)
    }

    pub async fn scrape_emails(
        &self,
        url: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let content = self.get(url, session_name).await?;
        let email_regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
            .map_err(|e| WebError::ParseError(e.to_string()))?;

        let mut emails = Vec::new();
        for cap in email_regex.find_iter(&content) {
            emails.push(cap.as_str().to_string());
        }

        Ok(emails)
    }

    pub async fn scrape_phone_numbers(
        &self,
        url: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let content = self.get(url, session_name).await?;
        // Match various phone number formats
        let phone_regex = Regex::new(r"(\+?\d{1,3}[-.\s]?)?\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}")
            .map_err(|e| WebError::ParseError(e.to_string()))?;

        let mut phones = Vec::new();
        for cap in phone_regex.find_iter(&content) {
            phones.push(cap.as_str().to_string());
        }

        Ok(phones)
    }

    pub async fn scrape_with_selector(
        &self,
        url: &str,
        selector: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let content = self.get(url, session_name).await?;
        let document = Html::parse_document(&content);

        let sel = Selector::parse(selector).map_err(|e| WebError::ParseError(e.to_string()))?;
        let mut results = Vec::new();

        for element in document.select(&sel) {
            results.push(element.text().collect::<Vec<_>>().join(" "));
        }

        Ok(results)
    }

    // ============================================================
    // Download Methods
    // ============================================================

    pub async fn download_file(
        &self,
        url: &str,
        output_path: &str,
        session_name: Option<&str>,
    ) -> Result<(), WebError> {
        if let Some(limiter) = &self.rate_limiter {
            limiter.acquire().await;
        }

        let mut request_builder = self.client.get(url);

        if let Some(session_name) = session_name {
            let sessions = self.sessions.lock().await;
            if let Some(session) = sessions.get(session_name) {
                if !session.cookies.is_empty() {
                    let cookie_str = session.to_cookie_header();
                    request_builder = request_builder.header(COOKIE, cookie_str);
                }
            }
        }

        let response = request_builder.send().await?;

        if response.status().is_success() {
            let bytes = response.bytes().await?;
            let generator = crate::builtin_tools::utilities::file_generation::FileGenerator::new(
                PathBuf::from("."),
                self.config.all_report_path.as_ref().map(PathBuf::from),
                crate::builtin_tools::utilities::file_generation::FileGenerationConfig::default(),
            );
            generator
                .generate_file(output_path, Some(&bytes))
                .await
                .map_err(|e| WebError::IoError(std::io::Error::other(e.to_string())))?;
            Ok(())
        } else {
            Err(WebError::RequestError(format!(
                "Download failed: HTTP {}",
                response.status()
            )))
        }
    }

    pub async fn download_multiple(
        &self,
        urls: Vec<String>,
        output_dir: &str,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let mut tasks = Vec::new();
        let client = self.client.clone();
        let _output_dir_path = PathBuf::from(output_dir);

        for (i, url) in urls.into_iter().enumerate() {
            let client = client.clone();
            let output_path = format!("{}/file_{}", output_dir, i);
            let _session_name = session_name.map(String::from);

            let task = tokio::spawn(async move {
                // Simplified download without rate limiting per task
                let response = client.get(&url).send().await?;
                if response.status().is_success() {
                    let bytes = response.bytes().await?;
                    let generator = crate::builtin_tools::utilities::file_generation::FileGenerator::new(
                        PathBuf::from("."),
                        None, // Async block might not have access to self if not handled carefully, but let's check. Wait, download_multiple uses self.
                        crate::builtin_tools::utilities::file_generation::FileGenerationConfig::default()
                    );
                    generator
                        .generate_file(&output_path, Some(&bytes))
                        .await
                        .map_err(|e| WebError::IoError(std::io::Error::other(e.to_string())))?;
                    Ok::<String, WebError>(output_path)
                } else {
                    Err(WebError::RequestError(format!(
                        "Download failed: HTTP {}",
                        response.status()
                    )))
                }
            });

            tasks.push(task);
        }

        let results = join_all(tasks).await;
        let mut downloaded = Vec::new();

        for result in results {
            match result {
                Ok(Ok(path)) => downloaded.push(path),
                Ok(Err(e)) => eprintln!("Download error: {}", e),
                Err(e) => eprintln!("Task error: {}", e),
            }
        }

        Ok(downloaded)
    }

    // ============================================================
    // JavaScript Execution & Browser Automation
    // ============================================================

    pub async fn execute_js(
        &self,
        url: &str,
        js_code: &str,
        _session_name: Option<&str>,
    ) -> Result<String, WebError> {
        let engine_name = if self.is_lightpanda {
            "Lightpanda [Elite]"
        } else {
            "Chromium [Native]"
        };
        tracing::debug!("Executing JavaScript on {} engine...", engine_name);

        if let Some(browser) = &self.browser {
            let tab = browser
                .new_tab()
                .map_err(|e| WebError::BrowserError(e.to_string()))?;

            // Navigate to URL
            tab.navigate_to(url)
                .map_err(|e| WebError::BrowserError(e.to_string()))?;

            // Wait for page load
            tab.wait_until_navigated()
                .map_err(|e| WebError::BrowserError(e.to_string()))?;

            // Execute JavaScript
            let result = tab
                .evaluate(js_code, true)
                .map_err(|e| WebError::JavascriptError(e.to_string()))?;

            Ok(format!("{:?}", result))
        } else if let Some(client) = &self.webdriver_client {
            client
                .goto(url)
                .await
                .map_err(|e: fantoccini::error::CmdError| WebError::BrowserError(e.to_string()))?;

            let result = client.execute(js_code, Vec::new()).await.map_err(
                |e: fantoccini::error::CmdError| WebError::JavascriptError(e.to_string()),
            )?;

            Ok(format!("{:?}", result))
        } else {
            Err(WebError::JavascriptError(
                "No browser available for JavaScript execution".to_string(),
            ))
        }
    }

    pub async fn screenshot(
        &self,
        url: &str,
        output_path: &str,
        full_page: bool,
    ) -> Result<(), WebError> {
        let engine_name = if self.is_lightpanda {
            "Lightpanda [Elite]"
        } else {
            "Chromium [Native]"
        };
        tracing::debug!(
            "Capturing {} screenshot on {} engine...",
            if full_page { "full-page" } else { "viewport" },
            engine_name
        );

        if let Some(browser) = &self.browser {
            let tab = browser
                .new_tab()
                .map_err(|e| WebError::BrowserError(e.to_string()))?;
            tab.navigate_to(url)
                .map_err(|e| WebError::BrowserError(e.to_string()))?;
            tab.wait_until_navigated()
                .map_err(|e| WebError::BrowserError(e.to_string()))?;

            let (width, height) = match tab.evaluate("JSON.stringify({width: Math.max(document.documentElement.scrollWidth, document.body.scrollWidth), height: Math.max(document.documentElement.scrollHeight, document.body.scrollHeight)})", false) {
                Ok(remote_obj) => {
                    // This is a bit hacky but RemoteObject in 1.0.21 is hard to parse directly without cdp types
                    let _raw = format!("{:?}", remote_obj);
                    // Extract width and height from debug string or just use defaults
                    (1920.0, 1080.0) // Defaulting for now to avoid complex parsing of debug string
                },
                Err(_) => (1920.0, 1080.0),
            };

            let png_data = if full_page {
                tab.capture_screenshot(
                    Page::CaptureScreenshotFormatOption::Png,
                    None,
                    Some(Page::Viewport {
                        x: 0.0,
                        y: 0.0,
                        width,
                        height,
                        scale: 1.0,
                    }),
                    true,
                )
                .map_err(|e| WebError::BrowserError(e.to_string()))?
            } else {
                tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)
                    .map_err(|e| WebError::BrowserError(e.to_string()))?
            };

            let generator = crate::builtin_tools::utilities::file_generation::FileGenerator::new(
                PathBuf::from("."),
                self.config.all_report_path.as_ref().map(PathBuf::from),
                crate::builtin_tools::utilities::file_generation::FileGenerationConfig::default(),
            );
            generator
                .generate_file(output_path, Some(&png_data))
                .await
                .map_err(|e| WebError::IoError(std::io::Error::other(e.to_string())))?;
            Ok(())
        } else {
            Err(WebError::BrowserError(
                "No browser available for screenshot".to_string(),
            ))
        }
    }

    pub async fn click_element(
        &self,
        selector: &str,
        _session_name: Option<&str>,
    ) -> Result<(), WebError> {
        if let Some(client) = &self.webdriver_client {
            let element = client
                .find(Locator::Css(selector))
                .await
                .map_err(|e: fantoccini::error::CmdError| WebError::BrowserError(e.to_string()))?;
            element
                .click()
                .await
                .map_err(|e: fantoccini::error::CmdError| WebError::BrowserError(e.to_string()))?;
            Ok(())
        } else {
            Err(WebError::BrowserError(
                "No WebDriver client available".to_string(),
            ))
        }
    }

    pub async fn type_text(
        &self,
        selector: &str,
        text: &str,
        _session_name: Option<&str>,
    ) -> Result<(), WebError> {
        if let Some(client) = &self.webdriver_client {
            let element = client
                .find(Locator::Css(selector))
                .await
                .map_err(|e: fantoccini::error::CmdError| WebError::BrowserError(e.to_string()))?;
            element
                .send_keys(text)
                .await
                .map_err(|e: fantoccini::error::CmdError| WebError::BrowserError(e.to_string()))?;
            Ok(())
        } else {
            Err(WebError::BrowserError(
                "No WebDriver client available".to_string(),
            ))
        }
    }

    pub async fn wait_for_selector(
        &self,
        selector: &str,
        timeout_secs: u64,
    ) -> Result<(), WebError> {
        let timeout = Duration::from_secs(timeout_secs);
        if let Some(client) = &self.webdriver_client {
            client
                .wait()
                .at_most(timeout)
                .for_element(Locator::Css(selector))
                .await
                .map_err(|e| {
                    WebError::TimeoutError(format!("Timed out waiting for {}: {}", selector, e))
                })?;
            Ok(())
        } else if let Some(browser) = &self.browser {
            let tab = browser
                .new_tab()
                .map_err(|e| WebError::BrowserError(e.to_string()))?;
            // We'll use a manual loop or check if headless_chrome has a timeout option.
            // For now, we utilize the tab's navigation/capture timeouts if applicable, or simple loop.
            // Tab::wait_for_element in headless_chrome 1.0.x is blocking and doesn't take timeout easily.
            // We'll prefix if we can't easily set it to avoid the warning, or implement a future timeout.
            tab.wait_for_element(selector).map_err(|e| {
                WebError::TimeoutError(format!("Timed out waiting for {}: {}", selector, e))
            })?;
            Ok(())
        } else {
            Err(WebError::BrowserError(
                "No automation client available for wait_for".to_string(),
            ))
        }
    }

    // ============================================================
    // Advanced Bypass Techniques
    // ============================================================

    pub async fn bypass_captcha(
        &self,
        captcha_image_url: &str,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        if !self.config.captcha_services.auto_solve {
            return Err(WebError::CaptchaError(
                "Captcha solving not enabled".to_string(),
            ));
        }

        // Download captcha image
        let captcha_path = format!(
            "{}/captcha_{}.png",
            self.config.storage_dir,
            chrono::Utc::now().timestamp()
        );
        self.download_file(captcha_image_url, &captcha_path, session_name)
            .await?;

        // Try different captcha services
        if let Some(api_key) = &self.config.captcha_services.two_captcha_api_key {
            return self.solve_with_2captcha(&captcha_path, api_key).await;
        }

        if let Some(api_key) = &self.config.captcha_services.anti_captcha_api_key {
            return self.solve_with_anticaptcha(&captcha_path, api_key).await;
        }

        if let Some(api_key) = &self.config.captcha_services.capsolver_api_key {
            return self.solve_with_capsolver(&captcha_path, api_key).await;
        }

        Err(WebError::CaptchaError(
            "No captcha service configured".to_string(),
        ))
    }

    async fn solve_with_2captcha(
        &self,
        image_path: &str,
        api_key: &str,
    ) -> Result<String, WebError> {
        // Implementation for 2Captcha API
        let image_data = fs::read(image_path)?;
        let base64_image = BASE64.encode(&image_data);

        let mut form_data = HashMap::new();
        form_data.insert("key".to_string(), api_key.to_string());
        form_data.insert("method".to_string(), "base64".to_string());
        form_data.insert("body".to_string(), base64_image);

        // Submit to 2captcha
        let submit_url = "http://2captcha.com/in.php";
        let submit_response = self.post(submit_url, &form_data, None).await?;

        if submit_response.contains("OK|") {
            let captcha_id = submit_response.split('|').nth(1).unwrap_or("");

            // Poll for result
            let poll_url = format!(
                "http://2captcha.com/res.php?key={}&action=get&id={}",
                api_key, captcha_id
            );

            let start = Instant::now();
            while start.elapsed()
                < Duration::from_secs(self.config.captcha_services.solve_timeout_seconds)
            {
                sleep(Duration::from_secs(5)).await;

                let poll_response = self.get(&poll_url, None).await?;

                if poll_response.contains("OK|") {
                    return Ok(poll_response.split('|').nth(1).unwrap_or("").to_string());
                } else if poll_response.contains("CAPCHA_NOT_READY") {
                    continue;
                } else {
                    break;
                }
            }
        }

        Err(WebError::CaptchaError(
            "Failed to solve captcha with 2Captcha".to_string(),
        ))
    }

    async fn solve_with_anticaptcha(
        &self,
        _image_path: &str,
        _api_key: &str,
    ) -> Result<String, WebError> {
        // AntiCaptcha implementation
        Err(WebError::CaptchaError(
            "AntiCaptcha not fully implemented".to_string(),
        ))
    }

    async fn solve_with_capsolver(
        &self,
        _image_path: &str,
        _api_key: &str,
    ) -> Result<String, WebError> {
        // Capsolver implementation
        Err(WebError::CaptchaError(
            "Capsolver not fully implemented".to_string(),
        ))
    }

    pub async fn bypass_waf(
        &self,
        url: &str,
        session_name: Option<&str>,
    ) -> Result<String, WebError> {
        // Implement WAF bypass techniques
        let mut headers = HeaderMap::new();

        // Add random headers to avoid fingerprinting
        headers.insert(
            "X-Forwarded-For",
            HeaderValue::from_str(&format!(
                "{}.{}.{}.{}",
                rand::random_range(1..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(1..255)
            ))
            .unwrap(),
        );

        headers.insert(
            "X-Real-IP",
            HeaderValue::from_str(&format!(
                "{}.{}.{}.{}",
                rand::random_range(1..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(1..255)
            ))
            .unwrap(),
        );

        headers.insert(
            "X-Originating-IP",
            HeaderValue::from_str(&format!(
                "{}.{}.{}.{}",
                rand::random_range(1..255),
                rand::random_range(0..255),
                rand::random_range(0..255),
                rand::random_range(1..255)
            ))
            .unwrap(),
        );

        // Use stealth mode headers
        if self.config.stealth_mode {
            headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
            headers.insert("Pragma", HeaderValue::from_static("no-cache"));
        }

        let mut request_builder = self.client.get(url).headers(headers);

        if let Some(session_name) = session_name {
            let sessions = self.sessions.lock().await;
            if let Some(session) = sessions.get(session_name) {
                if !session.cookies.is_empty() {
                    let cookie_str = session.to_cookie_header();
                    request_builder = request_builder.header(COOKIE, cookie_str);
                }
            }
        }

        let response = request_builder.send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(WebError::RequestError(format!(
                "WAF bypass failed: HTTP {}",
                response.status()
            )))
        }
    }

    pub async fn bypass_rate_limit(
        &self,
        url: &str,
        requests: usize,
        session_name: Option<&str>,
    ) -> Result<Vec<String>, WebError> {
        let mut results = Vec::new();

        // Use rotating proxies if available
        let proxy_count = self.config.proxy_list.len();

        for i in 0..requests {
            let current_proxy = if proxy_count > 0 {
                let proxy_url = &self.config.proxy_list[i % proxy_count];
                Some(
                    Proxy::all(proxy_url)
                        .map_err(|e| WebError::ConfigError(format!("Invalid proxy: {}", e)))?,
                )
            } else {
                None
            };

            // Build client with proxy
            let mut client_builder =
                ClientBuilder::new().timeout(Duration::from_secs(self.config.timeout_seconds));

            if let Some(proxy) = current_proxy {
                client_builder = client_builder.proxy(proxy);
            }

            // Rotate user agent
            let fingerprint = BrowserFingerprint::random();
            client_builder = client_builder.default_headers(fingerprint.to_headers());

            let client = client_builder.build()?;

            // Make request
            let mut request_builder = client.get(url);

            if let Some(session_name) = session_name {
                let sessions = self.sessions.lock().await;
                if let Some(session) = sessions.get(session_name) {
                    if !session.cookies.is_empty() {
                        let cookie_str = session.to_cookie_header();
                        request_builder = request_builder.header(COOKIE, cookie_str);
                    }
                }
            }

            match request_builder.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(text) = response.text().await {
                            results.push(text);
                        }
                    }
                }
                Err(e) => eprintln!("Request {} failed: {}", i, e),
            }

            // Small delay between requests
            sleep(Duration::from_millis(100)).await;
        }

        Ok(results)
    }

    // ============================================================
    // Session Management
    // ============================================================

    pub async fn get_session(&self, session_name: &str) -> Option<AuthSession> {
        let sessions = self.sessions.lock().await;
        sessions.get(session_name).cloned()
    }

    pub async fn list_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.lock().await;
        sessions.keys().cloned().collect()
    }

    pub async fn remove_session(&self, session_name: &str) {
        let mut sessions = self.sessions.lock().await;
        sessions.remove(session_name);
    }

    pub async fn clear_sessions(&self) {
        let mut sessions = self.sessions.lock().await;
        sessions.clear();
    }

    pub async fn refresh_session(&self, session_name: &str) -> Result<(), WebError> {
        let mut sessions = self.sessions.lock().await;

        if let Some(session) = sessions.get_mut(session_name) {
            // Try to refresh session by visiting login page again
            if let Ok(content) = self.get(&session.login_url, Some(session_name)).await {
                session.extract_csrf(&content);
                session.extract_jwt(&content);
                session.last_used = Utc::now();
                Ok(())
            } else {
                Err(WebError::AuthError("Failed to refresh session".to_string()))
            }
        } else {
            Err(WebError::AuthError("Session not found".to_string()))
        }
    }
}

// ============================================================
// Utility Functions
// ============================================================

pub fn generate_random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn extract_metadata(html: &str) -> HashMap<String, String> {
    let document = Html::parse_document(html);
    let mut metadata = HashMap::new();

    // Extract title
    if let Ok(title_selector) = Selector::parse("title") {
        if let Some(title) = document.select(&title_selector).next() {
            metadata.insert("title".to_string(), title.text().collect());
        }
    }

    // Extract meta tags
    if let Ok(meta_selector) = Selector::parse("meta") {
        for meta in document.select(&meta_selector) {
            if let Some(name) = meta.value().attr("name") {
                if let Some(content) = meta.value().attr("content") {
                    metadata.insert(name.to_string(), content.to_string());
                }
            }
            if let Some(property) = meta.value().attr("property") {
                if let Some(content) = meta.value().attr("content") {
                    metadata.insert(property.to_string(), content.to_string());
                }
            }
        }
    }

    metadata
}

pub fn detect_technology(html: &str, headers: &HeaderMap) -> Vec<String> {
    let mut technologies = Vec::new();

    // Check headers for server info
    if let Some(server) = headers.get("server") {
        if let Ok(server_str) = server.to_str() {
            technologies.push(format!("Server: {}", server_str));
        }
    }

    if let Some(powered_by) = headers.get("x-powered-by") {
        if let Ok(pb_str) = powered_by.to_str() {
            technologies.push(format!("X-Powered-By: {}", pb_str));
        }
    }

    // Check for common frameworks in HTML
    let patterns = [
        (r"wp-content", "WordPress"),
        (r"drupal", "Drupal"),
        (r"Joomla", "Joomla"),
        (r"csrf-token", "CSRF Protection"),
        (r"data-react", "React"),
        (r"ng-", "Angular"),
        (r"vue-", "Vue.js"),
        (r"jquery", "jQuery"),
        (r"bootstrap", "Bootstrap"),
        (r"laravel", "Laravel"),
        (r"symfony", "Symfony"),
        (r"django", "Django"),
        (r"rails", "Ruby on Rails"),
        (r"asp.net", "ASP.NET"),
    ];

    for (pattern, tech) in &patterns {
        if html.contains(pattern) {
            technologies.push(tech.to_string());
        }
    }

    technologies
}
