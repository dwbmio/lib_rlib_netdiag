#[cfg(any(target_os = "ios", debug_assertions))]
pub mod ios {
    use std::ffi::{c_char, CStr};

    pub fn init_logger() {
        // OsLogger::new("com.example.test")
        //     .level_filter(LevelFilter::Debug)
        //     .category_level_filter("Settings", LevelFilter::Trace)
        //     .init()
        //     .unwrap();
    }

    #[no_mangle]
    pub extern "C" fn init() {
        init_logger();
        info!("rust oslogger init suc!")
    }

    #[no_mangle]
    pub unsafe extern "C" fn ping(ip: *const c_char) {
        let rt = tokio::runtime::Builder::new_multi_thread()
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
    }
}
