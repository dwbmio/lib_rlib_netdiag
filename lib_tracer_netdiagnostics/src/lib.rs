/*
@Author:bbclient
 */
pub mod android_api;
pub mod config;
pub mod ios_api;
use log::{error, info};
use std::net::IpAddr;
use std::{io, time::Duration};
use surge_ping::{Client, IcmpPacket, PingIdentifier, PingSequence};
use thiserror::Error;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

#[derive(Error, Debug)]
enum NetDiagError {
    #[error("ping bind loc failed!")]
    BindError(#[from] surge_ping::SurgeError),

    #[error("ping bind loc failed2!")]
    Bind2Error(#[from] io::Error),

    #[error("dnt failed!")]
    ResolveError(#[from] trust_dns_resolver::error::ResolveError),
}

type NetDiagResult = Result<String, NetDiagError>;

fn dns_ip(domain_addr: &str) -> Result<Option<String>, NetDiagError> {
    // Construct a new Resolver with default configuration options
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    let ip = resolver.lookup_ip(domain_addr)?;
    let ip_str = ip.iter().next().and_then(|f| Some(f.to_string()));
    Ok(ip_str)
}

// Ping an address $ping_tims times， and return output message（interval 1s）
async fn ping(client: Client, addr: IpAddr, ping_times: u16) -> NetDiagResult {
    let mut ping_ret = String::new();
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(rand::random())).await;
    pinger.timeout(Duration::from_secs(1));
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for idx in 0..ping_times {
        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {
            Ok((IcmpPacket::V4(packet), dur)) => {
                let r = format!(
                    "No.{}: {} bytes from {}: icmp_seq={} ttl={:?} time={:0.2?}",
                    idx,
                    packet.get_size(),
                    packet.get_source(),
                    packet.get_sequence(),
                    packet.get_ttl(),
                    dur
                );
                ping_ret += &r;
            }

            Ok((IcmpPacket::V6(packet), dur)) => {
                let r = format!(
                    "No.{}: {} bytes from {}: icmp_seq={} hlim={} time={:0.2?}",
                    idx,
                    packet.get_size(),
                    packet.get_source(),
                    packet.get_sequence(),
                    packet.get_max_hop_limit(),
                    dur
                );
                ping_ret += &r;
            }
            Err(e) => {
                let r = format!("No.{}: host ping raise error={}", idx, e.to_string());
                ping_ret += &r;
            }
        };
    }
    println!("ping result is :{:?}", ping_ret);
    Ok(format!("[+] {} done.", pinger.host))
}

///
/// ping一组目标地址，获取目标地址的联通性
/// 优先匹配ip, 匹配ip失败尝试dns解domain看是否有可用ip，还没有就直接标记地址ping失败
///
/// ```rust
/// let c = 1;
/// info!("tett comment in test ")
/// ```
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
            Err(_e) => {
                //try as dns like to get ip first.
                let ip = dns_ip(h)?;
                if let Some(ip_str) = ip {
                    info!("lookup is value is {:?}", ip_str);
                    tasks.push(tokio::spawn(ping(
                        v4_cli.clone(),
                        ip_str.parse().expect("SYNTACTIC-SUGAR"),
                        ping_tims.unwrap_or(2),
                    )));
                }
            }
        }
    }
    futures::future::join_all(tasks).await;
    Ok("()".to_owned())
}

#[test]
fn test_dnsdomain() {
    let ip = dns_ip("www.baidu.com");
    println!("dns domain result  is :{:?}", ip);
}

#[tokio::test]
async fn test_pinghost() {
    let ret = ping_allhost(vec!["10.250.15.60"], Some(3)).await;
    println!("test result is :{:?}", ret);
}
