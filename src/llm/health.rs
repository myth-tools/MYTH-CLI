use crate::config::AppConfig;
use owo_colors::OwoColorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NimModelList {
    data: Vec<NimModel>,
}

#[derive(Debug, Deserialize)]
struct NimModel {
    id: String,
}

pub enum HealthStatus {
    Ok,
    NoKeys,
    InvalidKey(String),
    ModelNotFound(String),
    ConnectionError(String),
}

pub async fn perform_health_check(config: &AppConfig) -> HealthStatus {
    let api_keys = match config.llm.resolve_api_keys() {
        Ok(keys) => keys,
        Err(_) => return HealthStatus::NoKeys,
    };

    if api_keys.is_empty() {
        return HealthStatus::NoKeys;
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    let base_url = config.llm.base_url.trim_end_matches('/');

    // Check first key to verify endpoint and model availability
    let first_key = &api_keys[0];
    let models_url = format!("{}/models", base_url);

    match client
        .get(&models_url)
        .header("Authorization", format!("Bearer {}", first_key))
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().as_u16() == 401 {
                return HealthStatus::InvalidKey(first_key.clone());
            }
            if !resp.status().is_success() {
                return HealthStatus::ConnectionError(format!(
                    "NIM returned HTTP {}",
                    resp.status()
                ));
            }

            // Check if model exists in catalog
            match resp.json::<NimModelList>().await {
                Ok(list) => {
                    let target_model = &config.llm.model;
                    if list.data.iter().any(|m| &m.id == target_model) {
                        HealthStatus::Ok
                    } else {
                        HealthStatus::ModelNotFound(target_model.clone())
                    }
                }
                Err(e) => {
                    HealthStatus::ConnectionError(format!("Failed to parse NIM catalog: {}", e))
                }
            }
        }
        Err(e) => HealthStatus::ConnectionError(e.to_string()),
    }
}

pub fn display_health_report(status: &HealthStatus) {
    match status {
        HealthStatus::Ok => {
            // Usually quiet on success or handled by boot sequence
        }
        HealthStatus::NoKeys => {
            println!(
                "\n{}",
                " ✗ NEURAL LINK FAILURE: MISSING API KEYS "
                    .on_red()
                    .white()
                    .bold()
            );
            println!(
                "{}",
                "No NVIDIA NIM API keys detected in environment or user.yaml.".yellow()
            );
            println!(
                "Fix: Export {} or add it to ~/.config/myth/user.yaml",
                "NVIDIA_API_KEY".cyan().bold()
            );
        }
        HealthStatus::InvalidKey(key) => {
            let masked = if key.len() > 10 {
                format!("{}...{}", &key[0..4], &key[key.len() - 4..])
            } else {
                "****".to_string()
            };
            println!(
                "\n{}",
                " ✗ NEURAL LINK FAILURE: INVALID CREDENTIALS "
                    .on_red()
                    .white()
                    .bold()
            );
            println!(
                "The API key {} was rejected by the NIM relay (401 Unauthorized).",
                masked.cyan().bold()
            );
            println!(
                "{}",
                "Verify your key at https://build.nvidia.com/".yellow()
            );
        }
        HealthStatus::ModelNotFound(model) => {
            println!(
                "\n{}",
                " ✗ NEURAL LINK FAILURE: MODEL NOT FOUND "
                    .on_red()
                    .white()
                    .bold()
            );
            println!(
                "The model {} is not available on your configured NIM endpoint.",
                model.cyan().bold()
            );
            println!(
                "{}",
                "Check your base_url and model identifier in agent.yaml".yellow()
            );
        }
        HealthStatus::ConnectionError(e) => {
            println!(
                "\n{}",
                " ✗ NEURAL LINK FAILURE: CONNECTION ERROR "
                    .on_red()
                    .white()
                    .bold()
            );
            println!("Failed to synchronize with the NIM relay: {}", e.red());
            println!(
                "{}",
                "Check your internet connection or base_url configuration.".yellow()
            );
        }
    }
}
