use tracing_subscriber::EnvFilter;

pub struct Tfd {
    format_occupancy: usize,
}

impl Tfd {
    pub fn new(format_occupancy: usize) -> Tfd {
        let tfd = Tfd { format_occupancy };
        tfd.init_dev();
        tfd
    }

    fn init_dev(&self) {
        tracing_subscriber::fmt()
            .without_time()
            .with_target(false)
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    pub fn info(&self, func_identifier: &str, token: &str) {
        tracing::info!(
            "### {:<width$} - {}",
            token,
            func_identifier,
            width = self.format_occupancy
        );
    }

    pub fn debug(&self, func_identifier: &str, token: &str) {
        tracing::debug!(
            "### {:<width$} - {}",
            token,
            func_identifier,
            width = self.format_occupancy
        );
    }

    pub fn error(&self, func_identifier: &str, token: &str) {
        tracing::error!(
            "### {:<width$} - {}",
            token,
            func_identifier,
            width = self.format_occupancy
        );
    }
}
