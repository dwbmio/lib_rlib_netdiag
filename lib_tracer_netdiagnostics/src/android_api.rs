/// Expose the JNI interface for android below
/// 只有在目标平台是Android的时候才开启 [cfg(target_os="android")
/// 由于JNI要求驼峰命名，所以要开启 allow(non_snake_case)
#[cfg(any(target_os = "android", debug_assertions))]
#[allow(non_snake_case)]
pub mod android {

    extern crate android_logger;
    extern crate jni;
    extern crate log;
    use jni::objects::JClass;

    use jni::objects::JObject;
    use jni::objects::JObjectArray;
    use jni::objects::JString;
    use jni::sys::jint;
    use jni::JNIEnv;

    use android_logger::Config;
    use log::info;
    use log::warn;

    pub fn init_logger() {
        android_logger::init_once(Config::default().with_min_level(log::Level::Debug));
    }

    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_init() {
        init_logger();
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
        let output = env
            .new_string(format!("Hello, {}!", input))
            .expect("Couldn't create java string!");
        output
    }

    // ping-multi
    /**
     *
     */
    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_ping<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        host: JString<'local>,
        callback: JObject<'local>,
    ) {
        let _jvm = env.get_java_vm().unwrap();
        let callback = env.new_global_ref(callback).unwrap();
        let host: String = env
            .get_string(&host)
            .expect("Couldn't get java string!")
            .into();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let _ping_cli = rt.block_on(rt.spawn(async move {
            let result = crate::ping_allhost(vec![&host], None).await;
            info!("ping result is {:?}", result);
        }));
        env.call_method(&callback, "perNodeCallback", "(I)V", &[(1 as jint).into()])
            .unwrap();
    }

    // ping-single
    /* */
    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_multiping<'local>(
        env: JNIEnv<'local>,
        _class: JClass<'local>,
        host_list: JObjectArray<'local>,
        callback: JObject<'local>,
    ) {
        let _jvm = env.get_java_vm().unwrap();
        let _callback = env.new_global_ref(callback).unwrap();

        //array len
        let len = &env.get_array_length(&host_list).unwrap_or_default();
        if *len <= 0 {
            warn!("Input host list lenght is 0!");
        }
        let _hoststr_list: Vec<&str> = Vec::new();
        // for i in 0..*len {
        //     let ele = env.get_object_array_element(host_list, i);
        //     if let Ok(e) = ele {
        // let v =
        // if let Ok(h) = v {
        //     hoststr_list.push(h.to_str().unwrap_or_default());
        // }
        // }
        // }

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let _ping_cli = rt.block_on(rt.spawn(async {
            // let result = multi_ping(hoststr_list).await;
        }));
        // env.call_method(&callback, "perNodeCallback", "(I)V", &[(1 as jint).into()])
        //     .unwrap();
    }
}
