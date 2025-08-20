use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CpuLogger {
    log_file: Arc<Mutex<String>>,
    running: Arc<Mutex<bool>>,
}

impl CpuLogger {
    pub fn new() -> Self {
        Self {
            log_file: Arc::new(Mutex::new("server_log.txt".to_string())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start_logging(&self) {
        let running = Arc::clone(&self.running);
        let log_file = Arc::clone(&self.log_file);
        
        let mut running_guard = running.lock().await;
        if *running_guard {
            return; // Already running
        }
        *running_guard = true;
        drop(running_guard);

        let running_clone = Arc::clone(&self.running);
        let log_file_clone = Arc::clone(&self.log_file);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(120)); // 2 minutes
            
            loop {
                interval.tick().await;
                
                let running_check = running_clone.lock().await;
                if !*running_check {
                    break;
                }
                drop(running_check);

                if let Err(e) = Self::log_cpu_usage(&log_file_clone).await {
                    eprintln!("Failed to log CPU usage: {}", e);
                }
            }
        });
    }

    pub async fn stop_logging(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }

    async fn log_cpu_usage(log_file: &Arc<Mutex<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let cpu_usage = Self::get_cpu_usage().await?;
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        
        let log_entry = format!("[{}] CPU Usage: {:.2}%\n", timestamp, cpu_usage);
        
        let file_path = log_file.lock().await;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*file_path)?;
        
        file.write_all(log_entry.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }

    async fn get_cpu_usage() -> Result<f64, Box<dyn std::error::Error>> {
        // For Windows, we'll use a simple approach
        // In a production environment, you might want to use a more sophisticated method
        // like the `sysinfo` crate or Windows Performance Counters
        
        // This is a simplified implementation
        // For now, we'll return a placeholder value
        // In a real implementation, you would read actual CPU usage from the system
        
        Ok(25.0) // Placeholder value - replace with actual CPU monitoring
    }
}

impl Default for CpuLogger {
    fn default() -> Self {
        Self::new()
    }
}
