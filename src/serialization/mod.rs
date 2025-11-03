//! Bitcoin wire format serialization/deserialization
//! 
//! This module provides consensus-critical serialization functions that must match
//! Bitcoin Core's wire format exactly to ensure consensus compatibility.
//! 
//! All serialization uses little-endian byte order (Bitcoin standard).

pub mod varint;
pub mod transaction;
pub mod block;

pub use varint::{encode_varint, decode_varint, VarIntError};
pub use transaction::{serialize_transaction, deserialize_transaction};
pub use block::{serialize_block_header, deserialize_block_header, deserialize_block_with_witnesses};

