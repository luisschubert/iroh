//! Example showing how to bind mDNS discovery to a specific network interface.
//!
//! This example demonstrates how to use the new interface binding functionality
//! to ensure mDNS discovery uses a specific network interface instead of relying
//! on the system's routing table.
//!
//! This is particularly useful in scenarios where you have multiple network interfaces
//! (e.g., WiFi and LTE) and want to ensure discovery happens on a specific interface.

use std::net::{IpAddr, SocketAddr};

use iroh::{discovery::mdns::MdnsDiscovery, Endpoint};
use n0_snafu::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    println!("Interface-Bound Discovery Example!");

    // Example: Bind discovery to wlan0 interface
    // Replace with your actual interface IP address
    let wlan0_ip = IpAddr::V4([192, 168, 1, 100].into());
    let wlan0_addr = SocketAddr::new(wlan0_ip, 0); // Port 0 means let OS choose

    println!("Binding discovery to interface: {}", wlan0_addr);

    // Create endpoint with interface-bound discovery
    let endpoint = Endpoint::builder()
        .add_discovery(
            MdnsDiscovery::builder()
                .with_interface_addr(wlan0_addr) // Bind to specific interface
        )
        .bind()
        .await?;

    let node_id = endpoint.node_id();
    println!("Created endpoint {} bound to interface {}", node_id.fmt_short(), wlan0_addr);

    // The discovery service will now use the specified interface for multicast operations
    // instead of relying on the system's routing table

    // Example: Listen for discovered peers
    let mut discovery_stream = endpoint.discovery_stream();
    println!("Listening for peers on interface {}...", wlan0_addr);

    while let Some(item) = discovery_stream.next().await {
        match item {
            Ok(item) => {
                println!("Discovered peer: {} at {:?}", 
                    item.node_id().fmt_short(), 
                    item.node_info().data.direct_addresses()
                );
            }
            Err(e) => {
                tracing::error!("Discovery error: {}", e);
            }
        }
    }

    Ok(())
}
