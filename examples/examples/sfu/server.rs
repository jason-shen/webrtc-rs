use log::info;
use warp::ws::WebSocket;

use crate::models::PeerConnections;

pub async fn start_ws(ws: WebSocket, peer_connections: PeerConnections) {
    info!("websocket started!!");
}
