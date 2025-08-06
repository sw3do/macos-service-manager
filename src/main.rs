use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::str;

#[derive(Parser)]
#[command(name = "service-manager")]
#[command(about = "macOS Service Manager - Manage system services")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long, help = "Show only running services")]
        running: bool,
        #[arg(short, long, help = "Include brew services")]
        brew: bool,
    },
    Start {
        #[arg(short, long, help = "Include brew services")]
        brew: bool,
    },
    Stop {
        #[arg(short, long, help = "Include brew services")]
        brew: bool,
    },
    Status {
        #[arg(help = "Service name to check status")]
        service: String,
        #[arg(short, long, help = "Check as brew service")]
        brew: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    name: String,
    status: String,
    pid: Option<String>,
    service_type: String,
}

struct ServiceManager {
    brew_available: bool,
}

impl ServiceManager {
    fn new() -> Self {
        let brew_available = Self::check_brew_availability();
        Self { brew_available }
    }

    fn check_brew_availability() -> bool {
        Command::new("which")
            .arg("brew")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn list_launchd_services(
        &self,
        running_only: bool,
    ) -> Result<Vec<Service>, Box<dyn std::error::Error>> {
        let output = Command::new("launchctl").arg("list").output()?;

        let output_str = str::from_utf8(&output.stdout)?;
        let mut services = Vec::new();

        for line in output_str.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let pid = if parts[0] == "-" {
                    None
                } else {
                    Some(parts[0].to_string())
                };
                let status = if pid.is_some() { "running" } else { "stopped" };
                let name = parts[2].to_string();

                if !running_only || status == "running" {
                    services.push(Service {
                        name,
                        status: status.to_string(),
                        pid,
                        service_type: "launchd".to_string(),
                    });
                }
            }
        }

        Ok(services)
    }

    async fn list_brew_services(
        &self,
        running_only: bool,
    ) -> Result<Vec<Service>, Box<dyn std::error::Error>> {
        if !self.brew_available {
            return Ok(Vec::new());
        }

        let output = Command::new("brew").arg("services").arg("list").output()?;

        let output_str = str::from_utf8(&output.stdout)?;
        let mut services = Vec::new();

        for line in output_str.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let status = parts[1].to_string();

                if !running_only || status == "started" {
                    services.push(Service {
                        name,
                        status,
                        pid: None,
                        service_type: "brew".to_string(),
                    });
                }
            }
        }

        Ok(services)
    }

    async fn start_service(
        &self,
        service_name: &str,
        is_brew: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if is_brew {
            if !self.brew_available {
                return Err("Brew is not available".into());
            }
            let output = Command::new("brew")
                .arg("services")
                .arg("start")
                .arg(service_name)
                .output()?;

            if output.status.success() {
                println!(
                    "{}",
                    format!("✅ Brew service '{service_name}' started").green()
                );
            } else {
                let error = str::from_utf8(&output.stderr)?;
                return Err(format!("Failed to start service: {error}").into());
            }
        } else {
            let output = Command::new("launchctl")
                .arg("load")
                .arg("-w")
                .arg(service_name)
                .output()?;

            if output.status.success() {
                println!(
                    "{}",
                    format!("✅ Launchd service '{service_name}' started").green()
                );
            } else {
                let error = str::from_utf8(&output.stderr)?;
                return Err(format!("Failed to start service: {error}").into());
            }
        }
        Ok(())
    }

    async fn stop_service(
        &self,
        service_name: &str,
        is_brew: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if is_brew {
            if !self.brew_available {
                return Err("Brew is not available".into());
            }
            let output = Command::new("brew")
                .arg("services")
                .arg("stop")
                .arg(service_name)
                .output()?;

            if output.status.success() {
                println!(
                    "{}",
                    format!("🛑 Brew service '{service_name}' stopped").red()
                );
            } else {
                let error = str::from_utf8(&output.stderr)?;
                return Err(format!("Failed to stop service: {error}").into());
            }
        } else {
            let output = Command::new("launchctl")
                .arg("unload")
                .arg("-w")
                .arg(service_name)
                .output()?;

            if output.status.success() {
                println!(
                    "{}",
                    format!("🛑 Launchd service '{service_name}' stopped").red()
                );
            } else {
                let error = str::from_utf8(&output.stderr)?;
                return Err(format!("Failed to stop service: {error}").into());
            }
        }
        Ok(())
    }

    async fn get_service_status(
        &self,
        service_name: &str,
        is_brew: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if is_brew {
            if !self.brew_available {
                return Err("Brew is not available".into());
            }
            let services = self.list_brew_services(false).await?;
            if let Some(service) = services.iter().find(|s| s.name == service_name) {
                let status_color = match service.status.as_str() {
                    "started" => service.status.green(),
                    "stopped" => service.status.red(),
                    _ => service.status.yellow(),
                };
                println!(
                    "📋 Brew Service: {} - Status: {}",
                    service.name.blue(),
                    status_color
                );
            } else {
                println!(
                    "{}",
                    format!("❌ Brew service '{service_name}' not found").red()
                );
            }
        } else {
            let services = self.list_launchd_services(false).await?;
            if let Some(service) = services.iter().find(|s| s.name == service_name) {
                let status_color = match service.status.as_str() {
                    "running" => service.status.green(),
                    "stopped" => service.status.red(),
                    _ => service.status.yellow(),
                };
                let pid_info = service
                    .pid
                    .as_ref()
                    .map_or("N/A".to_string(), |p| p.clone());
                println!(
                    "📋 Launchd Service: {} - Status: {} - PID: {}",
                    service.name.blue(),
                    status_color,
                    pid_info.cyan()
                );
            } else {
                println!(
                    "{}",
                    format!("❌ Launchd service '{service_name}' not found").red()
                );
            }
        }
        Ok(())
    }

    async fn interactive_start_service(
        &self,
        include_brew: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut all_services = Vec::new();

        let launchd_services = self.list_launchd_services(false).await?;
        all_services.extend(launchd_services);

        if include_brew && self.brew_available {
            let brew_services = self.list_brew_services(false).await?;
            all_services.extend(brew_services);
        }

        let stopped_services: Vec<&Service> = all_services
            .iter()
            .filter(|s| {
                s.status == "stopped" || (s.service_type == "brew" && s.status != "started")
            })
            .collect();

        if stopped_services.is_empty() {
            println!("{}", "✅ All services are already running!".green());
            return Ok(());
        }

        let service_names: Vec<String> = stopped_services
            .iter()
            .map(|s| format!("{} [{}]", s.name, s.service_type.to_uppercase()))
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 Select the service you want to start:")
            .items(&service_names)
            .interact()?;

        let selected_service = stopped_services[selection];
        let is_brew = selected_service.service_type == "brew";

        self.start_service(&selected_service.name, is_brew).await
    }

    async fn interactive_stop_service(
        &self,
        include_brew: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut all_services = Vec::new();

        let launchd_services = self.list_launchd_services(false).await?;
        all_services.extend(launchd_services);

        if include_brew && self.brew_available {
            let brew_services = self.list_brew_services(false).await?;
            all_services.extend(brew_services);
        }

        let running_services: Vec<&Service> = all_services
            .iter()
            .filter(|s| s.status == "running" || s.status == "started")
            .collect();

        if running_services.is_empty() {
            println!("{}", "🛑 No running services found!".red());
            return Ok(());
        }

        let service_names: Vec<String> = running_services
            .iter()
            .map(|s| {
                let pid_info = s
                    .pid
                    .as_ref()
                    .map_or("".to_string(), |p| format!(" (PID: {p})"));
                format!("{} [{}]{}", s.name, s.service_type.to_uppercase(), pid_info)
            })
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🛑 Select the service you want to stop:")
            .items(&service_names)
            .interact()?;

        let selected_service = running_services[selection];
        let is_brew = selected_service.service_type == "brew";

        self.stop_service(&selected_service.name, is_brew).await
    }

    fn print_services(&self, services: &[Service]) {
        if services.is_empty() {
            println!("{}", "📭 No services found".yellow());
            return;
        }

        println!("{}", "🔧 System Services:".bold().blue());
        println!("{}", "─".repeat(80).blue());

        for service in services {
            let status_icon = match service.status.as_str() {
                "running" | "started" => "🟢",
                "stopped" => "🔴",
                _ => "🟡",
            };

            let status_color = match service.status.as_str() {
                "running" | "started" => service.status.green(),
                "stopped" => service.status.red(),
                _ => service.status.yellow(),
            };

            let type_badge = match service.service_type.as_str() {
                "brew" => "[BREW]".magenta(),
                "launchd" => "[LAUNCHD]".cyan(),
                _ => "[UNKNOWN]".white(),
            };

            let pid_info = service
                .pid
                .as_ref()
                .map_or("".to_string(), |p| format!(" (PID: {p})"));

            println!(
                "{} {} {} - {}{}",
                status_icon,
                type_badge,
                service.name.bold(),
                status_color,
                pid_info.dimmed()
            );
        }

        println!("{}", "─".repeat(80).blue());
        println!(
            "{}",
            format!("📊 Total {} services listed", services.len()).bold()
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let service_manager = ServiceManager::new();

    if !service_manager.brew_available {
        println!(
            "{}",
            "⚠️  Brew not found. Only launchd services can be managed.".yellow()
        );
    }

    match cli.command {
        Commands::List { running, brew } => {
            let mut all_services = Vec::new();

            let launchd_services = service_manager.list_launchd_services(running).await?;
            all_services.extend(launchd_services);

            if brew && service_manager.brew_available {
                let brew_services = service_manager.list_brew_services(running).await?;
                all_services.extend(brew_services);
            }

            service_manager.print_services(&all_services);
        }
        Commands::Start { brew } => {
            service_manager.interactive_start_service(brew).await?;
        }
        Commands::Stop { brew } => {
            service_manager.interactive_stop_service(brew).await?;
        }
        Commands::Status { service, brew } => {
            service_manager.get_service_status(&service, brew).await?;
        }
    }

    Ok(())
}
