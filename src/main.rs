use futures::prelude::*;
use libp2p::{identity, Multiaddr, PeerId};
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent, dial_opts::DialOpts};
use std::error::Error;


#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;

    let behavior = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behavior, local_peer_id);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }


    // Ok(())
}