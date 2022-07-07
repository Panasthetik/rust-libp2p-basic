# rust-libp2p-basic
Basic libp2p Rust implementation which creates a basic TCP swarm, establishes connections, and pings on connected ID's.

# directions 
```cargo run``` in the first terminal.

Copy the PeerId from the ip4 address (end of path);

In a second terminal:
```cargo run -- /ip4/127.0.0.1/tcp/<COPIED_PEER_ID>
..where COPIED_PEER_ID is the number you copied from terminal one.

Observe that these two peers are connected, and ping each other every 15 seconds!

You can also add more peers to the swarm by copying PeerId's and creating new connections in other terminals.

Can be used as a basic starter of a decentralized node project (blockchain, other peer-to-peer).
