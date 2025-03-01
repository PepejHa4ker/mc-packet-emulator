use crate::auth::join_auth_server;
use crate::connection::connection::Connection;
use crate::connection::connection_state::ConnectionState;
use crate::protocol::crypto::{encrypt_with_server_pubkey, generate_shared_secret};
use crate::protocol::fields::{ByteArrayShort, VarString};
use crate::protocol::packets::*;

impl ServerPacketHandler for Connection {
    async fn handle_login_disconnect(&mut self, packet: LoginDisconnect) {
        println!("Got login disconnect packet: {:?}", packet);
    }

    async fn handle_encryption_request(&mut self, packet: EncryptionRequest) {
        let server_id_str = &packet.server_id.0; 
        let shared_secret = generate_shared_secret();
        let encrypted_secret =
            encrypt_with_server_pubkey(&shared_secret, &packet.public_key.0).unwrap();
        let encrypted_token =
            encrypt_with_server_pubkey(&packet.verify_token.0, &packet.public_key.0).unwrap();

        let response = EncryptionResponse {
            shared_secret: ByteArrayShort(encrypted_secret),
            verify_token: ByteArrayShort(encrypted_token),
        };
        self.send_packet(&response).await;

        self.enable_encryption(&shared_secret).unwrap();
        println!("Encrypted!!!");

        let access_token = "";
        let selected_profile = "";

        match join_auth_server(
            server_id_str,
            &shared_secret,
            &packet.public_key.0,
            access_token,
            selected_profile,
        )
        .await
        {
            Ok(_) => println!("Successfully joined auth server!"),
            Err(e) => eprintln!("Failed to join auth server: {}", e),
        }
    }

    async fn handle_login_success(&mut self, packet: LoginSuccess) {
        self.state = ConnectionState::Play;
        println!(
            "Login success! username: {:?}, uuid: {:?}",
            &packet.username, &packet.uuid
        );
    }

    async fn handle_keep_alive(&mut self, packet: server::KeepAlive) {
        println!("Received keepalive");
        let c_keep_alive = client::KeepAlive {
            keep_alive_id: packet.keep_alive_id,
        };
        self.send_packet(&c_keep_alive).await;
    }

    async fn handle_join_game(&mut self, packet: JoinGame) {
        println!("Got joined game packet {:?}", &packet);
    }

    async fn handle_s_chat_message(&mut self, _packet: SChatMessage) {
        todo!()
    }

    async fn handle_time_update(&mut self, _packet: TimeUpdate) {
        todo!()
    }

    async fn handle_entity_equipment(&mut self, _packet: EntityEquipment) {
        todo!()
    }

    async fn handle_spawn_position(&mut self, _packet: SpawnPosition) {
        todo!()
    }

    async fn handle_update_health(&mut self, _packet: UpdateHealth) {
        todo!()
    }

    async fn handle_respawn(&mut self, _packet: Respawn) {
        todo!()
    }

    async fn handle_player_position_and_look(&mut self, _packet: PlayerPositionAndLook) {
        todo!()
    }

    async fn handle_held_item_change(&mut self, _packet: HeldItemChange) {
        todo!()
    }

    async fn handle_use_bed(&mut self, _packet: UseBed) {
        todo!()
    }

    async fn handle_animation(&mut self, _packet: Animation) {
        todo!()
    }

    async fn handle_spawn_player(&mut self, _packet: SpawnPlayer) {
        todo!()
    }

    async fn handle_collect_item(&mut self, _packet: CollectItem) {
        todo!()
    }

    async fn handle_spawn_object(&mut self, _packet: SpawnObject) {
        todo!()
    }

    async fn handle_spawn_mob(&mut self, _packet: SpawnMob) {
        todo!()
    }

    async fn handle_spawn_painting(&mut self, _packet: SpawnPainting) {
        todo!()
    }

    async fn handle_spawn_experience_orb(&mut self, _packet: SpawnExperienceOrb) {
        todo!()
    }

    async fn handle_entity_velocity(&mut self, _packet: EntityVelocity) {
        todo!()
    }

    async fn handle_destroy_entities(&mut self, _packet: DestroyEntities) {
        todo!()
    }

    async fn handle_entity(&mut self, _packet: Entity) {
        todo!()
    }

    async fn handle_entity_rel_move(&mut self, _packet: EntityRelMove) {
        todo!()
    }

    async fn handle_entity_look_and_movement(&mut self, _packet: EntityLookAndMovement) {
        todo!()
    }

    async fn handle_entity_look_move(&mut self, _packet: EntityLookMove) {
        todo!()
    }

    async fn handle_entity_teleport(&mut self, _packet: EntityTeleport) {
        todo!()
    }

    async fn handle_entity_status(&mut self, _packet: EntityStatus) {
        todo!()
    }

    async fn handle_attach_entity(&mut self, _packet: AttachEntity) {
        todo!()
    }

    async fn handle_entity_metadata(&mut self, _packet: EntityMetadata) {
        todo!()
    }

    async fn handle_entity_effect(&mut self, _packet: EntityEffect) {
        todo!()
    }

    async fn handle_remove_entity_effect(&mut self, _packet: RemoveEntityEffect) {
        todo!()
    }

    async fn handle_experience(&mut self, _packet: Experience) {
        todo!()
    }

    async fn handle_set_experience(&mut self, _packet: SetExperience) {
        todo!()
    }

    async fn handle_entity_properties(&mut self, _packet: EntityProperties) {
        todo!()
    }

    async fn handle_chunk_data(&mut self, _packet: ChunkData) {
        todo!()
    }

    async fn handle_multi_block_change(&mut self, _packet: MultiBlockChange) {
        todo!()
    }

    async fn handle_block_change(&mut self, _packet: BlockChange) {
        todo!()
    }

    async fn handle_map_chunk_bulk(&mut self, _packet: MapChunkBulk) {
        todo!()
    }

    async fn handle_explosion(&mut self, _packet: Explosion) {
        todo!()
    }

    async fn handle_effect(&mut self, _packet: Effect) {
        todo!()
    }

    async fn handle_sound_effect(&mut self, _packet: SoundEffect) {
        todo!()
    }

    async fn handle_change_game_state(&mut self, _packet: ChangeGameState) {
        todo!()
    }

    async fn handle_close_window(&mut self, _packet: CloseWindow) {
        todo!()
    }

    async fn handle_set_slot(&mut self, _packet: SetSlot) {
        todo!()
    }

    async fn handle_window_items(&mut self, _packet: WindowItems) {
        todo!()
    }

    async fn handle_update_tile_entity(&mut self, _packet: UpdateTileEntity) {
        todo!()
    }

    async fn handle_statistics(&mut self, _packet: Statistics) {
        todo!()
    }

    async fn handle_player_list_item(&mut self, _packet: PlayerListItem) {
        todo!()
    }

    async fn handle_player_abilities(&mut self, _packet: PlayerAbilities) {
        todo!()
    }

    async fn handle_custom_payload(&mut self, packet: server::CustomPayload) {
        if packet.channel.0 == "FML|HS" {
            println!("Received FML handshake payload: {:?}", packet.data.0);
            let response_channel = VarString("FML|HS".to_string());
            let response_data = ByteArrayShort(vec![0x00]);
            let response_packet = client::CustomPayload {
                channel: response_channel,
                data: response_data,
            };
            self.send_packet(&response_packet).await;
            println!("Sent FML handshake HELLO response");
        } else {
            println!("Received custom payload on channel: {:?}", packet.channel.0);
        }
    }
}
