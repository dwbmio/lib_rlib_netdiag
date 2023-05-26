



pub fn test_traceroute() {
    
    let cfg = config::TraceRouteConf::default();
    let (tx, rx) = mpcs::channel();

    let _ = thread::spawn(async move || {

        let proto = cfg.proto;

        let bind = Bind::default();
        let tracer = Tracer::new(&bind).await.unwrap();
        let source = tracer.reserve(proto, addr).await.unwrap();

        let mut done = false;
        let mut ttl = 1;
        let mut probe = source.probe()?;

        while !done && ttl <= limit {
            let mut nodes = HashMap::<IpAddr, Vec<String>>::new();

            let stream = tracer.probe(&mut probe, ttl, expiry);
            let stream = stream.take(count);
            pin_mut!(stream);

            while let Some(Ok(node)) = stream.next().await {
                if let Node::Node(_, ip, rtt, last) = node {
                    let rtt = format!("{:>0.2?}", rtt);
                    nodes.entry(ip).or_default().push(rtt);
                    done = last || ip == addr;
                }

                sleep(delay).await;
            }

            env.call_method(&callback, "perNodeCallback", "(I)V", &[1])
                .unwrap();

            ttl += 1;
        }
    });
    rx.recv().unwarap();
}

fn main () {
    test_traceroute();
}