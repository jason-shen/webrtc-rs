use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use uuid::Uuid;
use warp::{ws::Message, Error};
use webrtc::peer_connection::RTCPeerConnection;

#[derive(Debug, Default, Clone)]
pub struct PeerConnections(HashMap<String, Vec<PeerConnectionState>>);
impl PeerConnections {
    pub fn new_peer(
        &mut self,
        room_name: String,
        tx: Arc<RwLock<UnboundedSender<Result<Message, Error>>>>,
        peer_connection: Arc<RwLock<RTCPeerConnection>>,
    ) {
        let new_peer = PeerConnectionState {
            id: Uuid::new_v4(),
            websocket: tx,
            peer_connection,
        };
        self.0.entry(room_name).or_default().push(new_peer)
    }
    pub fn get_peers(&self, room_id: String) -> Option<Vec<PeerConnectionState>> {
        self.0.get(&room_id).map(|ps| ps.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PeerConnectionState {
    pub id: Uuid,
    pub websocket: Arc<RwLock<mpsc::UnboundedSender<Result<Message, warp::Error>>>>,
    pub peer_connection: Arc<RwLock<RTCPeerConnection>>,
}

// impl Clone for PeerConnectionState {
//     fn clone(&self) -> PeerConnectionState {
//         Self {
//             websocket: self.websocket,
//             peer_connection: self.peer_connection,
//         }
//     }
// }
