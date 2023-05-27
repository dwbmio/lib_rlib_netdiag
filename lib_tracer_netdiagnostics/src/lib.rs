/*
@Author:bbclient
 */

pub mod config;
use futures::{pin_mut, StreamExt};
use log::LevelFilter;
use log::{debug, error, info};
use netdiag::{anyhow::anyhow, tokio, trace::Node, Bind, Tracer};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::net::IpAddr;
use std::os::raw::c_char;
use std::{sync::mpsc, thread, time::Duration};

/// Expose the JNI interface for android below
/// 只有在目标平台是Android的时候才开启 [cfg(target_os="android")
/// 由于JNI要求驼峰命名，所以要开启 allow(non_snake_case)
#[cfg(any(target_os = "android", debug_assertions))]
#[allow(non_snake_case)]
pub mod android {

    extern crate android_logger;
    extern crate jni;
    extern crate log;

    use super::*;
    use jni::objects::JClass;
    use jni::objects::JObject;
    use jni::objects::JString;
    use jni::sys::jint;
    use jni::JNIEnv;

    use android_logger::{Config, FilterBuilder};

    pub fn init_logger() {
        android_logger::init_once(Config::default().with_min_level(log::Level::Debug));
        debug!("this is a debug {}", "message1312312312");
    }

    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_init() {
        init_logger();
    }
    
    pub async fn traceroute(host: String) -> netdiag::anyhow::Result<()> {
        debug!("12312312312");
        let conf = config::TraceRouteConf::new(host, "udp".to_owned(), 0);
        let proto: netdiag::Protocol = conf.proto;

        let addr: String = format!("{}:0", conf.host);
        debug!("addr is {}, proto:{:?}", addr, proto);
        let addr: std::net::IpAddr = tokio::net::lookup_host(&addr)
            .await?
            .next()
            .ok_or_else(|| anyhow!("invalid target"))?
            .ip();

        debug!("tracing {} ({})", conf.host, addr);

        let bind = Bind::default();
        debug!("bind arrd: {:?}",  bind);
        let tracer = Tracer::new(&bind).await?;
        debug!("11111111111");
        let source = tracer.reserve(proto, addr).await?;
        debug!("11111111111");
        let mut done = false;
        let mut ttl = 1;
        let mut probe = source.probe()?;

        while !done && ttl <= conf.limit {
            let mut nodes = HashMap::<IpAddr, Vec<String>>::new();

            let stream = tracer.probe(&mut probe, ttl, Duration::from_millis(conf.expiry));
            let stream = stream.take(conf.count);
            pin_mut!(stream);

            while let Some(Ok(node)) = stream.next().await {
                if let Node::Node(_, ip, rtt, last) = node {
                    let rtt = format!("{:>0.2?}", rtt);
                    nodes.entry(ip).or_default().push(rtt);
                    done = last || ip == addr;
                }

                tokio::time::sleep(Duration::from_millis(conf.delay)).await;
            }
            debug!("nodes: ttl={}, count={}", ttl, conf.count);
            // env.call_method(&callback, "perNodeCallback", "(I)V", &[(1 as jint).into()])
            //     .unwrap();

            ttl += 1;
        }
        Ok(())
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

    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_helloasync(
        env: JNIEnv,
        _class: JClass,
        callback: JObject,
    ) {
        // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
        // functions in other threads, we must first obtain the `JavaVM` interface
        // which, unlike `JNIEnv` is `Send`.
        let jvm = env.get_java_vm().unwrap();

        // We need to obtain global reference to the `callback` object before sending
        // it to the thread, to prevent it from being collected by the GC.
        let callback = env.new_global_ref(callback).unwrap();

        // Use channel to prevent the Java program to finish before the thread
        // has chance to start.
        let (tx, rx) = mpsc::channel();

        let _ = thread::spawn(move || {
            // Signal that the thread has started.
            tx.send(()).unwrap();

            // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
            let mut env = jvm.attach_current_thread().unwrap();

            for i in 0..11 {
                let progress = (i * 10) as jint;
                // Now we can use all available `JNIEnv` functionality normally.
                env.call_method(&callback, "perNodeCallback", "(I)V", &[progress.into()])
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
            }

            // The current thread is detached automatically when `env` goes out of scope.
        });

        // Wait until the thread has started.
        rx.recv().unwrap();
    }

    // traceroute
    /* */
    #[no_mangle]
    pub extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_traceroute(
        mut env: JNIEnv,
        _class: JClass,
        callback: JObject,
    ) {
        let jvm = env.get_java_vm().unwrap();
        let callback = env.new_global_ref(callback).unwrap();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        // let mut handles = Vec::with_capacity(10);
        rt.block_on(rt.spawn(async {
            let ret = traceroute("10.250.15.60".to_string()).await;
            debug!("result is {:?}", ret);
        }));
        env.call_method(&callback, "perNodeCallback", "(I)V", &[(1 as jint).into()])
            .unwrap();
    }
}

#[test]
fn test_mod() {}
