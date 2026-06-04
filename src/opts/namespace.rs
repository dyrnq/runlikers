use crate::Inspector;
use serde_json::Value;

impl Inspector {
    pub fn parse_network(&self, options: &mut Vec<String>) {
        if let Some(n) = self.get_container_fact("HostConfig.NetworkMode") {
            if n != "default" && !n.is_empty() {
                options.push(format!("--network={}", n));
            }
        }
    }

    pub fn parse_privileged(&self, options: &mut Vec<String>) {
        if let Some(p) = self.get_container_fact("HostConfig.Privileged") {
            if p == "true" {
                options.push("--privileged".to_string());
            }
        }
    }

    pub fn parse_pid(&self, options: &mut Vec<String>) {
        if let Some(pid) = self.get_container_fact("HostConfig.PidMode") {
            if !pid.is_empty() && pid != "default" {
                options.push(format!("--pid {}", self.quote(&pid)));
            }
        }
    }

    pub fn parse_ipc(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.IpcMode") {
            if !v.is_empty() && v != "default" {
                options.push(format!("--ipc={}", v));
            }
        }
    }

    pub fn parse_uts(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.UTSMode") {
            if !v.is_empty() && v != "default" {
                options.push(format!("--uts={}", v));
            }
        }
    }

    pub fn parse_userns(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.UsernsMode") {
            if !v.is_empty() {
                options.push(format!("--userns={}", v));
            }
        }
    }

    pub fn parse_dns(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "HostConfig.Dns", "dns");
    }

    pub fn parse_dns_option(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "HostConfig.DnsOptions", "dns-option");
    }

    pub fn parse_dns_search(&mut self, options: &mut Vec<String>) {
        self.multi_option(options, "HostConfig.DnsSearch", "dns-search");
    }

    pub fn parse_extra_hosts(&self, options: &mut Vec<String>) {
        for host in &self.get_container_fact_list("HostConfig.ExtraHosts") {
            if let Value::String(s) = host {
                options.push(format!("--add-host {}", self.quote(s)));
            }
        }
    }

    pub fn parse_network_alias(&self, options: &mut Vec<String>) {
        for network in self
            .get_container_fact_map("NetworkSettings.Networks")
            .values()
        {
            if let Some(Value::Array(aliases)) = network.get("Aliases") {
                for alias in aliases {
                    if let Value::String(s) = alias {
                        options.push(format!("--network-alias {}", s));
                    }
                }
                break;
            }
        }
    }

    pub fn parse_link_local_ip(&self, options: &mut Vec<String>) {
        for network in self
            .get_container_fact_map("NetworkSettings.Networks")
            .values()
        {
            if let Some(Value::Array(ips)) = network.get("LinkLocalIPs") {
                for ip in ips {
                    if let Value::String(s) = ip {
                        options.push(format!("--link-local-ip {}", s));
                    }
                }
                break;
            }
        }
    }
}
