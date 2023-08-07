#[cfg(any(target_os = "ios"))]
pub mod ios {
    use std::ffi::{c_char, CStr, CString};
    use std::io::Write;

    pub fn init_logger() {
        const LOG_LV_TAG:&str = "LIB_NETDIAG_LOG";
        let lv = std::env::var(LOG_LV_TAG).map_or("INFO".to_owned(), |f| f);
        std::env::set_var(LOG_LV_TAG, lv);
        env_logger::Builder::from_env(LOG_LV_TAG)
            .format(|b, record| writeln!(b, "[lib-netdiag]{}", record.args()));
    }

    #[no_mangle]
    pub extern "C" fn r_netdiag_init() {
        println!("test rnet_diginit");
        init_logger();
        info!("rust oslogger init suc!");
    }

    #[no_mangle]
    pub unsafe extern "C" fn r_netdiag_ping(ip: *const c_char) -> *const c_char {
        println!("call ping action ...");
        let rt: Result<tokio::runtime::Runtime, &str> = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|_f| "runtime create failed!");

        if rt.is_err() {
            log::warn!("tokio runtime create thread failed!")
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(ip) };
        let _ping_cli = rt.unwrap().block_on(async move {
            let result = crate::ping_allhost(vec![c_str.to_str().unwrap_or_default()], None).await;
            result
        });
        let rel_out = match _ping_cli {
            Ok(r) => String::from(r.join("\n")),
            Err(e) => e.to_string(),
        };
        println!("call ping action ret:{:?}", rel_out);
        let cs_r = CString::new(rel_out).unwrap_or_default().into_raw();
        cs_r
    }
}
