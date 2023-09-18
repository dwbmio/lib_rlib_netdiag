#[macro_use]
extern crate log;

pub mod android_api;
pub mod ios_api;
use log::error;
use std::net::IpAddr;
use std::{io, time::Duration};
use surge_ping::{Client, IcmpPacket, PingIdentifier, PingSequence};
use thiserror::Error;
use trust_dns_resolver::{config::*, Resolver};
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Error, Debug)]
pub enum NetDiagError {
    #[error("ping bind loc failed!")]
    BindError(#[from] surge_ping::SurgeError),

    #[error("ping bind loc failed2!")]
    Bind2Error(#[from] io::Error),

    #[error("dns failed!")]
    ResolveError(#[from] trust_dns_resolver::error::ResolveError),
}

type NetDiagResult = Result<String, NetDiagError>;
type NetDiaglistResult = Result<Vec<String>, NetDiagError>;

///domain查询远端ip
/// ```
/// use tokio::runtime::Runtime;
///
/// let rt = Runtime::new().unwrap();
/// rt.block_on(aysnc {
///     let ip = dns_ip("www.baidu.com").await;
///     
/// });
///
///
///
/// ```
async fn dns_ip(domain_addr: &str) -> Result<Option<String>, NetDiagError> {
    // Construct a new Resolver with default configuration options
    trace!("dns domain <{}> start...", domain_addr);


    let (config, options) = super::system_conf::read_system_conf()?;
    let resover= Resolver::from_system_conf();

    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())?;
    trace!("resolver suc!");
    let ip = resolver.lookup_ip(domain_addr).await?;
    let ip_str = ip.iter().next().and_then(|f| Some(f.to_string()));
    Ok(ip_str)
}

// Ping an address $ping_tims times， and return output message（interval 1s）
async fn ping(client: Client, addr: IpAddr, ping_times: u16) -> NetDiagResult {
    info!("ping <{}> start...", addr.to_string());
    let mut ping_ret = String::new();
    let payload: [u8; 56] = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(rand::random())).await;
    pinger.timeout(Duration::from_secs(2));
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for idx in 0..ping_times {
        interval.tick().await;
        match pinger.ping(PingSequence(idx), &payload).await {
            Ok((IcmpPacket::V4(packet), dur)) => {
                let r = format!(
                    "No.{}: {} bytes from {}: icmp_seq={} ttl={:?} time={:0.2?}\n",
                    idx,
                    packet.get_size(),
                    packet.get_source(),
                    packet.get_sequence(),
                    packet.get_ttl(),
                    dur
                );
                log::debug!("ipv4 value is :{:?}", r);
                ping_ret += &r;
            }

            Ok((IcmpPacket::V6(packet), dur)) => {
                let r = format!(
                    "No.{}: {} bytes from {}: icmp_seq={} hlim={} time={:0.2?}\n",
                    idx,
                    packet.get_size(),
                    packet.get_source(),
                    packet.get_sequence(),
                    packet.get_max_hop_limit(),
                    dur
                );
                log::debug!("ipv6 value is :{:?}", r);
                ping_ret += &r;
            }
            Err(e) => {
                let r: String = format!("idx = {}  error:{}\n", idx, e.to_string());
                log::debug!("surge-error value is :{:?}", r);
                ping_ret += &r;
            }
        };
    }
    Ok(format!("{} \n[+] {} done.", ping_ret, pinger.host))
}

///
/// ping一组目标地址，获取目标地址的联通性
/// 优先匹配ip, 匹配ip失败尝试dns解domain看是否有可用ip，还没有就直接标记地址ping失败
///
/// ```
/// use log::{info};
/// use tracer_netdiagnostics::ping_allhost;
/// use tokio::runtime::Runtime;
///
/// let rt = Runtime::new().unwrap();
/// rt.block_on(async {
/// let ret = ping_allhost(vec!["10.250.15.60"], Some(3)).await;
/// println!("test result is :{:?}", ret);
/// info!("tett comment in test ");
/// });
/// ```
pub async fn ping_allhost(host_list: Vec<&str>, ping_tims: Option<u16>) -> NetDiaglistResult {
    let v4_cli = Client::new(&surge_ping::Config::default())?;
    let v6_cli = Client::new(
        &surge_ping::Config::builder()
            .kind(surge_ping::ICMP::V6)
            .build(),
    )?;
    let mut tasks = Vec::new();
    trace!("test task start...");
    for h in host_list {
        match h.parse() {
            Ok(IpAddr::V4(h)) => {
                debug!("v4 start...");
                tasks.push(ping(v4_cli.clone(), IpAddr::V4(h), ping_tims.unwrap_or(2)));
            }
            Ok(IpAddr::V6(h)) => {
                debug!("v6 start...");
                tasks.push(ping(v6_cli.clone(), IpAddr::V6(h), ping_tims.unwrap_or(2)));
            }
            Err(_e) => {
                debug!("domain");
                //try as dns like to get ip first.
                let ip = dns_ip(h).await?;
                if let Some(ip_str) = ip {
                    trace!("lookup is value is {:?}", ip_str);
                    tasks.push(ping(
                        v4_cli.clone(),
                        ip_str.parse().expect("SYNTACTIC-SUGAR"),
                        ping_tims.unwrap_or(2),
                    ));
                }
            }
        }
    }
    let mut ping_result: Vec<String> = vec![];
    let mut ret = futures::future::join_all(tasks).await;

    for el in ret.iter_mut() {
        if let Ok(e) = el {
            ping_result.push(e.to_owned());
            continue;
        }
        ping_result.push("ping failed!".to_owned());
    }

    Ok(ping_result)
}
