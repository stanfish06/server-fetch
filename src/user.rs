use std::env;
use std::fmt;
use std::fs;

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
pub struct UserInfo {
    user_name: String,
    user_groups: Vec<String>
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
            user_groups: user_groups
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
        writeln!(f, "┌{}┐", "-".repeat(width))?;
        writeln!(f, "|{:^width$}|", self.user_name, width = width)?;
        writeln!(f, "└{}┘", "-".repeat(width))?;
        // print user groups
        writeln!(f, "┌{}┐", "-".repeat(width))?;
        writeln!(f, "|{:^width$}|", "Groups", width = width)?;
        writeln!(f, "|{}|", "-".repeat(width))?;
        for group in &self.user_groups {
            writeln!(f, "|{:<width$}|", group, width = width)?;
        }
        writeln!(f, "└{}┘", "-".repeat(width))
    }
}
