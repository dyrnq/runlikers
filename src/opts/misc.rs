use crate::Inspector;
use serde_json::Value;

impl Inspector {
    pub fn parse_restart(&self, options: &mut Vec<String>) {
        if let Some(name) = self.get_container_fact("HostConfig.RestartPolicy.Name") {
            if name != "no" && !name.is_empty() {
                if let Some(r) =
                    self.get_container_fact("HostConfig.RestartPolicy.MaximumRetryCount")
                {
                    if r != "0" {
                        options.push(format!("--restart=on-failure:{}", r));
                        return;
                    }
                }
                options.push(format!("--restart={}", name));
            }
        }
    }

    pub fn parse_labels(&self, options: &mut Vec<String>) {
        if self.no_labels {
            return;
        }
        let container_labels = self.get_container_fact_map("Config.Labels");
        let image_labels = self.get_image_fact_map("Config.Labels");
        for (key, value) in &container_labels {
            let val_str = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            if let Some(img_val) = image_labels.get(key) {
                let img_str = match img_val {
                    Value::String(s) => s.clone(),
                    _ => img_val.to_string(),
                };
                if img_str == val_str {
                    continue;
                }
            }
            options.push(format!("--label='{}={}'", key, val_str));
        }
    }

    pub fn parse_log(&self, options: &mut Vec<String>) {
        if let Some(ref typ) = self.get_container_fact("HostConfig.LogConfig.Type") {
            if typ != "json-file" {
                options.push(format!("--log-driver={}", typ));
            }
        }
        for (key, value) in &self.get_container_fact_map("HostConfig.LogConfig.Config") {
            let v = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            options.push(format!("--log-opt {}={}", key, v));
        }
    }

    pub fn parse_devices(&self, options: &mut Vec<String>) {
        for device in &self.get_container_fact_list("HostConfig.Devices") {
            if let Value::Object(d) = device {
                let host = d.get("PathOnHost").and_then(|v| v.as_str()).unwrap_or("");
                let container = d
                    .get("PathInContainer")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let perms = d
                    .get("CgroupPermissions")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let mut s = format!("{}:{}", host, container);
                if !perms.is_empty() && perms != "rw" {
                    s.push(':');
                    s.push_str(perms);
                }
                options.push(format!("--device {}", self.quote(&s)));
            }
        }
    }

    pub fn parse_cap_add(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "HostConfig.CapAdd", "cap-add");
    }

    pub fn parse_cap_drop(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "HostConfig.CapDrop", "cap-drop");
    }

    pub fn parse_volumes_from(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "HostConfig.VolumesFrom", "volumes-from");
    }

    pub fn parse_security_opt(&self, options: &mut Vec<String>) {
        for opt in &self.get_container_fact_list("HostConfig.SecurityOpt") {
            if let Value::String(s) = opt {
                options.push(format!("--security-opt {}", self.quote(s)));
            }
        }
    }

    pub fn parse_sysctl(&self, options: &mut Vec<String>) {
        for (key, value) in &self.get_container_fact_map("HostConfig.Sysctls") {
            let v = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            options.push(format!("--sysctl {}={}", key, v));
        }
    }

    pub fn parse_group_add(&self, options: &mut Vec<String>) {
        for g in &self.get_container_fact_list("HostConfig.GroupAdd") {
            if let Value::String(s) = g {
                options.push(format!("--group-add {}", s));
            }
        }
    }

    pub fn parse_runtime(&self, options: &mut Vec<String>) {
        if let Some(r) = self.get_container_fact("HostConfig.Runtime") {
            if !r.is_empty() {
                options.push(format!("--runtime={}", r));
            }
        }
    }

    pub fn parse_init(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.Init") {
            if v == "true" {
                options.push("--init".to_string());
            }
        }
    }

    pub fn parse_readonly(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.ReadonlyRootfs") {
            if v == "true" {
                options.push("--read-only".to_string());
            }
        }
    }

    pub fn parse_publish_all(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.PublishAllPorts") {
            if v == "true" {
                options.push("-P".to_string());
            }
        }
    }

    pub fn parse_healthcheck(&self, options: &mut Vec<String>) {
        let hc = self.get_container_fact_map("HostConfig.Healthcheck");
        if hc.is_empty() {
            return;
        }

        if let Some(Value::Array(test)) = hc.get("Test") {
            if test.iter().any(|v| v.as_str() == Some("NONE")) {
                options.push("--no-healthcheck".to_string());
                return;
            }
            if let Some(Value::String(cmd)) = test.first() {
                if cmd != "NONE" {
                    let rest: Vec<String> = test[1..]
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    let full = if rest.is_empty() {
                        cmd.clone()
                    } else {
                        format!("{} {}", cmd, rest.join(" "))
                    };
                    options.push(format!("--health-cmd {}", self.quote(&full)));
                }
            }
        }
        if let Some(Value::String(v)) = hc.get("Interval") {
            options.push(format!("--health-interval={}", v));
        }
        if let Some(Value::String(v)) = hc.get("Timeout") {
            options.push(format!("--health-timeout={}", v));
        }
        if let Some(Value::String(v)) = hc.get("StartPeriod") {
            options.push(format!("--health-start-period={}", v));
        }
        if let Some(Value::Number(v)) = hc.get("Retries") {
            options.push(format!("--health-retries={}", v));
        }
    }

    pub fn parse_stop_signal(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("Config.StopSignal") {
            if !v.is_empty() {
                options.push(format!("--stop-signal={}", v));
            }
        }
    }

    pub fn parse_stop_timeout(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("Config.StopTimeout") {
            if v != "0" {
                options.push(format!("--stop-timeout={}", v));
            }
        }
    }

    pub fn parse_storage_opt(&self, options: &mut Vec<String>) {
        for (key, value) in &self.get_container_fact_map("HostConfig.StorageOpt") {
            let v = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            options.push(format!("--storage-opt {}={}", key, v));
        }
    }

    pub fn parse_detach(&self, options: &mut Vec<String>) {
        if let Some(a) = self.get_container_fact("Config.AttachStdout") {
            if a == "false" {
                options.push("--detach=true".to_string());
            }
        }
    }

    pub fn parse_tty(&self, options: &mut Vec<String>) {
        if let Some(t) = self.get_container_fact("Config.Tty") {
            if t == "true" {
                options.push("-t".to_string());
            }
        }
    }

    pub fn parse_autoremove(&self, options: &mut Vec<String>) {
        if let Some(ar) = self.get_container_fact("HostConfig.AutoRemove") {
            if ar == "true" {
                options.push("--rm".to_string());
            }
        }
    }
}
