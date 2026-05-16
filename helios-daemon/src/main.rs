mod ipc;
mod network;
mod execution;
mod orchestrator;

use helios_proto::distributed_compiler_server::DistributedCompilerServer;
use network::grpc_server::HeliosCompilerService;
use orchestrator::scheduler::PeerRegistry;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 50051;
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let peer_registry = PeerRegistry::new();
    network::discovery::start_discovery(port, peer_registry.clone()).await;
    let ipc_registry = peer_registry.clone();
    std::thread::spawn(move || {
        ipc::named_pipe::start_ipc_server(ipc_registry);
    });
    println!("gRPC: Listening on {}", addr);
    let service = HeliosCompilerService::default();
    Server::builder()
        .add_service(DistributedCompilerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}