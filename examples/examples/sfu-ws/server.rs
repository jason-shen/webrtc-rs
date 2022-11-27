use anyhow::Result;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use warp::ws::WebSocket;
use webrtc::{
    api::{media_engine::MediaEngine, APIBuilder},
    ice_transport::{ice_connection_state::RTCIceConnectionState, ice_server::RTCIceServer},
    interceptor::registry::Registry,
    media::track,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        RTCPeerConnection,
    },
    rtp_transceiver::rtp_codec::RTPCodecType,
    track::track_local::TrackLocal,
    Error,
};

use crate::models::{PeerConnectionState, PeerConnections};

pub async fn start_ws(ws: WebSocket, peer_connections: PeerConnections) -> Result<()> {
    info!("websocket started!!");
    let peer_connection = Arc::new(new_peer_connection().await?);
    peer_connection
        .add_transceiver_from_kind(RTPCodecType::Video, &[])
        .await;
    peer_connection
        .add_transceiver_from_kind(RTPCodecType::Audio, &[])
        .await;
    // add our new peerConnection to our PeerConnections list

    Ok(())
}

async fn new_peer_connection() -> Result<RTCPeerConnection, webrtc::Error> {
    let mut media_engine = MediaEngine::default();
    media_engine.register_default_codecs()?;
    let registry = Registry::new();

    let api = APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(registry)
        .build();
    // Prepare the configuration
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };
    api.new_peer_connection(config).await
}

async fn singal_peer_connections(peer_connections: &mut Vec<PeerConnectionState>) -> Result<bool> {
    for (index, peer) in peer_connections.iter().enumerate() {
        if peer.peer_connection.connection_state() == RTCPeerConnectionState::Closed {
            peer_connections.remove(index);
            return Ok(true);
        }
        // map of sender we already are seanding, so we don't double send
        let mut existing_sender = HashMap::<String, bool>::new();
        for sender in peer.peer_connection.get_senders().await {
            if let Some(track) = sender.track().await {
                let track_id: &str = track.id().as_ref();
                if track_id.is_empty() {
                    continue;
                }
                existing_sender.insert(track_id.to_owned(), true);

                // If we have a RTPSender that doesn't map to a existing track remove and signal
                for (i, _) in peer.local_tracks.iter().enumerate() {
                    if peer.local_tracks.get(i).is_none() {
                        let _ = peer.peer_connection.remove_track(&sender).await;
                        continue;
                    }
                    return Ok(true);
                }
            }
        }
        // Don't receive videos we are sending, make sure we don't have loopback
        for receiver in peer.peer_connection.get_receivers().await {
            if let Some(track) = receiver.track().await {
                let track_id: String = track.id().await;
                if track_id.is_empty() {
                    continue;
                }
                existing_sender.insert(track_id.to_owned(), true);
            }
        }
        // Add all track we aren't sending yet to the PeerConnection
        for (i, local_track) in peer.local_tracks.iter().enumerate() {
            if peer.local_tracks.get(i).is_none() {
                let _ = peer
                    .peer_connection
                    .add_track(local_track.clone() as Arc<dyn TrackLocal + Send + Sync>)
                    .await;
            }
        }

        let offer = peer.peer_connection.create_offer(None).await?;
        peer.peer_connection.set_local_description(offer).await?;
    }
    Ok(false)
}
