pub mod opts;
pub mod parsers;

use bollard::Docker;
use serde_json::Value;
use std::collections::BTreeMap;

pub struct Inspector {
    pub no_name: bool,
    pub use_volume_id: bool,
    pub pretty: bool,
    pub no_labels: bool,
    pub use_mount_flag: bool,
    pub tidy: bool,
    pub docker_host: Option<String>,
    pub container_facts: Option<Value>,
    pub image_facts: Option<Value>,
}

impl Inspector {
    pub fn new(no_name: bool, use_volume_id: bool, pretty: bool, no_labels: bool) -> Self {
        Inspector {
            no_name,
            use_volume_id,
            pretty,
            no_labels,
            use_mount_flag: false,
            tidy: false,
            docker_host: None,
            container_facts: None,
            image_facts: None,
        }
    }

    pub fn set_container_facts(&mut self, raw_json: &str) -> Result<(), serde_json::Error> {
        let v: Value = serde_json::from_str(raw_json)?;
        let v = if let Value::Array(arr) = &v {
            arr.first().cloned().unwrap_or(v)
        } else {
            v
        };
        self.container_facts = Some(v);
        Ok(())
    }

    pub async fn inspect(&mut self, container: &str) -> Result<(), Box<dyn std::error::Error>> {
        let docker = if let Some(ref host) = self.docker_host {
            Docker::connect_with_host(host)?
        } else {
            Docker::connect_with_local_defaults()?
        };
        let container_json = docker.inspect_container(container, None).await?;
        self.container_facts = Some(serde_json::to_value(&container_json)?);
        let image_id = self.get_container_fact("Image").unwrap_or_default();
        let image_json = docker.inspect_image(&image_id).await?;
        self.image_facts = Some(serde_json::to_value(&image_json)?);
        Ok(())
    }

    pub fn get_container_fact(&self, path: &str) -> Option<String> {
        self.get_fact(path, self.container_facts.as_ref())
    }

    pub fn get_image_fact(&self, path: &str) -> Option<String> {
        self.get_fact(path, self.image_facts.as_ref())
    }

    pub fn get_fact(&self, path: &str, value: Option<&Value>) -> Option<String> {
        let value = value?;
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;
        for p in &parts {
            current = current.get(*p)?;
        }
        match current {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            Value::Array(a) => Some(serde_json::to_string(a).unwrap_or_default()),
            Value::Object(o) => Some(serde_json::to_string(o).unwrap_or_default()),
            Value::Null => None,
        }
    }

    pub fn get_container_fact_list(&self, path: &str) -> Vec<Value> {
        self.get_fact_list(path, self.container_facts.as_ref())
    }

    pub fn get_image_fact_list(&self, path: &str) -> Vec<Value> {
        self.get_fact_list(path, self.image_facts.as_ref())
    }

    pub fn get_fact_list(&self, path: &str, value: Option<&Value>) -> Vec<Value> {
        let value = match value {
            Some(v) => v,
            None => return vec![],
        };
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;
        for p in &parts {
            current = match current.get(*p) {
                Some(v) => v,
                None => return vec![],
            };
        }
        match current {
            Value::Array(a) => a.clone(),
            _ => vec![],
        }
    }

    pub fn get_container_fact_map(&self, path: &str) -> BTreeMap<String, Value> {
        self.get_fact_map(path, self.container_facts.as_ref())
    }

    pub fn get_image_fact_map(&self, path: &str) -> BTreeMap<String, Value> {
        self.get_fact_map(path, self.image_facts.as_ref())
    }

    pub fn get_fact_map(&self, path: &str, value: Option<&Value>) -> BTreeMap<String, Value> {
        let value = match value {
            Some(v) => v,
            None => return BTreeMap::new(),
        };
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;
        for p in &parts {
            current = match current.get(*p) {
                Some(v) => v,
                None => return BTreeMap::new(),
            };
        }
        match current {
            Value::Object(o) => o.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            _ => BTreeMap::new(),
        }
    }
}
