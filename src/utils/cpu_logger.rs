use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::sleep;
use sysinfo::System;
use chrono::Utc;

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
            let mut interval = tokio::time::interval(Duration::from_secs(120)); // ogni 2 minuti

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
        let (global_cpu, proc_cpu, mem_usage) = Self::get_cpu_usage().await?;
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

        let log_entry = format!(
            "[{}] Global CPU: {:.2}% | Server CPU: {:.2}% | Server Memory: {} KB\n",
            timestamp, global_cpu, proc_cpu, mem_usage
        );

        let file_path = log_file.lock().await;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*file_path)?;

        file.write_all(log_entry.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    async fn get_cpu_usage() -> Result<(f32, f32, u64), Box<dyn std::error::Error>> {
        let mut sys = System::new_all();
        sys.refresh_all();

        // CPU globale (media di tutti i core)
        let global_cpu = sys.global_cpu_info().cpu_usage();

        // Processo corrente
        let pid = sysinfo::get_current_pid()?;
        let proc = sys.process(pid).ok_or("Process not found")?;
        let proc_cpu = proc.cpu_usage();
        let mem_usage = proc.memory(); // in KB

        Ok((global_cpu, proc_cpu, mem_usage))
    }
}

impl Default for CpuLogger {
    fn default() -> Self {
        Self::new()
    }
}

