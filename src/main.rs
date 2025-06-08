use anyhow::{Context, Result};
use clap::{Arg, Command};
use config::Config;
use dirs::config_dir;
use md5;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct TranslationRequest {
    appid: String,
    q: String,
    from: String,
    to: String,
    salt: u32,
    sign: String,
}

#[derive(Debug, Deserialize)]
struct TranslationResponse {
    from: Option<String>,
    to: Option<String>,
    trans_result: Option<Vec<TransResult>>,
    error_code: Option<String>,
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TransResult {
    src: String,
    dst: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    appid: String,
    appkey: String,
}

fn make_md5(s: &str) -> String {
    format!("{:x}", md5::compute(s.as_bytes()))
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = config_dir().context("Could not find config directory")?;
    Ok(config_dir.join("trans-cli").join("config.toml"))
}

fn load_config() -> Result<AppConfig> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        create_default_config(&config_path)?;
        anyhow::bail!(
            "Config file created at {:?}. Please edit it with your Baidu API credentials.",
            config_path
        );
    }

    let settings = Config::builder()
        .add_source(config::File::from(config_path))
        .build()
        .context("Failed to read config file")?;

    let config: AppConfig = settings
        .try_deserialize()
        .context("Failed to parse config file")?;

    if config.appid == "INPUT_YOUR_APPID" || config.appkey == "INPUT_YOUR_APPKEY" {
        anyhow::bail!("Please configure your Baidu API credentials in the config file");
    }

    Ok(config)
}

fn create_default_config(config_path: &PathBuf) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    let default_config = r#"# Baidu Translation API Configuration
# Get your credentials from: https://fanyi-api.baidu.com/
appid = "INPUT_YOUR_APPID"
appkey = "INPUT_YOUR_APPKEY"
"#;

    fs::write(config_path, default_config).context("Failed to write config file")?;
    Ok(())
}

async fn translate_text(
    client: &Client,
    config: &AppConfig,
    query: &str,
    from_lang: &str,
    to_lang: &str,
) -> Result<TranslationResponse> {
    let endpoint = "http://api.fanyi.baidu.com";
    let path = "/api/trans/vip/translate";
    let url = format!("{}{}", endpoint, path);

    // Generate salt and sign
    let salt: u32 = rand::thread_rng().gen_range(32768..65536);
    let sign_str = format!("{}{}{}{}", config.appid, query, salt, config.appkey);
    let sign = make_md5(&sign_str);

    let mut params = HashMap::new();
    params.insert("appid", config.appid.clone());
    params.insert("q", query.to_string());
    params.insert("from", from_lang.to_string());
    params.insert("to", to_lang.to_string());
    params.insert("salt", salt.to_string());
    params.insert("sign", sign);

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .context("Failed to send request")?;

    let result: TranslationResponse = response
        .json()
        .await
        .context("Failed to parse response")?;

    Ok(result)
}

fn print_translation_result(result: &TranslationResponse) -> Result<()> {
    if let Some(error_code) = &result.error_code {
        anyhow::bail!("Translation error {}: {:?}", error_code, result.error_msg);
    }

    if let Some(trans_results) = &result.trans_result {
        for trans in trans_results {
            println!("{}", trans.dst);
        }
    } else {
        anyhow::bail!("No translation result received");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("trans-cli")
        .version("0.1.0")
        .about("A CLI tool for text translation using Baidu API")
        .arg(
            Arg::new("text")
                .help("Text to translate")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("from")
                .short('f')
                .long("from")
                .value_name("LANG")
                .help("Source language code (default: auto)")
                .default_value("auto"),
        )
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .value_name("LANG")
                .help("Target language code (default: zh)")
                .default_value("zh"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .help("Show config file path and exit")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("config") {
        let config_path = get_config_path()?;
        println!("Config file path: {:?}", config_path);
        return Ok(());
    }

    let text = matches.get_one::<String>("text");
    if text.is_none() {
        eprintln!("Error: Text to translate is required");
        eprintln!("Use --help for usage information");
        std::process::exit(1);
    }

    let config = load_config()?;
    let client = Client::new();

    let text = text.unwrap();
    let from_lang = matches.get_one::<String>("from").unwrap();
    let to_lang = matches.get_one::<String>("to").unwrap();

    let result = translate_text(&client, &config, text, from_lang, to_lang).await?;
    print_translation_result(&result)?;

    Ok(())
}
