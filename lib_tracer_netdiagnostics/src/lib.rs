/*
@Author:bbclient
 */

pub mod config;
use futures::{pin_mut, StreamExt};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{thread, time::Duration};
use std::collections::HashMap;
use std::net::IpAddr;
use netdiag::{anyhow::anyhow, tokio, trace::Node, Bind, Tracer};

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient: &str = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

pub async fn traceroute() {
    let conf = config::TraceRouteConf::default();
    let proto = conf.proto;

    let addr = format!("{}:0", conf.host);
    let addr: std::net::IpAddr = tokio::net::lookup_host(&addr)
        .await
        .unwrap()
        .next()
        .ok_or_else(|| anyhow!("invalid target"))
        .unwrap()
        .ip();

    println!("tracing {} ({})", conf.host, addr);

    let bind = Bind::default();
    let tracer = Tracer::new(&bind).await.unwrap();
    let source = tracer.reserve(proto, addr).await.unwrap();

    let mut done = false;
    let mut ttl = 1;
    let mut probe = source.probe().unwrap();

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

        // env.call_method(&callback, "perNodeCallback", "(I)V", &[(1 as jint).into()])
        //     .unwrap();

        ttl += 1;
    }
}

/// Expose the JNI interface for android below
/// 只有在目标平台是Android的时候才开启 [cfg(target_os="android")
/// 由于JNI要求驼峰命名，所以要开启 allow(non_snake_case)
// #[cfg(any(target_os = "android", debug_assertions))]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use self::jni::objects::JClass;
    use self::jni::sys::jint;
    use self::jni::JNIEnv;
    use super::*;
    use jni::objects::JObject;
    use netdiag::{anyhow::anyhow, tokio, trace::Node, Bind, Tracer};
    use std::sync::mpsc;

    // #[no_mangle]
    // pub unsafe extern "C" fn Java_com_bbclient_example_1rustlib_RNetDiagnostics_greeting(
    //     mut env: JNIEnv,
    //     _: JClass,
    //     java_pattern: JString,
    // ) -> jstring {
    //     let c_string = CString::new("Hello!").expect("CString::new failed");
    //     let raw = c_string.into_raw();
    //     unsafe {
    //         some_extern_function(raw);
    //         let c_string = CString::from_raw(raw);
    //     }
    //     // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
    //     let world_ptr = CString::from_raw("test");
    //     let output = env
    //         .new_string(world_ptr.to_str().unwrap())
    //         .expect("Couldn't create java string!");
    //     output.to_owned()
    // }

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
        rt.block_on(rt.spawn(traceroute()));
        env.call_method(&callback, "perNodeCallback", "(I)V", &[(1 as jint).into()])
            .unwrap();
    }
}

#[test]
fn test_mod() {}
