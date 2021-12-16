use std::fs::{File};
use std::collections::{HashMap};

use common::helpers::{misconfiguration};
use common::serializable;
use common::{Result};

use serde::{Serialize, Deserialize};

serializable! {
    pub struct UserSettings {
        pub role: String,
        pub pass: String,
    }

    pub struct Members {
        roles: HashMap<String, Vec<String>>,
        users: HashMap<String, UserSettings>,
    }
}

#[derive(Clone, Debug)]
pub struct Role {
    pub title: String,
    pub allowed_commands: Vec<String>,
}

impl Members {
    pub fn role(&self, role: &str) -> Result<Role> {
        if !self.roles.contains_key(role) {
            return misconfiguration(&format!("No such a role > {}", role))
        }

        let it = Role {
            title: role.to_owned(),
            allowed_commands: self.roles[role].clone(),
        };

        Ok(it)
    }

    pub fn has_user(&self, user: &str) -> bool {
        self.users.contains_key(user)
    }

    pub fn settings_for(&self, user: &str) -> Result<UserSettings> {
        if !self.has_user(user) {
            return misconfiguration(&format!("No such user > {}", user))
        }

        let settings = self.users[user].clone();

        Ok(settings)
    }

    pub fn role_for(&self, user: &str) -> Result<Role> {
        let settings = self.settings_for(user)?;
        self.role(&settings.role)
    }
}

const MEMBERS_FILE: &'static str = "members.json";

pub fn load_members() -> Result<Members> {
    let mut file = File::open(MEMBERS_FILE)?;
    let it: Members = serde_json::from_reader(&mut file)?;
    Ok(it)
}

pub fn save_members(members: &Members) -> Result<()> {
    let mut file = File::create(MEMBERS_FILE)?;
    serde_json::to_writer_pretty(&mut file, members)?;
    Ok(())
}
