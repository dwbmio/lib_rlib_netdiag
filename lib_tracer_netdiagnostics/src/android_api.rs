/// Expose the JNI interface for android below
/// 只有在目标平台是Android的时候才开启 [cfg(target_os="android")
/// 由于JNI要求驼峰命名，所以要开启 allow(non_snake_case)
#[cfg(any(target_os = "android"))]
#[allow(non_snake_case)]
pub mod android {

    extern crate android_logger;
    extern crate jni;

    use jni::objects::JClass;

    use jni::objects::JObject;
    use jni::objects::JString;
    use jni::JNIEnv;

    use android_logger::Config;

    pub fn init_logger() {
        android_logger::init_once(
            Config::default().with_max_level(log::LevelFilter::Debug), // .with_tag("[lib-trace]"),
        );
    }

    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_init() {
        init_logger();
        info!("rust android_logger init suc!")
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_greeting<'local>(
        mut env: JNIEnv<'local>,
        // this is the class that owns our static method. Not going to be used, but
        // still needs to have an argument slot
        _class: JClass<'local>,
        input: JString<'local>,
    ) -> JString<'local> {
        let input: String = env
            .get_string(&input)
            .expect("Couldn't get java string!")
            .into();

        // Then we have to create a new java string to return. Again, more info
        // in the `strings` module.
        debug!("test heres greeting");
        let output = env
            .new_string(format!("Hello, {}!", input))
            .expect("Couldn't create java string!");
        output
    }

    // ping
    /**
     *
     */
    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_ping<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        ip: JString<'local>,
        callback: JObject<'local>,
    ) {
        debug!("ping  start...");
        let _jvm = env.get_java_vm().unwrap();
        let _callback = env.new_global_ref(&callback).unwrap();
        let _ip: String = env
            .get_string(&ip)
            .expect("Couldn't get java string!")
            .into();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|_f| "runtime create failed!");

        if rt.is_err() {
            log::warn!("tokio runtime create thread failed!")
        }
        let _ping_cli = rt.unwrap().block_on(async move {
            let result = crate::ping_allhost(vec![&_ip], None).await;
            let rel_out = match result {
                Ok(r) => env.new_string(r.join("\n")).unwrap_or_default(),
                Err(e) => env.new_string(e.to_string()).unwrap_or_default(),
            };
            env.call_method(
                &callback,
                "pingResult",
                "(Ljava/lang/String;)V",
                &[(&rel_out).into()],
            )
            .unwrap();
        });
    }
}
