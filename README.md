# BTCDecoded Bitcoin Consensus Proof

**Direct mathematical implementation of Bitcoin consensus rules from the Orange Paper.**

This crate provides pure, side-effect-free functions that implement the mathematical specifications defined in the Orange Paper. It serves as the mathematical foundation for Bitcoin consensus validation.

## Architecture Position

This is **Tier 2** of the 5-tier BTCDecoded architecture:

```
1. Orange Paper (mathematical foundation)
2. consensus-proof (pure math implementation) ← THIS CRATE
3. protocol-engine (Bitcoin abstraction)
4. reference-node (full node implementation)
5. developer-sdk (governance infrastructure)
```

## Core Functions

This crate implements all major Bitcoin consensus functions from the Orange Paper:

### Transaction Validation
- Transaction structure and limit validation
- Input validation against UTXO set
- Script execution and verification

### Block Validation
- Block connection and validation
- Transaction application to UTXO set
- Proof of work verification

### Economic Model
- Block reward calculation
- Total supply computation
- Difficulty adjustment

### Mempool Protocol
- Transaction mempool validation
- Standard transaction checks
- Transaction replacement (RBF) logic

### Mining Protocol
- Block creation from mempool
- Block mining and nonce finding
- Block template generation

### Chain Management
- Chain reorganization handling
- P2P network message processing

### Advanced Features
- **SegWit**: Witness data validation and weight calculation
- **Taproot**: P2TR output validation and key aggregation


## Design Principles

1. **Pure Functions**: All functions are deterministic and side-effect-free
2. **Mathematical Accuracy**: Direct implementation of Orange Paper specifications
3. **Exact Version Pinning**: All consensus-critical dependencies pinned to exact versions
4. **Comprehensive Testing**: Extensive test coverage with integration tests
5. **No Consensus Rule Interpretation**: Only mathematical implementation

## Dependencies

All consensus-critical dependencies are pinned to exact versions:

```toml
# Consensus-critical cryptography - EXACT VERSIONS
secp256k1 = "=0.28.2"
sha2 = "=0.10.9"
ripemd = "=0.1.3"
bitcoin_hashes = "=0.11.0"

# Non-consensus-critical utilities - COMPATIBLE VERSIONS
serde = { version = "~1.0", features = ["derive"] }
serde_json = "~1.0"
anyhow = "~1.0"
thiserror = "~1.0"
```

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage

# Run integration tests
cargo test --test integration_tests
cargo test --test integration_opportunities
```

## Orange Paper Compliance

This implementation covers all major Orange Paper sections:

- **Section 5**: Transaction and Block Validation
- **Section 6**: Script System
- **Section 7**: Economic Model
- **Section 8**: Proof of Work
- **Section 9**: Mempool and Network Protocol
- **Section 10**: Mining Protocol
- **Section 11**: Advanced Features (SegWit, Taproot)

## Security

See [SECURITY.md](SECURITY.md) for security policies and [BTCDecoded Security Policy](https://github.com/BTCDecoded/.github/blob/main/SECURITY.md) for organization-wide guidelines.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and the [BTCDecoded Contribution Guide](https://github.com/BTCDecoded/.github/blob/main/CONTRIBUTING.md).

## License

MIT License - see LICENSE file for details.
