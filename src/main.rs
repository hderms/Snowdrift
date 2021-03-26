use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use hyper::StatusCode;

//Internal modules
pub mod worker;
use worker::Worker;

pub mod id_generator;
use id_generator::Id;

pub mod configuration;
use configuration::Configuration;
async fn create_id_response(id: Id, sequence: u64) -> Result<Response<Body>, BoxError> {
    let mut response = Response::new(Body::from(id.as_string_output()));
    if sequence > (u16::MAX as u64) {
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        *response.body_mut() = Body::from("Ran out of IDs!");
    }
    Ok(response)
}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let configuration = Configuration::from_env();

    let addr = SocketAddr::from(([0, 0, 0, 0], configuration.port));
    let worker = Arc::new(Mutex::new(Worker::new(configuration.machine_id)));

    let make_service = make_service_fn(move |client: &AddrStream| {
        let _ip = client.remote_addr();
        let shared = worker.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |_req| {
                let mut work = shared.lock().unwrap();
                let (sequence, now) = work.next_id_and_timestamp();

                let id = Id::new(now as u64, work.machine_id, sequence as u16);
                create_id_response(id, sequence)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
