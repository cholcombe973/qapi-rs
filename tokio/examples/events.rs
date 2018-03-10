extern crate tokio_qapi;
extern crate tokio_uds;
extern crate tokio_core;
extern crate futures;
extern crate env_logger;

#[cfg(feature = "qmp")]
mod main {
    use std::env::args;
    use tokio_uds::UnixStream;
    use tokio_core::reactor::Core;
    use futures::Stream;
    use tokio_qapi;

    pub fn main() {
        ::env_logger::init();

        let socket_addr = args().nth(1).expect("argument: QMP socket path");

        let mut core = Core::new().expect("failed to create core");
        let stream = UnixStream::connect(socket_addr, &core.handle()).expect("failed to connect to socket");
        let (stream, events) = tokio_qapi::event_stream(stream);
        let (caps, _stream) = core.run(tokio_qapi::qmp_handshake(stream)).expect("failed to handshake");
        println!("{:#?}", caps);
        core.run(events.for_each(|e| Ok(println!("Got event {:#?}", e)))).unwrap();
    }
}

#[cfg(not(feature = "qmp"))]
mod main {
    pub fn main() { panic!("requires feature qmp") }
}

fn main() { main::main() }