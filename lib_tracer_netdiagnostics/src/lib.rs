/*
@Author:bbclient
 */
pub mod android_api;
pub mod config;
pub mod ios_api;
use futures::future::join_all;
use log::{error, info};
use std::net::IpAddr;
use std::{io, time::Duration};
use surge_ping::{Client, IcmpPacket, PingIdentifier, PingSequence};
use thiserror::Error;

#[derive(Error, Debug)]
enum NetDiagError {
    #[error("ping bind loc failed!")]
    BindError(#[from] surge_ping::SurgeError),

    #[error("ping bind loc failed2!")]
    Bind2Error(#[from] io::Error),
}

type NetDiagResult = Result<String, NetDiagError>;

// Ping an address 5 times， and print output message（interval 1s）
async fn ping(client: Client, addr: IpAddr, ping_times: u16) {
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(rand::random())).await;
    pinger.timeout(Duration::from_secs(1));
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for idx in 0..ping_times {
        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {
            Ok((IcmpPacket::V4(packet), dur)) => println!(
                "No.{}: {} bytes from {}: icmp_seq={} ttl={:?} time={:0.2?}",
                idx,
                packet.get_size(),
                packet.get_source(),
                packet.get_sequence(),
                packet.get_ttl(),
                dur
            ),
            Ok((IcmpPacket::V6(packet), dur)) => println!(
                "No.{}: {} bytes from {}: icmp_seq={} hlim={} time={:0.2?}",
                idx,
                packet.get_size(),
                packet.get_source(),
                packet.get_sequence(),
                packet.get_max_hop_limit(),
                dur
            ),
            Err(e) => println!("No.{}: {} ping {}", idx, pinger.host, e),
        };
    }
    println!("[+] {} done.", pinger.host);
}

async fn ping_allhost(host_list: Vec<&str>, ping_tims: Option<u16>) -> NetDiagResult {
    let v4_cli = Client::new(&surge_ping::Config::default())?;
    let v6_cli = Client::new(
        &surge_ping::Config::builder()
            .kind(surge_ping::ICMP::V6)
            .build(),
    )?;
    let mut tasks = Vec::new();

    for h in host_list {
        match h.parse() {
            Ok(IpAddr::V4(h)) => {
                tasks.push(tokio::spawn(ping(
                    v4_cli.clone(),
                    IpAddr::V4(h),
                    ping_tims.unwrap_or(2),
                )));
            }
            Ok(IpAddr::V6(h)) => {
                tasks.push(tokio::spawn(ping(
                    v6_cli.clone(),
                    IpAddr::V6(h),
                    ping_tims.unwrap_or(2),
                )));
            }
            Err(e) => {
                println!("{} parse to ipaddr error: {}", h, e)
            }
        }
    }
    join_all(tasks).await;
    info!("test multi_ping");
    Ok("()".to_owned())
}
