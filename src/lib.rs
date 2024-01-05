use std::sync::OnceLock;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;


pub fn tfd()-> &'static Tfd{
    static INSTANCE:OnceLock<Tfd> = OnceLock::new();

    INSTANCE.get_or_init(||{
        Tfd::new()
    })
}

pub struct Tfd {
    pub format_occupancy: usize,
}

impl Tfd {
    fn new() -> Tfd {
        let tfd = Tfd { format_occupancy: 15 };
        tfd.init_dev();
        tfd
    }

    fn init_dev(&self) {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .without_time()
            .with_target(false)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
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

impl Default for Tfd {
    fn default() -> Self {
        Self::new()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_logging() {
        let tfd = tfd();
        tfd.info("test_info", "Info message test");
    }

    #[test]
    fn test_debug_logging() {
        let tfd = tfd();
        tfd.debug("test_debug", "Debug message test");
    }

    #[test]
    fn test_error_logging() {
        let tfd = tfd();
        tfd.error("test_error", "Error message test");
    }
}
