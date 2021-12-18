use common::serializable;

serializable! {
    pub enum ClientMessage {
        Execute { command: Vec<String> },
    }

    pub enum ServerMessage {
        Notification { message: String },
        Role { title: String, allowed_commands: Vec<String> },
        FilesList { files: Vec<String> },
        UsersList { users: Vec<(String, String)> },
        MoveTo { location: String },
        KillResult { killed_users_count: u32 }
    }
}
