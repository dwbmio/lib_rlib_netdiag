use log::info;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let ret = tracer_netdiagnostics::ping_allhost(vec!["www.baidu.com"], Some(3)).await;
        println!("test result is :{:?}", ret);
        info!("tett comment in test ");
    })
}
