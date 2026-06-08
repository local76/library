/// Common structured request sent to background daemons / IPC services.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcRequest {
    pub command: String,
    pub payload: String,
}

impl IpcRequest {
    pub fn new(command: &str, payload: &str) -> Self {
        Self {
            command: command.to_string(),
            payload: payload.to_string(),
        }
    }

    /// Serializes to a simple message format: "COMMAND:payload"
    pub fn serialize(&self) -> String {
        format!("{}:{}", self.command, self.payload)
    }

    /// Deserializes from "COMMAND:payload"
    pub fn deserialize(msg: &str) -> Option<Self> {
        let parts: Vec<&str> = msg.splitn(2, ':').collect();
        if parts.is_empty() {
            None
        } else {
            let command = parts[0].trim().to_string();
            let payload = parts.get(1).map(|p| p.to_string()).unwrap_or_default();
            Some(Self { command, payload })
        }
    }
}

/// Common structured response returned by background daemons / IPC services.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcResponse {
    pub success: bool,
    pub message: String,
    pub payload: String,
}

impl IpcResponse {
    pub fn ok(msg: &str, payload: &str) -> Self {
        Self {
            success: true,
            message: msg.to_string(),
            payload: payload.to_string(),
        }
    }

    pub fn err(msg: &str) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            payload: String::new(),
        }
    }

    /// Serializes to a simple message format: "STATUS:message:payload"
    /// Where STATUS is "OK" or "ERR"
    pub fn serialize(&self) -> String {
        let status = if self.success { "OK" } else { "ERR" };
        format!("{}:{}:{}", status, self.message, self.payload)
    }

    /// Deserializes from "STATUS:message:payload"
    /// Where STATUS is "OK" or "ERR"
    pub fn deserialize(msg: &str) -> Option<Self> {
        let parts: Vec<&str> = msg.splitn(3, ':').collect();
        if parts.len() < 2 {
            None
        } else {
            let success = parts[0] == "OK";
            let message = parts[1].to_string();
            let payload = parts.get(2).map(|p| p.to_string()).unwrap_or_default();
            Some(Self { success, message, payload })
        }
    }
}

/// Helper to serialize core types for API.
pub fn serialize_dashboard(info: &crate::core::DashboardInfo) -> String {
    format!("{{\"os\":\"{}\",\"cpu\":\"{}\"}}", info.os, info.cpu)
}

/// Serializes a DashboardInfo into a format suitable for transmission.
pub fn serialize_dashboard_info(info: &crate::core::DashboardInfo) -> String {
    format!(
        "{};{};{};{};{};{};{};{};{}",
        info.os,
        info.logo_text,
        info.kernel,
        info.hostname,
        info.cpu,
        info.uptime_secs,
        info.mem_used_mb,
        info.mem_total_mb,
        info.power_status
    )
}

/// Deserializes a transmitted string back into DashboardInfo.
pub fn deserialize_dashboard_info(serialized: &str) -> Option<crate::core::DashboardInfo> {
    let parts: Vec<&str> = serialized.split(';').collect();
    if parts.len() < 9 {
        None
    } else {
        Some(crate::core::DashboardInfo {
            os: parts[0].to_string(),
            logo_text: parts[1].to_string(),
            kernel: parts[2].to_string(),
            hostname: parts[3].to_string(),
            cpu: parts[4].to_string(),
            uptime_secs: parts[5].parse().unwrap_or(0),
            mem_used_mb: parts[6].parse().unwrap_or(0),
            mem_total_mb: parts[7].parse().unwrap_or(0),
            mem_used_pct: if let (Ok(u), Ok(t)) = (parts[6].parse::<f32>(), parts[7].parse::<f32>()) {
                if t > 0.0 { (u / t) * 100.0 } else { 0.0 }
            } else {
                0.0
            },
            power_status: parts[8].to_string(),
            disk_summary: String::new(),
            gpus: String::new(),
            monitors: String::new(),
        })
    }
}
