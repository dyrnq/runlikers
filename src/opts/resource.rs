use crate::Inspector;
use serde_json::Value;

impl Inspector {
    pub fn parse_memory(&self, options: &mut Vec<String>) {
        if let Some(m) = self.get_container_fact("HostConfig.Memory") {
            if m != "0" {
                options.push(format!("--memory=\"{}\"", m));
            }
        }
    }

    pub fn parse_memory_reservation(&self, options: &mut Vec<String>) {
        if let Some(m) = self.get_container_fact("HostConfig.MemoryReservation") {
            if m != "0" {
                options.push(format!("--memory-reservation=\"{}\"", m));
            }
        }
    }

    pub fn parse_memory_swap(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.MemorySwap") {
            if v != "0" {
                options.push(format!("--memory-swap=\"{}\"", v));
            }
        }
    }

    pub fn parse_memory_swappiness(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.MemorySwappiness") {
            if v != "-1" {
                options.push(format!("--memory-swappiness={}", v));
            }
        }
    }

    pub fn parse_kernel_memory(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.KernelMemory") {
            if v != "0" {
                options.push(format!("--kernel-memory=\"{}\"", v));
            }
        }
    }

    pub fn parse_shm_size(&self, options: &mut Vec<String>) {
        if let Some(s) = self.get_container_fact("HostConfig.ShmSize") {
            options.push(format!("--shm-size=\"{}\"", s));
        }
    }

    pub fn parse_cpuset(&self, options: &mut Vec<String>) {
        if let Some(c) = self.get_container_fact("HostConfig.CpusetCpus") {
            if !c.is_empty() {
                options.push(format!("--cpuset-cpus={}", c));
            }
        }
        if let Some(c) = self.get_container_fact("HostConfig.CpusetMems") {
            if !c.is_empty() {
                options.push(format!("--cpuset-mems={}", c));
            }
        }
    }

    pub fn parse_ulimit(&self, options: &mut Vec<String>) {
        for ulimit in &self.get_container_fact_list("HostConfig.Ulimits") {
            if let Value::Object(u) = ulimit {
                let name = u.get("Name").and_then(|v| v.as_str()).unwrap_or("");
                let soft = u.get("Soft").and_then(|v| v.as_u64()).unwrap_or(0);
                let hard = u.get("Hard").and_then(|v| v.as_u64()).unwrap_or(0);
                if !name.is_empty() {
                    options.push(format!("--ulimit {}={}:{}", name, soft, hard));
                }
            }
        }
    }

    pub fn parse_pids_limit(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.PidsLimit") {
            if v != "0" {
                options.push(format!("--pids-limit={}", v));
            }
        }
    }

    pub fn parse_oom_kill_disable(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.OomKillDisable") {
            if v == "true" {
                options.push("--oom-kill-disable".to_string());
            }
        }
    }

    pub fn parse_oom_score_adj(&self, options: &mut Vec<String>) {
        if let Some(v) = self.get_container_fact("HostConfig.OomScoreAdj") {
            if v != "0" {
                options.push(format!("--oom-score-adj={}", v));
            }
        }
    }

    pub fn parse_blkio_device_read_bps(&self, options: &mut Vec<String>) {
        for d in &self.get_container_fact_list("HostConfig.BlkioDeviceReadBps") {
            if let Value::Object(o) = d {
                let path = o.get("Path").and_then(|v| v.as_str()).unwrap_or("");
                let rate = o.get("Rate").and_then(|v| v.as_u64()).unwrap_or(0);
                if !path.is_empty() && rate > 0 {
                    options.push(format!("--device-read-bps {}:{}", path, rate));
                }
            }
        }
    }

    pub fn parse_blkio_device_read_iops(&self, options: &mut Vec<String>) {
        for d in &self.get_container_fact_list("HostConfig.BlkioDeviceReadIOps") {
            if let Value::Object(o) = d {
                let path = o.get("Path").and_then(|v| v.as_str()).unwrap_or("");
                let rate = o.get("Rate").and_then(|v| v.as_u64()).unwrap_or(0);
                if !path.is_empty() && rate > 0 {
                    options.push(format!("--device-read-iops {}:{}", path, rate));
                }
            }
        }
    }

    pub fn parse_blkio_device_write_bps(&self, options: &mut Vec<String>) {
        for d in &self.get_container_fact_list("HostConfig.BlkioDeviceWriteBps") {
            if let Value::Object(o) = d {
                let path = o.get("Path").and_then(|v| v.as_str()).unwrap_or("");
                let rate = o.get("Rate").and_then(|v| v.as_u64()).unwrap_or(0);
                if !path.is_empty() && rate > 0 {
                    options.push(format!("--device-write-bps {}:{}", path, rate));
                }
            }
        }
    }

    pub fn parse_blkio_device_write_iops(&self, options: &mut Vec<String>) {
        for d in &self.get_container_fact_list("HostConfig.BlkioDeviceWriteIOps") {
            if let Value::Object(o) = d {
                let path = o.get("Path").and_then(|v| v.as_str()).unwrap_or("");
                let rate = o.get("Rate").and_then(|v| v.as_u64()).unwrap_or(0);
                if !path.is_empty() && rate > 0 {
                    options.push(format!("--device-write-iops {}:{}", path, rate));
                }
            }
        }
    }
}
