mod ipc;
mod network;
mod ui;
mod execution;
mod orchestrator;

use helios_proto::distributed_compiler_server::DistributedCompilerServer;
use network::grpc_server::HeliosCompilerService;
use std::net::SocketAddr;
use tonic::transport::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let port  = 50051;
            let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
            let _mdns = network::discovery::start_mdns(port).expect("Failed to start mDNS");
            std::thread::spawn(|| {
               ipc::named_pipe::start_ipc_server();
            });
            println!("gRPC: Listening on {}", addr);
            let service = HeliosCompilerService::default();

            Server::builder()
                .add_service(DistributedCompilerServer::new(service))
                .serve(addr)
                .await.unwrap();
        });
    });

    ui::dx12_backend::run_ui_loop();
    Ok(())
}