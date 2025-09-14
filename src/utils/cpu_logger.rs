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
    system: Arc<Mutex<System>>, // Persist system state for stable CPU readings
}

impl CpuLogger {
    pub fn new() -> Self {
        // Initialize and pre-refresh the system to avoid first-sample zeros
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            log_file: Arc::new(Mutex::new("server_log.txt".to_string())),
            running: Arc::new(Mutex::new(false)),
            system: Arc::new(Mutex::new(sys)),
        }
    }

    pub async fn start_logging(&self) {
        let running = Arc::clone(&self.running);
        let log_file = Arc::clone(&self.log_file);
        let system = Arc::clone(&self.system);

        let mut running_guard = running.lock().await;
        if *running_guard {
            return; // Already running
        }
        *running_guard = true;
        drop(running_guard);

        let running_clone = Arc::clone(&self.running);
        let log_file_clone = Arc::clone(&self.log_file);
        let system_clone = Arc::clone(&self.system);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(120)); // ogni 2 minuti

            // Warm-up to stabilize CPU deltas
            if let Ok(pid) = sysinfo::get_current_pid() {
                {
                    let mut sys = system.lock().await;
                    sys.refresh_cpu();
                    sys.refresh_process(pid);
                }
                sleep(Duration::from_millis(300)).await;
            }

            loop {
                interval.tick().await;

                let running_check = running_clone.lock().await;
                if !*running_check {
                    break;
                }
                drop(running_check);

                if let Err(e) = Self::log_cpu_usage(&log_file_clone, &system_clone).await {
                    eprintln!("Failed to log CPU usage: {}", e);
                }
            }
        });
    }

    pub async fn stop_logging(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }

    async fn log_cpu_usage(log_file: &Arc<Mutex<String>>, system: &Arc<Mutex<System>>) -> Result<(), Box<dyn std::error::Error>> {
        let (global_cpu, proc_cpu, mem_usage) = Self::get_cpu_usage(system).await?;
        let timestamp = (Utc::now() + chrono::Duration::hours(2)).format("%Y-%m-%d %H:%M:%S");

        let log_entry = format!(
            "[{}] Global CPU: {:.2}% | Server CPU: {:.2}% | Server Memory: {} B ({:.2} MB)\n",
            timestamp, global_cpu, proc_cpu, mem_usage, mem_usage as f32 / (1024.0 * 1024.0)
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

    async fn get_cpu_usage(system: &Arc<Mutex<System>>) -> Result<(f32, f32, u64), Box<dyn std::error::Error>> {
        // Processo corrente
        let pid = sysinfo::get_current_pid()?;

        // Two-step refresh to compute deltas for stable CPU usage
        {
            let mut sys = system.lock().await;
            sys.refresh_cpu();
            sys.refresh_process(pid);
        }
        sleep(Duration::from_millis(300)).await;

        let (global_cpu, proc_cpu, mem_usage) = {
            let mut sys = system.lock().await;
            sys.refresh_cpu();
            sys.refresh_process(pid);

            // CPU globale (media di tutti i core)
            let global_cpu = sys.global_cpu_info().cpu_usage();

            // Processo corrente
            let proc = sys.process(pid).ok_or("Process not found")?;
            let proc_cpu = proc.cpu_usage();
            let mem_usage = proc.memory();

            (global_cpu, proc_cpu, mem_usage)
        };

        Ok((global_cpu, proc_cpu, mem_usage))
    }
}

impl Default for CpuLogger {
    fn default() -> Self {
        Self::new()
    }
}

