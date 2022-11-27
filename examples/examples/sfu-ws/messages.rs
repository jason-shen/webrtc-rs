use serde::{Deserialize, Serialize};
use webrtc::{
    ice_transport::ice_candidate::RTCIceCandidateInit,
    peer_connection::sdp::session_description::RTCSessionDescription,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RequestMessage {
    #[serde(rename_all = "camelCase")]
    JoinRoom { room_name: String },
    #[serde(rename_all = "camelCase")]
    Candidate { data: RTCIceCandidateInit },
    #[serde(rename_all = "camelCase")]
    Answer { data: RTCSessionDescription },
    #[serde(rename_all = "camelCase")]
    Ping,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ResponseMessage {
    #[serde(rename_all = "camelCase")]
    Offer { data: RTCSessionDescription },
    #[serde(rename_all = "camelCase")]
    Pong,
}
