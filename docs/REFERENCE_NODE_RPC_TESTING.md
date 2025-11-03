# Reference-Node RPC Integration for Testing

This document explains how to use `reference-node`'s RPC infrastructure to test `consensus-proof` validation.

## Overview

`reference-node` provides a complete Bitcoin node implementation with JSON-RPC 2.0 API that uses `consensus-proof` for all consensus decisions. This allows us to test consensus validation through RPC calls instead of direct function calls.

## Architecture

```
┌─────────────────┐
│  Test Suite     │
│  (consensus-    │
│   proof tests)  │
└────────┬────────┘
         │ RPC Calls
         ▼
┌─────────────────┐
│  reference-node │
│  (RPC Server)   │
└────────┬────────┘
         │ Uses
         ▼
┌─────────────────┐
│ consensus-proof │
│  (Validation)   │
└─────────────────┘
```

## Starting Reference-Node for Testing

### Method 1: In-Process Test Node

For unit tests, start a test node in the same process:

```rust
use reference_node::{ReferenceNode, ProtocolVersion};

#[tokio::test]
async fn test_consensus_validation_via_rpc() {
    // Start reference-node in regtest mode (safe for testing)
    let node = ReferenceNode::new(Some(ProtocolVersion::Regtest))?;
    
    // Start RPC server on random port
    let rpc_manager = node.rpc_manager();
    let rpc_addr = "127.0.0.1:0".parse()?;
    rpc_manager.start(rpc_addr).await?;
    
    // Get actual RPC address
    let actual_addr = rpc_manager.server_addr();
    
    // Now you can make RPC calls to test consensus-proof
    // ...
}
```

### Method 2: Standalone Test Node

For integration tests, start a standalone node:

```bash
# Start reference-node in regtest mode
cd reference-node
cargo run -- --regtest --rpc-bind=127.0.0.1:18332
```

Then connect to it from tests:

```rust
use consensus_proof::tests::integration::reference_node_rpc::*;

let config = ReferenceNodeRpcConfig {
    url: "http://127.0.0.1:18332".to_string(),
    username: None,
    password: None,
};
```

## RPC Methods for Testing Consensus

### Transaction Validation

Use `testmempoolaccept` to test transaction validation:

```rust
use reference_node::rpc::rawtx::RawTxRpc;

let rpc = RawTxRpc::new();

// Test if transaction would be accepted
let tx_hex = hex::encode(&serialized_tx);
let result = rpc.test_mempool_accept(vec![tx_hex]).await?;

// Check validation result
if result[0].allowed {
    // Transaction is valid according to consensus-proof
    println!("Transaction accepted");
} else {
    // Transaction rejected: result[0].reject_reason
    println!("Transaction rejected: {}", result[0].reject_reason);
}
```

### Block Validation

Use `submitblock` to test block validation:

```rust
use reference_node::rpc::mining::MiningRpc;

let rpc = MiningRpc::new();

// Submit block for validation
let block_hex = hex::encode(&serialized_block);
let result = rpc.submit_block(block_hex).await?;

// Empty string means valid block
if result.is_empty() {
    // Block is valid according to consensus-proof
    println!("Block accepted");
} else {
    // Block rejected: result contains error message
    println!("Block rejected: {}", result);
}
```

### Blockchain Queries

Use blockchain RPC methods to query validated state:

```rust
use reference_node::rpc::blockchain::BlockchainRpc;

let rpc = BlockchainRpc::new();

// Get block information
let block_info = rpc.get_block(&block_hash, Some(2)).await?;

// Get blockchain info
let chain_info = rpc.get_blockchain_info().await?;

// Get UTXO set info
let utxo_info = rpc.get_txout_set_info().await?;
```

## Integration Test Example

Here's a complete example of testing consensus-proof via reference-node RPC:

```rust
use consensus_proof::*;
use consensus_proof::serialization::transaction::serialize_transaction;
use consensus_proof::tests::integration::reference_node_rpc::*;

#[tokio::test]
async fn test_transaction_validation_via_rpc() {
    // Create a test transaction
    let tx = Transaction {
        version: 1,
        inputs: vec![],
        outputs: vec![TransactionOutput {
            value: 1000,
            script_pubkey: vec![0x51], // OP_1
        }],
        lock_time: 0,
    };
    
    // Serialize transaction
    let tx_hex = hex::encode(&serialize_transaction(&tx));
    
    // Connect to reference-node RPC
    let config = ReferenceNodeRpcConfig::default();
    let rpc_client = ReferenceNodeRpcClient::new(config)?;
    
    // Test transaction validation
    let result = rpc_client.test_mempool_accept(&tx_hex).await?;
    
    // Verify result matches direct consensus-proof validation
    let direct_result = check_transaction(&tx)?;
    let direct_valid = matches!(direct_result, ValidationResult::Valid);
    
    assert_eq!(result.allowed, direct_valid, 
        "RPC validation should match direct validation");
}
```

## RPC Configuration

### Default Configuration

```rust
pub struct ReferenceNodeRpcConfig {
    pub url: String,           // RPC server URL
    pub username: Option<String>,  // RPC username (if auth enabled)
    pub password: Option<String>,  // RPC password (if auth enabled)
}

impl Default for ReferenceNodeRpcConfig {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:18332".to_string(), // Regtest default
            username: None,
            password: None,
        }
    }
}
```

### Environment Variables

You can configure RPC connection via environment variables:

```bash
export REFERENCE_NODE_RPC_URL=http://127.0.0.1:18332
export REFERENCE_NODE_RPC_USER=rpcuser
export REFERENCE_NODE_RPC_PASS=rpcpass
```

## Testing Workflow

### 1. Start Reference-Node

```bash
cd reference-node
cargo run -- --regtest --rpc-bind=127.0.0.1:18332
```

### 2. Run Tests

```bash
cd consensus-proof
cargo test --test reference_node_rpc_tests
```

### 3. Verify Results

Tests will:
- Validate transactions via `testmempoolaccept`
- Validate blocks via `submitblock`
- Query blockchain state via `getblock`, `getblockchaininfo`, etc.
- Compare RPC results with direct consensus-proof validation

## Benefits of RPC Testing

1. **Integration Testing**: Tests the full stack from RPC → node → consensus-proof
2. **Real-World Scenarios**: Tests how consensus-proof behaves in a real node context
3. **Protocol Validation**: Ensures RPC methods correctly use consensus-proof
4. **Network Testing**: Can test across multiple nodes (future enhancement)

## Comparison with Direct Testing

### Direct Testing (Current)
```rust
let result = check_transaction(&tx)?;
// Fast, unit-level testing
```

### RPC Testing (This Document)
```rust
let result = rpc_client.test_mempool_accept(&tx_hex).await?;
// Slower, integration-level testing, but tests full stack
```

## Future Enhancements

1. **Multi-Node Testing**: Test consensus across multiple reference-nodes
2. **Network Scenarios**: Test block/transaction propagation
3. **Performance Testing**: Benchmark consensus validation via RPC
4. **Differential Testing**: Compare reference-node RPC results with Core RPC

## See Also

- [Reference-Node RPC Documentation](../../reference-node/RPC_IMPLEMENTATION_STATUS.md)
- [Differential Testing with Core](differential_tests.md)
- [Consensus-Proof Testing Guide](TESTING.md)

