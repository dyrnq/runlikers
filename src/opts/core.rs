use crate::Inspector;
use serde_json::Value;

impl Inspector {
    pub fn parse_hostname(&self, options: &mut Vec<String>) {
        if let Some(h) = self.get_container_fact("Config.Hostname") {
            if self.tidy {
                // Docker auto-assigns hostname = container ID[:12] when not specified
                if let Some(cid) = self.get_container_fact("Id") {
                    if h == cid.get(..12).unwrap_or(&cid) {
                        return;
                    }
                }
            }
            options.push(format!("--hostname={}", h));
        }
    }

    pub fn parse_user(&self, options: &mut Vec<String>) {
        if let Some(u) = self.get_container_fact("Config.User") {
            if !u.is_empty() && u != "0" {
                options.push(format!("--user={}", u));
            }
        }
    }

    pub fn parse_macaddress(&self, options: &mut Vec<String>) {
        let config_mac = self.get_container_fact("Config.MacAddress");
        // In tidy mode, only output if user explicitly specified --mac-address
        // (Config.MacAddress is set). Docker auto-assigned MACs show up only
        // in NetworkSettings.MacAddress with Config.MacAddress = None.
        if self.tidy {
            if let Some(m) = config_mac {
                if !m.is_empty() {
                    options.push(format!("--mac-address={}", m));
                }
            }
        } else {
            let mac = config_mac.or_else(|| self.get_container_fact("NetworkSettings.MacAddress"));
            if let Some(m) = mac {
                if !m.is_empty() {
                    options.push(format!("--mac-address={}", m));
                }
            }
        }
    }

    pub fn parse_ipv6(&self, options: &mut Vec<String>) {
        for network in self
            .get_container_fact_map("NetworkSettings.Networks")
            .values()
        {
            if let Some(Value::String(addr)) =
                network.get("IPAMConfig").and_then(|i| i.get("IPv6Address"))
            {
                options.push(format!("--ip6={}", addr));
                break;
            }
        }
    }

    pub fn parse_ipv4(&self, options: &mut Vec<String>) {
        for network in self
            .get_container_fact_map("NetworkSettings.Networks")
            .values()
        {
            if let Some(Value::String(addr)) =
                network.get("IPAMConfig").and_then(|i| i.get("IPv4Address"))
            {
                options.push(format!("--ip={}", addr));
                break;
            }
        }
    }

    pub fn parse_ports(&self, options: &mut Vec<String>) {
        let mut all_ports = self.get_container_fact_map("NetworkSettings.Ports");
        all_ports.extend(self.get_container_fact_map("HostConfig.PortBindings"));

        for (port_key, bindings_val) in &all_ports {
            let parts: Vec<&str> = port_key.split('/').collect();
            let container_port = parts[0];
            let protocol = if parts.len() > 1 { parts[1] } else { "tcp" };
            let proto = if protocol == "tcp" {
                String::new()
            } else {
                format!("/{}", protocol)
            };

            match bindings_val {
                Value::Null => options.push(format!("--expose={}{}", container_port, proto)),
                Value::Array(bindings) => {
                    for binding in bindings {
                        if let Value::Object(b) = binding {
                            let ip = b.get("HostIp").and_then(|v| v.as_str()).unwrap_or("");
                            let hp = b.get("HostPort").and_then(|v| v.as_str()).unwrap_or("");
                            let ip_part = if !ip.is_empty() && ip != "0.0.0.0" && ip != "::" {
                                format!("{}:", ip)
                            } else {
                                String::new()
                            };
                            let hp_part = if !hp.is_empty() && hp != "0" {
                                format!("{}:", hp)
                            } else {
                                String::new()
                            };
                            options.push(format!(
                                "-p {}{}{}{}",
                                ip_part, hp_part, container_port, proto
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn parse_volumes(&self, options: &mut Vec<String>) {
        let mounts = self.get_container_fact_list("Mounts");
        let binds = self.get_container_fact_list("HostConfig.Binds");

        if !mounts.is_empty() {
            for mount in &mounts {
                if let Value::Object(m) = mount {
                    let typ = m.get("Type").and_then(|v| v.as_str()).unwrap_or("");
                    let source = m.get("Source").and_then(|v| v.as_str()).unwrap_or("");
                    let dest = m.get("Destination").and_then(|v| v.as_str()).unwrap_or("");
                    let mode = m.get("Mode").and_then(|v| v.as_str()).unwrap_or("");
                    let rw = m.get("RW").and_then(|v| v.as_bool()).unwrap_or(true);

                    match typ {
                        "bind" | "volume" => {
                            let vol_src = if self.use_volume_id {
                                source.to_string()
                            } else {
                                source
                                    .replace("/var/lib/docker/volumes/", "")
                                    .trim_end_matches("/_data")
                                    .to_string()
                            };
                            let vol_mode: String = if !rw {
                                ":ro".to_string()
                            } else if !mode.is_empty() && mode != "rw" {
                                format!(":{}", mode)
                            } else {
                                String::new()
                            };
                            options.push(format!("--volume {}:{}{}", vol_src, dest, vol_mode));
                            if self.use_mount_flag {
                                let mount_src = if typ == "volume" {
                                    vol_src.clone()
                                } else {
                                    source.to_string()
                                };
                                let mut mount_opts = format!(
                                    "type={},source={},destination={}",
                                    typ, mount_src, dest
                                );
                                if !rw {
                                    mount_opts.push_str(",readonly");
                                }
                                if !mode.is_empty() && mode != "rw" {
                                    for m in mode.split(',') {
                                        let m = m.trim();
                                        if m != "rw" {
                                            mount_opts.push_str(&format!(",{}", m));
                                        }
                                    }
                                }
                                options.push(format!("--mount {}", self.quote(&mount_opts)));
                            }
                        }
                        "tmpfs" => options.push(format!("--tmpfs {}", dest)),
                        _ => {}
                    }
                }
            }
        } else {
            for bind in &binds {
                if let Value::String(s) = bind {
                    options.push(format!("--volume {}", self.quote(s)));
                    if self.use_mount_flag {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() >= 2 {
                            let src = parts[0];
                            let dest = parts[1];
                            let mode = if parts.len() > 2 { parts[2] } else { "" };
                            let typ = if src.starts_with('/') || src.starts_with('.') {
                                "bind"
                            } else {
                                "volume"
                            };
                            let mut mount_opts =
                                format!("type={},source={},destination={}", typ, src, dest);
                            if mode == "ro" {
                                mount_opts.push_str(",readonly");
                            }
                            options.push(format!("--mount {}", self.quote(&mount_opts)));
                        }
                    }
                }
            }
        }
    }

    pub fn parse_links(&self, options: &mut Vec<String>) {
        for link in &self.get_container_fact_list("HostConfig.Links") {
            if let Value::String(s) = link {
                options.push(format!("--link {}", self.quote(s)));
            }
        }
    }

    pub fn parse_entrypoint(&self, options: &mut Vec<String>) {
        let entry = self.get_container_fact_list("Config.Entrypoint");
        let img_entry = self.get_image_fact_list("Config.Entrypoint");
        if !entry.is_empty() && entry != img_entry {
            if let Some(Value::String(ep)) = entry.first() {
                options.push(format!("--entrypoint {}", self.quote(ep)));
            }
        }
    }

    pub fn parse_workdir(&self, options: &mut Vec<String>) {
        if let Some(w) = self.get_container_fact("Config.WorkingDir") {
            if w.is_empty() {
                return;
            }
            if self.tidy {
                if let Some(img_w) = self.get_image_fact("Config.WorkingDir") {
                    if w == img_w {
                        return;
                    }
                }
            }
            options.push(format!("--workdir={}", w));
        }
    }

    pub fn parse_env(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "Config.Env", "env");
    }
}
