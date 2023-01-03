use std::fs;
use std::ops::Add;
use std::time::{Duration, SystemTime};

use chrono::Local;
use job_scheduler::{Job, JobScheduler};
use sysinfo::{DiskExt, System, SystemExt};

fn info(message: String) {
    let time = Local::now().format("%d-%m-%Y %H:%M:%S");
    println!("{} INFO {}", time, message);
}

fn error(message: String) {
    let time = Local::now().format("%d-%m-%Y %H:%M:%S");
    println!("{} ERROR {}", time, message);
}

const DURATION_7_DAYS: u64 = 7 * 24 * 60 * 60;

// CRON JOB
// sec   min   hour   day of month   month   day of week   year
// *     *     *      *              *       *             *

fn main() {
    info(String::from("📅 Scheduler is running..."));
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // Update all information of our `System` struct.
    sys.refresh_all();


    // Create scheduler
    let mut sched = JobScheduler::new();

    let clean_disks = || {
        info(String::from("🔍 Calculating a free space..."));

        // take all disks
        for disk in sys.disks() {
            let available_space = disk.available_space() / 1_000_000_000; // bytes to Gb
            let disk_name = disk.mount_point();

            info(format!("💽 Available free space on disc {:?}: {:?}", disk_name, available_space));

            // FOR Disc C && free space less then 5 GB
            if disk.mount_point().starts_with("C://") && available_space < 5 {
                info(String::from("🧹 Folder TMP is being cleaned..."));

                // paths contain Iterator
                let paths = match fs::read_dir("C:/Users/sevri/AppData/Local/Temp/test") {
                    Ok(p) => p,
                    Err(e) => {
                        error(e.to_string());
                        return;
                    }
                };

                for path in paths {
                    let p = path.unwrap();
                    let file_created = p.path().metadata().unwrap().created().unwrap();

                    if file_created < SystemTime::now().add(Duration::from_secs(DURATION_7_DAYS)) {
                        match fs::remove_file(p.path()) {
                            Ok(_) => {}
                            Err(e) => {
                                error(e.to_string());
                                return;
                            }
                        };
                    }
                }

                info(String::from("🍻 Folder TMP has been cleaned."));
            }
            // Disk C end
        }
    };

    // Add new cron job
    // Check free space every hour
    sched.add(Job::new("* 0 * * * *".parse().unwrap(), clean_disks));

    loop {
        sched.tick();
    }
}