use crate::polocloud::player_controller_client::PlayerControllerClient;
use crate::polocloud::{
    PlayerConnectActorRequest, PlayerFindByNameRequest, PlayerFindByServiceRequest,
    PlayerFindByUniqueIdRequest, PlayerKickActorRequest, PlayerMessageActorRequest,
};
use crate::proto::polo_player::PolocloudPlayer;
use tonic::transport::Channel;
use tonic::{Request, Status};

pub struct PlayerProvider {
    player_stub: PlayerControllerClient<Channel>,
}
impl PlayerProvider {
    pub fn new(channel: Channel) -> Self {
        Self {
            player_stub: PlayerControllerClient::new(channel),
        }
    }

    pub async fn find_all_async(&mut self) -> Result<Vec<PolocloudPlayer>, Status> {
        let response = self.player_stub.find_all(Request::new(())).await?;

        let players = response
            .into_inner()
            .players
            .into_iter()
            .map(PolocloudPlayer::from_snapshot)
            .collect();

        Ok(players)
    }

    pub async fn find_by_name_async(
        &mut self,
        name: String,
    ) -> Result<Option<PolocloudPlayer>, Status> {
        let request = Request::new(PlayerFindByNameRequest { name });
        let response = self.player_stub.find_by_name(request).await?;

        let player_option = response
            .into_inner()
            .players
            .into_iter()
            .map(PolocloudPlayer::from_snapshot)
            .nth(0);

        Ok(player_option)
    }

    pub async fn find_by_service_async(
        &mut self,
        service_name: String,
    ) -> Result<Vec<PolocloudPlayer>, Status> {
        let request = Request::new(PlayerFindByServiceRequest {
            current_service_name: service_name,
        });
        let response = self.player_stub.find_by_service(request).await?;

        let players = response
            .into_inner()
            .players
            .into_iter()
            .map(PolocloudPlayer::from_snapshot)
            .collect();

        Ok(players)
    }

    pub async fn find_by_unique_id_async(
        &mut self,
        unique_id: String,
    ) -> Result<Option<PolocloudPlayer>, Status> {
        let request = Request::new(PlayerFindByUniqueIdRequest { unique_id });
        let response = self.player_stub.find_by_unique_id(request).await?;

        let player_option = response
            .into_inner()
            .players
            .into_iter()
            .map(PolocloudPlayer::from_snapshot)
            .nth(0);

        Ok(player_option)
    }

    pub async fn player_count_async(&mut self) -> Result<i32, Status> {
        let response = self.player_stub.player_count(Request::new(())).await?;
        Ok(response.into_inner().count)
    }

    pub async fn message_player_async(
        &mut self,
        unique_id: String,
        message: String,
    ) -> Result<bool, Status> {
        let request = Request::new(PlayerMessageActorRequest { unique_id, message });
        let response = self.player_stub.message_player(request).await?;
        Ok(response.into_inner().success)
    }

    pub async fn kick_player_async(
        &mut self,
        unique_id: String,
        reason: String,
    ) -> Result<bool, Status> {
        let request = Request::new(PlayerKickActorRequest { unique_id, reason });
        let response = self.player_stub.kick_player(request).await?;
        Ok(response.into_inner().success)
    }

    pub async fn connect_player_async(
        &mut self,
        unique_id: String,
        target_service_name: String,
    ) -> Result<bool, Status> {
        let request = Request::new(PlayerConnectActorRequest {
            unique_id,
            target_service_name,
        });
        let response = self.player_stub.connect_player(request).await?;
        Ok(response.into_inner().success)
    }
}
