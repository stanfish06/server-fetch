#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;
use std::fmt;
use std::fs;
use std::str;
use std::ffi::CStr;
use std::ffi::c_void;
use chrono::{DateTime, Utc, TimeZone};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/*---------------
user information:
- user name 
- login time 
- groups
- kernel version
- hostname
- CPU
- memory
- ...
---------------*/
// wtmpdb_get_boottime
pub struct UserInfo {
    user_name: String,
    user_groups: Vec<String>,
    boot_time: u64
}

impl UserInfo {
    fn fetch_groups(&mut self) {
        let user_name = &self.user_name;
        let file_content = fs::read_to_string("/etc/group").unwrap();
        let mut user_groups: Vec<String> = Vec::new();
        for line in file_content.lines() {
            let cols: Vec<&str> = line.split(":").collect();
            if cols.iter().any(|&item| user_name == item) {
                user_groups.push(String::from(cols[0]));
            }
        }
        self.user_groups = user_groups;
    }

    pub fn new() -> UserInfo {
        let user_name = match env::var("HOME") {
            Ok(val) => val.split("/").collect::<Vec<&str>>().last().map(|&s| String::from(s)).unwrap_or_else(|| String::from("none")),
            Err(e) => String::from("none")
        };
        let user_groups = Vec::new();
        let mut user_info = UserInfo {
            user_name: user_name,
            user_groups: user_groups,
            boot_time: 0
        };
        user_info.fetch_groups();
        user_info
    }
}

impl fmt::Display for UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // print user name
        let num_chars = self.user_name.len();
        let num_padding = 1;
        let width = num_chars + num_padding * 2;
        writeln!(f, "┌{}┐", "─".repeat(width))?;
        writeln!(f, "│{:^width$}│", self.user_name, width = width)?;
        writeln!(f, "└{}┘", "─".repeat(width))?;
        // print user groups
        writeln!(f, "┌{}┐", "─".repeat(width))?;
        writeln!(f, "│{:^width$}│", "Groups", width = width)?;
        writeln!(f, "│{}│", "─".repeat(width))?;
        for group in &self.user_groups {
            writeln!(f, "│{:<width$}│", group, width = width)?;
        }
        writeln!(f, "└{}┘", "─".repeat(width))?;
        writeln!(f, "┌{}┐", "─".repeat(41))?;
        writeln!(f, "│{:^width$}│", "Recent logins", width = 41)?;
        writeln!(f, "│{}│", "─".repeat(41))?;
        // login
        unsafe{ 
            let mut entries: *mut Entry = std::ptr::null_mut();
            let mut data = wtmpdb_data {
                count: 0,
                capacity: 0,
                entries,
            };
            get_login_info(&mut data as *mut wtmpdb_data);
            let mut ct: usize = 0;
            for i in 0..data.count {
                let entry = *data.entries.offset(i as isize);
                if !entry.user.is_null() {
                    let _user = CStr::from_ptr(entry.user as *const i8);
                    let user_name: &str = _user.to_str().unwrap();
                    // only show 5 most recent logins for now
                    if user_name == self.user_name {
                        if ct < 5 {
                            let login_time = (entry.login / 1_000_000) as i64;
                            let logout_time = (entry.logout / 1_000_000) as i64;
                            let login_time_h = Utc.timestamp_opt(login_time, 0).unwrap().format("%Y-%m-%d %H:%M:%S");
                            // 0 means still logged in, UINT64_MAX - 1 means crash
                            if logout_time == 0 {
                                writeln!(f, "│{:<width$}│", format!("{} - active", login_time_h), width=41)?;
                            } else if entry.logout == u64::MAX - 1 {
                                writeln!(f, "│{:<width$}│", format!("{} - crash", login_time_h), width=41)?;
                            } else {
                                let logout_time_h = Utc.timestamp_opt(logout_time, 0).unwrap().format("%Y-%m-%d %H:%M:%S");
                                writeln!(f, "│{:<width$}│", format!("{} - {}", login_time_h, logout_time_h), width=41)?;
                            }
                        }
                        ct = ct + 1;
                    }
                }
            }
            writeln!(f, "│{}│", "─".repeat(41))?;
            writeln!(f, "│{:^width$}│", format!("{} total logins", ct), width=41)?;

            for i in 0..data.count {
                let entry = *data.entries.offset(i as isize);
                if !entry.user.is_null() {
                    free(entry.user as *mut c_void);
                }
            }

            if !data.entries.is_null() {
                free(data.entries as *mut c_void);
            }
        };
        writeln!(f, "└{}┘", "─".repeat(41))?;
        Ok(())
    }
}
