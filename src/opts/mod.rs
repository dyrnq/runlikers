pub mod core;
pub mod misc;
pub mod namespace;
pub mod resource;

use crate::Inspector;
use serde_json::Value;

impl Inspector {
    pub fn quote(&self, s: &str) -> String {
        shlex::try_quote(s)
            .map(|c| c.to_string())
            .unwrap_or_else(|_| s.to_string())
    }

    pub fn multi_option(&mut self, options: &mut Vec<String>, path: &str, option: &str) {
        let container_values = self.get_container_fact_list(path);
        let image_values = self.get_image_fact_list(path);
        for value in &container_values {
            if let Value::String(s) = value {
                if !image_values.iter().any(|iv| iv == value) {
                    options.push(format!("--{}={}", option, self.quote(s)));
                }
            }
        }
    }

    pub fn is_docker_default(&self, option: &str, value: &str) -> bool {
        match option {
            "runtime" if value == "runc" => true,
            "ipc" if value == "private" || value == "shareable" => true,
            "shm-size" if value == "\"67108864\"" || value == "67108864" => true,
            "stop-signal" => {
                if let Some(img_signal) = self.get_image_fact("Config.StopSignal") {
                    value == img_signal.as_str()
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
