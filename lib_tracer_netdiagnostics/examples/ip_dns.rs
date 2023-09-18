use lib_tracer_netdiagnostics::ping_allhost;
use log::info;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let ret = ping_allhost(vec!["10.250.15.60"], Some(3)).await;
        println!("test result is :{:?}", ret);
        info!("tett comment in test ");
    })
}
