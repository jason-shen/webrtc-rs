use anyhow::Result;
use clap::{AppSettings, Arg, Command};
use log::{debug, info};
use std::{io::Write, net::SocketAddr};
use warp::{serve, ws::Ws, Filter, Server};

use crate::{models::PeerConnections, server::start_ws};

mod models;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Command::new("sfu")
        .version("0.1.0")
        .author("Jason Shen <hello@jasonshen.co.nz>")
        .about("An example of a basic sfu.")
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand_negates_reqs(true)
        .arg(
            Arg::new("FULLHELP")
                .help("Prints more detail help information")
                .long("fullhelp"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Prints debug log information"),
        )
        .arg(
            Arg::new("address")
                .long("address")
                .default_value("127.0.0.1:6000")
                .short('a')
                .help("Prints address"),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("FULLHELP") {
        app.print_long_help().unwrap();
        std::process::exit(0);
    }

    let debug = matches.is_present("debug");

    if debug {
        env_logger::Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}:{} [{}] {} - {}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S.%6f"),
                    record.args()
                )
            })
            .filter(None, log::LevelFilter::Trace)
            .init();
    }
    // websockets setup
    let addr = matches.value_of("address").unwrap().parse::<String>()?;
    let socket_address: SocketAddr = addr.parse().expect("invalid socket address");
    let peer_connections = PeerConnections::default();
    let peer_connections = warp::any().map(move || peer_connections.clone());
    let ws_socket =
        warp::path("ws")
            .and(warp::ws())
            .and(peer_connections)
            .map(|ws: Ws, peer_connections| {
                ws.on_upgrade(move |socket| start_ws(socket, peer_connections))
            });
    let routers = ws_socket;
    let server = serve(routers).try_bind(socket_address);
    debug!("yes debug here");
    server.await;
    Ok(())
}
