use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

const SERVICE_TYPE: &str = "_helios._tcp.local.";

pub fn start_mdns(port: u16) -> Result<ServiceDaemon, mdns_sd:: Error> {
    let mdns = ServiceDaemon::new()?;
    let instance_name = format!("helios-node-{}", uuid::Uuid::new_v4().as_simple());

    let ip = "0.0.0.0"; //replace with your IP :D

    let service_info = ServiceInfo::new(
        SERVICE_TYPE,
        &instance_name,
        &format!("{}.local.", instance_name),
        ip,
        port,
        HashMap::new(),
    )?.with_ext_host_info(ip.parse().unwrap(), port);
    mdns.register(service_info)?;
    println!("mDNS: Broadcasting service on port {}", port);
    Ok(mdns)
}