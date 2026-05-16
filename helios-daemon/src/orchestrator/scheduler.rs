use helios_proto::distributed_compiler_client::DistributedCompilerClient;
use helios_proto::CompileTask;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
pub struct PeerRegistry {
    pub peers: RwLock<HashMap<String, u32>>,
}

impl PeerRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            peers: RwLock::new(HashMap::new()),
        })
    }
    pub async fn add_peer(&self, address: String) {
        let mut peers = self.peers.write().await;
        peers.entry(address).or_insert(0);
    }

    ///finds least loaded node to send raw code :D
    pub async fn dispatch(&self, task: CompileTask) -> Result<helios_proto::CompileResponse, String> {
        let best_peer = {
            let peers = self.peers.read().await;
            peers.iter()
                .min_by_key(|(_, jobs)| *jobs)
                .map(|(addr, _)| addr.clone())
        };

        if let Some(peer_addr) = best_peer {
            {
                let mut peers = self.peers.write().await;
                if let Some(jobs) = peers.get_mut(&peer_addr) {
                    *jobs += 1;
                }
            }
            let endpoint = format!("http://{}", peer_addr);
            let mut client = DistributedCompilerClient::connect(endpoint)
                .await
                .map_err(|e| format!("Failed to connect to peer: {}", e))?;
            let request = tonic::Request::new(task);
            let response = client.dispatch_task(request).await.map_err(|e| e.to_string())?;

            {
                let mut peers = self.peers.write().await;
                if let Some(jobs) = peers.get_mut(&peer_addr) {
                    *jobs = jobs.saturating_sub(1);
                }
            }

            Ok(response.into_inner())
        } else {
            Err("No peers available on the network".to_string())
        }
    }
}