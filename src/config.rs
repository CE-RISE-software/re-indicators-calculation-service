#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub hex_core_base_url: String,
    pub artifact_base_url_template: String,
    pub http_timeout_secs: u64,
    pub bind_address: String,
    pub port: u16,
}

impl RuntimeConfig {
    pub fn from_env() -> Self {
        Self {
            hex_core_base_url: read_string("HEX_CORE_BASE_URL")
                .unwrap_or_else(|| crate::DEFAULT_HEX_CORE_BASE_URL.to_string()),
            artifact_base_url_template: read_string("ARTIFACT_BASE_URL_TEMPLATE")
                .unwrap_or_else(|| crate::DEFAULT_ARTIFACT_BASE_URL_TEMPLATE.to_string()),
            http_timeout_secs: read_u64("HTTP_TIMEOUT_SECS")
                .unwrap_or(crate::DEFAULT_HTTP_TIMEOUT_SECS),
            bind_address: read_string("BIND_ADDRESS")
                .unwrap_or_else(|| crate::DEFAULT_BIND_ADDRESS.to_string()),
            port: read_u16("PORT").unwrap_or(crate::DEFAULT_PORT),
        }
    }

    pub fn socket_addr(&self) -> std::net::SocketAddr {
        let address = format!("{}:{}", self.bind_address, self.port);
        address.parse().unwrap_or_else(|_| {
            panic!("invalid bind address or port in runtime configuration: {address}")
        })
    }
}

fn read_string(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn read_u64(key: &str) -> Option<u64> {
    std::env::var(key).ok()?.trim().parse().ok()
}

fn read_u16(key: &str) -> Option<u16> {
    std::env::var(key).ok()?.trim().parse().ok()
}
