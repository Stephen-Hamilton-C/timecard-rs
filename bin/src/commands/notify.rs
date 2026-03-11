use std::{env::home_dir, fs, path::PathBuf};

use colored::Colorize;
use anyhow::Context;
use chrono::{DateTime, Datelike, Local};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use timecard::Timecard;
use clap::Args;

use crate::{config::Config, traits::{Loadable, Saveable}};


#[cfg(target_os = "linux")]
const SERVICE_UNIT: &str = include_str!("../../assets/timecard-notify.service");
#[cfg(target_os = "linux")]
const SERVICE_TIMER: &str = include_str!("../../assets/timecard-notify.timer");

#[derive(Args, Debug)]
pub struct NotifyArgs {
    /// Installs a notification service so a desktop notification appears when you reach the expected end time.
    /// This flag only works on Linux systemd systems
    #[arg(short, long, action)]
    install: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct NotifyData {
    last_notification: Option<DateTime<Local>>,
}

impl Loadable<NotifyData> for NotifyData {
    fn load(path: &PathBuf) -> anyhow::Result<NotifyData> {
        if fs::exists(path).unwrap_or(false) {
            let notify_data = fs::read_to_string(path)?;
            let notify: NotifyData = serde_json::from_str(&notify_data)
                .context("Failed to parse notify data")?;
            Ok(notify)
        } else {
            Ok(NotifyData { last_notification: None })
        }
    }
}

impl Saveable for NotifyData {
    fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let notify_data = serde_json::to_string(self)?;
        fs::write(path, notify_data)?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn install_notify_service() -> anyhow::Result<()> {
    use std::{env, process::Command, str::FromStr};
    use anyhow::bail;

    println!("Installing notification daemon...");
    let config_home = env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|v| !v.is_empty())
        .map(|v| PathBuf::from_str(&v).context("XDG_CONFIG_HOME appears to have a bad path"))
        .transpose()?
        .unwrap_or(home_dir().unwrap().join(".config"));
    let service_dir = config_home
        .join("systemd")
        .join("user");
    let exe_path = env::current_exe()
        .context("Failed to determine current executable path")?;
    let unit_path = service_dir.join("timecard-notify.service");
    let timer_path = service_dir.join("timecard-notify.timer");

    let unit_data = SERVICE_UNIT.replace("$TIMECARD_PATH", &exe_path.display().to_string());

    println!("{} {}", "CREATE".green(), unit_path.display());
    fs::write(&unit_path, unit_data)
        .context("Failed to create systemd unit service")?;
    println!("{} {}", "CREATE".green(), timer_path.display());
    fs::write(&timer_path, SERVICE_TIMER)
        .context("Failed to create systemd timer")?;

    println!("{} systemd --user daemon-reload", "RUN".cyan());
    let status = Command::new("systemctl")
        .arg("--user")
        .arg("daemon-reload")
        .status()?;

    if !status.success() {
        bail!("Failed to reload systemd daemon: {}", status);
    }

    println!("{} systemd --user enable --now timecard-notify.timer", "RUN".cyan());
    let status = Command::new("systemctl")
        .arg("--user")
        .arg("enable")
        .arg("--now")
        .arg("timecard-notify.timer")
        .status()?;

    if !status.success() {
        bail!("Failed to enable systemd daemon: {}", status);
    }

    println!("Notification daemon successfully installed!");

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn install_notify_service() -> anyhow::Result<()> {
    eprintln!("{}", "Notification service install is only supported on Linux systems running systemd.".yellow())
}

pub fn notify(args: &NotifyArgs, timecard: &Timecard, data_dir: &PathBuf) -> anyhow::Result<()> {
    if args.install {
        install_notify_service()?;
    } else {
        let config = Config::get();
        let notify_path = data_dir.join("notify.json");
        let last_notif = NotifyData::load(&notify_path)?.last_notification;
        let now = Local::now();

        // Check if user was already notified today
        if last_notif.is_none_or(|t| t.num_days_from_ce() != now.num_days_from_ce()) {
            let expected_end = timecard.get_expected_end_time(config.work_duration, &now);
            if let Some(end_time) = expected_end 
                && end_time.num_days_from_ce() == now.num_days_from_ce()
                && now >= end_time
            {
                // User has not yet been notified, and it is now past expected end time
                let notify_data = NotifyData {
                    last_notification: Some(now),
                };
                notify_data.save(&notify_path)?;

                // Send notification
                Notification::new()
                    .summary("Time to clock out!")
                    .body("It's been a full day of work")
                    .icon("clock")
                    // Urgency must be critical for notification to persist
                    .urgency(notify_rust::Urgency::Critical)
                    .timeout(0)
                    .show()
                    .context("Failed to send notification")?;
                println!("Sent notification at {}", now);
            }
        }
    }

    Ok(())
}
