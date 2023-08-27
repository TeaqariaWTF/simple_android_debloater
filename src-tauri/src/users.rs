use crate::adb_cmd::{ADBCommand, ADBShell};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub device_id: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user {}", self.id)
    }
}

pub trait ListUsers {
    fn list_users(&self, device_id: String) -> Result<Vec<User>>;
}

pub struct ADBTerminalImpl {
    pub adb_path: String
}

impl ADBTerminalImpl {
    pub fn list_users(&self, device_id: String) -> Result<Vec<User>> {
        let shell_cmd: ADBShell =
        ADBShell::new(self.adb_path.to_owned()).for_device(device_id.to_owned());

        let res = shell_cmd.with_commands(&["pm list users "]).execute();
        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(o) => {
                let re = Regex::new(r"UserInfo\{(.*)\}").unwrap();

                let mut users: Vec<User> = vec![];
                for (_, [cap]) in re.captures_iter(&o).map(|c| c.extract()) {
                    let split: Vec<&str> = cap.split(":").collect();
                    if split.len() < 2 {
                        return Err(anyhow!("unable to parse user. input {}", cap));
                    }
                    users.push(User {
                        id: split[0].to_string(),
                        name: split[1].to_string(),
                        device_id: device_id.to_owned(),
                    })
                }
                return Ok(users);
            }
        }
    }
}
