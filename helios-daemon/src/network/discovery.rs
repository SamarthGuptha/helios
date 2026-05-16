use crate::orchestrator::scheduler::PeerRegistry;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;

const BROADCAST_PORT: u16 = 50052;

pub async fn start_discovery(grpc_port: u16, registry: Arc<PeerRegistry>) {
    println!("Network: Starting custom UDP P2P Discovery on port {}", BROADCAST_PORT);
    let sender_socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind UDP sender");
    sender_socket.set_broadcast(true).unwrap();
    let beacon_msg = format!("HELIOS_NODE:{}", grpc_port);
    let broadcast_addr = format!("255.255.255.255:{}", BROADCAST_PORT);

    tokio::spawn(async move {
        loop {
            let _ = sender_socket.send_to(beacon_msg.as_bytes(), &broadcast_addr).await;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
    let listener_socket = UdpSocket::bind(format!("0.0.0.0:{}", BROADCAST_PORT))
        .await
        .expect("Failed to bind UDP listener");

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            if let Ok((len, addr)) = listener_socket.recv_from(&mut buf).await {
                let msg = String::from_utf8_lossy(&buf[..len]);

                if msg.starts_with("HELIOS_NODE:") {
                    let parts: Vec<&str> = msg.split(':').collect();
                    if parts.len() == 2 {
                        let peer_port = parts[1];
                        let peer_grpc_addr = format!("{}:{}", addr.ip(), peer_port);
                        registry.add_peer(peer_grpc_addr).await;
                    }
                }
            }
        }
    });
}