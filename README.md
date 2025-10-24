# Mini-Chain PoH

A minimal blockchain implementation with Proof of History (PoH) consensus mechanism inspired by Solana. This project aims to improve Rust programming skills while understanding core blockchain concepts including PoH consensus, cryptography, and distributed systems.

## Current Implementation

- **PoH Engine**: SHA256-based hash chain with tick/slot progression
- **Cryptographic Keys**: Ed25519 key generation, signing, and verification
- **Core Constants**: 12,500 hashes per tick, 64 ticks per slot (matching Solana)

## Architecture Overview

The system consists of several interconnected components:

- **PoH Service**: Continuously generates hash chain and manages timing
- **Recorder**: Captures entries and signals slot completion
- **Bank**: Processes transactions and seals blocks per slot
- **Validator**: Verifies transactions and batches them for PoH inclusion

## Future Work

- [ ] Implement channel-based communication between components
- [ ] Add transaction processing and state management
- [ ] Build block sealing mechanism with RocksDB persistence
- [ ] Create validator transaction verification logic
- [ ] Add networking layer for peer-to-peer communication
- [ ] Implement proper error handling and logging
- [ ] Add comprehensive testing suite
- [ ] Performance optimization and benchmarking

## Dependencies

- `ed25519-dalek`: Cryptographic operations
- `sha2`: Hash functions
- `base58`: Address encoding
- `rand`: Random number generation

## Running

```bash
cargo run
```

This will demonstrate basic PoH state transitions and cryptographic operations.
