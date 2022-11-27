use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use uuid::Uuid;
use warp::{ws::Message, Error};
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::{peer_connection::RTCPeerConnection, track::track_local::TrackLocal};

#[derive(Debug, Default, Clone)]
pub struct PeerConnections(HashMap<String, Vec<PeerConnectionState>>);
impl PeerConnections {
    pub fn new_peer(
        &mut self,
        room_name: String,
        tx: Arc<RwLock<UnboundedSender<Result<Message, Error>>>>,
        peer_connection: Arc<RTCPeerConnection>,
        local_tracks: Vec<Arc<TrackLocalStaticRTP>>,
    ) {
        let new_peer = PeerConnectionState {
            id: Uuid::new_v4(),
            websocket: tx,
            peer_connection,
            local_tracks,
        };
        self.0.entry(room_name).or_default().push(new_peer)
    }
    pub fn get_peers(&self, room_name: String) -> Option<Vec<PeerConnectionState>> {
        self.0.get(&room_name).map(|ps| ps.clone())
    }
    pub fn get_local_track(&self, room_name: String, track_id: Uuid) -> bool {
        match self.get_peers(room_name) {
            Some(p) => p.into_iter().find(|t| t.id == track_id).is_some(),
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerConnectionState {
    pub id: Uuid,
    pub websocket: Arc<RwLock<mpsc::UnboundedSender<Result<Message, warp::Error>>>>,
    pub peer_connection: Arc<RTCPeerConnection>,
    pub local_tracks: Vec<Arc<TrackLocalStaticRTP>>,
}

// impl Clone for PeerConnectionState {
//     fn clone(&self) -> PeerConnectionState {
//         Self {
//             websocket: self.websocket,
//             peer_connection: self.peer_connection,
//         }
//     }
// }
