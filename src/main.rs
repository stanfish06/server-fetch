use crate::user::UserInfo;
mod user;

fn main() {
    let user_info = UserInfo::new();
    println!("{}", user_info.to_string());
}
