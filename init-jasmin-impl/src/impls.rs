use std::fs;

use serde_json::Value;

pub struct Impl {
    pub outpath: String,
    pub impl_name: String,
    pub archs: Vec<String>,
    pub impl_modifiers: Vec<String>,
    pub src_files: Vec<String>, // base_name: Does not include the .jinc extension
    pub test_dirs: Vec<String>,
    pub parameter_sets: Option<Vec<String>>,
}

pub enum ConfigError {
    MissingField(String),
    MissingValueForField(String),
}

impl ConfigError {
    pub fn get_message(&self) -> &str {
        match self {
            ConfigError::MissingField(msg) => msg,
            ConfigError::MissingValueForField(msg) => msg,
        }
    }
}

fn parse_json_config(file_content: &str, outpath: &str) -> Result<Impl, ConfigError> {
    let json_str: Value = serde_json::from_str(file_content)
        .map_err(|_| ConfigError::MissingField("Invalid JSON".to_string()))?;

    let impl_name = json_str
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| ConfigError::MissingField("impl_name".to_string()))?
        .to_string();

    let archs = json_str
        .get("arch")
        .and_then(Value::as_array)
        .ok_or_else(|| ConfigError::MissingField("archs".to_string()))?
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    let impl_modifiers = json_str
        .get("impl_modifiers")
        .and_then(Value::as_array)
        .ok_or_else(|| ConfigError::MissingField("impl_modifiers".to_string()))?
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    let src_files = json_str
        .get("src_files")
        .and_then(Value::as_array)
        .ok_or_else(|| ConfigError::MissingField("dirs".to_string()))?
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    let test_dirs = json_str
        .get("test_dirs")
        .and_then(Value::as_array)
        .ok_or_else(|| ConfigError::MissingField("test_dirs".to_string()))?
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    let parameter_sets = json_str
        .get("parameter_sets")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect()
        });

    Ok(Impl {
        outpath: outpath.to_string(),
        impl_name,
        archs,
        impl_modifiers,
        src_files,
        test_dirs,
        parameter_sets,
    })
}

impl Impl {
    pub fn build(config_path: &str, outpath: &str) -> Result<Impl, ConfigError> {
        let file_content = fs::read_to_string(config_path).expect("Failed to read config file");
        parse_json_config(&file_content, outpath)
    }
}
