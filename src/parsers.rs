use crate::Inspector;
use serde_json::Value;

impl Inspector {
    pub fn format_cli(&mut self) -> String {
        let mut options: Vec<String> = Vec::new();
        let image = self.get_container_fact("Config.Image").unwrap_or_default();
        if image.is_empty() {
            return String::new();
        }

        let name = self
            .get_container_fact("Name")
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_default();
        if !self.no_name && !name.is_empty() {
            options.push(format!("--name={}", name));
        }

        self.parse_hostname(&mut options);
        self.parse_user(&mut options);
        self.parse_macaddress(&mut options);
        self.parse_ipv6(&mut options);
        self.parse_pid(&mut options);
        self.parse_cpuset(&mut options);
        self.parse_entrypoint(&mut options);
        self.parse_volumes(&mut options);
        self.parse_env(&mut options);
        self.parse_volumes_from(&mut options);
        self.parse_cap_add(&mut options);
        self.parse_cap_drop(&mut options);
        self.parse_dns(&mut options);
        self.parse_network(&mut options);
        self.parse_privileged(&mut options);
        self.parse_workdir(&mut options);
        self.parse_ports(&mut options);
        self.parse_links(&mut options);
        self.parse_restart(&mut options);
        self.parse_devices(&mut options);
        if !self.no_labels {
            self.parse_labels(&mut options);
        }
        self.parse_log(&mut options);
        self.parse_extra_hosts(&mut options);
        self.parse_runtime(&mut options);
        self.parse_shm_size(&mut options);
        self.parse_memory(&mut options);
        self.parse_memory_reservation(&mut options);
        self.parse_ipc(&mut options);
        self.parse_uts(&mut options);
        self.parse_userns(&mut options);
        self.parse_init(&mut options);
        self.parse_readonly(&mut options);
        self.parse_publish_all(&mut options);
        self.parse_security_opt(&mut options);
        self.parse_sysctl(&mut options);
        self.parse_group_add(&mut options);
        self.parse_dns_option(&mut options);
        self.parse_dns_search(&mut options);
        self.parse_ipv4(&mut options);
        self.parse_link_local_ip(&mut options);
        self.parse_network_alias(&mut options);
        self.parse_healthcheck(&mut options);
        self.parse_stop_signal(&mut options);
        self.parse_stop_timeout(&mut options);
        self.parse_pids_limit(&mut options);
        self.parse_oom_kill_disable(&mut options);
        self.parse_oom_score_adj(&mut options);
        self.parse_memory_swap(&mut options);
        self.parse_memory_swappiness(&mut options);
        self.parse_kernel_memory(&mut options);
        self.parse_ulimit(&mut options);
        self.parse_blkio_device_read_bps(&mut options);
        self.parse_blkio_device_read_iops(&mut options);
        self.parse_blkio_device_write_bps(&mut options);
        self.parse_blkio_device_write_iops(&mut options);
        self.parse_storage_opt(&mut options);
        self.parse_detach(&mut options);
        self.parse_tty(&mut options);
        self.parse_autoremove(&mut options);

        let mut parameters: Vec<String> = Vec::new();
        if self.tidy {
            for opt in &options {
                let opt_name = opt.trim_start_matches('-');
                let key = opt_name
                    .split(&['=', ' '][..])
                    .next()
                    .unwrap_or("")
                    .to_string();
                let value_part = if let Some(idx) = opt.find(&['=', ' '][..]) {
                    &opt[idx + 1..]
                } else {
                    ""
                };
                if !self.is_docker_default(&key, value_part.trim_matches(&['\"', '\''][..])) {
                    parameters.push(opt.clone());
                }
            }
        } else {
            parameters.extend(options);
        }
        parameters.push(image);

        let cmd_parts = self.get_container_fact_list("Config.Cmd");
        if !cmd_parts.is_empty() {
            let img_cmd_parts = self.get_image_fact_list("Config.Cmd");
            // Skip if cmd matches the image default (user didn't override)
            if cmd_parts != img_cmd_parts {
                let cmd_strs: Vec<String> = cmd_parts
                    .iter()
                    .map(|part| match part {
                        Value::String(s) => self.quote(s),
                        _ => part.to_string(),
                    })
                    .collect();
                parameters.push(cmd_strs.join(" "));
            }
        }

        let joined = if self.pretty {
            parameters.join(&format!(" \\\n{}", self.indent))
        } else {
            parameters.join(" ")
        };
        format!("docker run {}", joined)
    }
}
