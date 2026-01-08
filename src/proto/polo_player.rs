use crate::polocloud::PlayerSnapshot;

#[derive(Debug, Clone)]
pub struct PolocloudPlayer {
    name: String,
    uuid: String,
    current_server_name: String,
    current_proxy_name: String,
    first_time_joined: i64,
}

impl PolocloudPlayer {
    pub fn new(
        name: String,
        uuid: String,
        current_server_name: String,
        current_proxy_name: String,
        first_time_joined: i64,
    ) -> Self {
        Self {
            name,
            uuid,
            current_server_name,
            current_proxy_name,
            first_time_joined,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn current_server_name(&self) -> &str {
        &self.current_server_name
    }

    pub fn current_proxy_name(&self) -> &str {
        &self.current_proxy_name
    }

    pub fn first_time_joined(&self) -> i64 {
        self.first_time_joined
    }

    pub fn from_snapshot(snapshot: PlayerSnapshot) -> Self {
        Self {
            name: snapshot.name,
            uuid: snapshot.unique_id,
            current_server_name: snapshot.current_server_name,
            current_proxy_name: snapshot.current_proxy_name,
            first_time_joined: snapshot.first_time_joined,
        }
    }
}
