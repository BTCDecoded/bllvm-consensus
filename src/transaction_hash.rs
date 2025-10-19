//! Transaction hash calculation for signature verification
//! 
//! Implements Bitcoin's transaction sighash algorithm for ECDSA signature verification.
//! This is critical for proper signature validation in script execution.

use crate::types::*;
use crate::error::Result;
use sha2::{Sha256, Digest};

/// SIGHASH types for transaction signature verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SighashType {
    /// Sign all inputs and outputs (default)
    All = 0x01,
    /// Sign no outputs (anyone can spend)
    None = 0x02,
    /// Sign output at same index as input
    Single = 0x03,
    /// Sign only this input (anyone can spend other inputs)
    AnyoneCanPay = 0x80,
}

impl SighashType {
    /// Parse sighash type from byte
    pub fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            0x01 => Ok(SighashType::All),
            0x02 => Ok(SighashType::None),
            0x03 => Ok(SighashType::Single),
            0x81 => Ok(SighashType::All | SighashType::AnyoneCanPay),
            0x82 => Ok(SighashType::None | SighashType::AnyoneCanPay),
            0x83 => Ok(SighashType::Single | SighashType::AnyoneCanPay),
            _ => Err(crate::error::ConsensusError::InvalidSighashType(byte)),
        }
    }
}

impl std::ops::BitOr for SighashType {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SighashType::All, SighashType::AnyoneCanPay) => SighashType::All,
            (SighashType::None, SighashType::AnyoneCanPay) => SighashType::None,
            (SighashType::Single, SighashType::AnyoneCanPay) => SighashType::Single,
            _ => self,
        }
    }
}

/// Calculate transaction sighash for signature verification
/// 
/// This implements the Bitcoin transaction hash algorithm used for ECDSA signatures.
/// The sighash determines which parts of the transaction are signed.
/// 
/// # Arguments
/// * `tx` - The transaction being signed
/// * `input_index` - Index of the input being signed
/// * `prevouts` - Previous transaction outputs (for input validation)
/// * `sighash_type` - Type of sighash to calculate
/// 
/// # Returns
/// 32-byte hash to be signed with ECDSA
pub fn calculate_transaction_sighash(
    tx: &Transaction,
    input_index: usize,
    prevouts: &[TransactionOutput],
    sighash_type: SighashType,
) -> Result<Hash> {
    // Validate input index
    if input_index >= tx.inputs.len() {
        return Err(crate::error::ConsensusError::InvalidInputIndex(input_index));
    }
    
    // Validate prevouts match inputs
    if prevouts.len() != tx.inputs.len() {
        return Err(crate::error::ConsensusError::InvalidPrevoutsCount(prevouts.len(), tx.inputs.len()));
    }
    
    // Create sighash preimage
    let mut preimage = Vec::new();
    
    // 1. Transaction version (4 bytes, little endian)
    preimage.extend_from_slice(&tx.version.to_le_bytes());
    
    // 2. Number of inputs (varint)
    preimage.extend_from_slice(&encode_varint(tx.inputs.len() as u64));
    
    // 3. Inputs (depending on sighash type)
    for (i, input) in tx.inputs.iter().enumerate() {
        if matches!(sighash_type, SighashType::AnyoneCanPay) || i == input_index {
            // Include this input
            preimage.extend_from_slice(&input.prevout.hash);
            preimage.extend_from_slice(&input.prevout.index.to_le_bytes());
            preimage.extend_from_slice(&encode_varint(input.script_sig.len() as u64));
            preimage.extend_from_slice(&input.script_sig);
            preimage.extend_from_slice(&input.sequence.to_le_bytes());
        } else {
            // Skip this input (use dummy values)
            preimage.extend_from_slice(&[0u8; 32]); // prevout hash
            preimage.extend_from_slice(&[0u8; 4]);  // prevout index
            preimage.push(0); // empty script_sig
            preimage.extend_from_slice(&[0u8; 4]);  // sequence
        }
    }
    
    // 4. Number of outputs (varint)
    preimage.extend_from_slice(&encode_varint(tx.outputs.len() as u64));
    
    // 5. Outputs (depending on sighash type)
    match sighash_type {
        SighashType::All => {
            // Include all outputs
            for output in &tx.outputs {
                preimage.extend_from_slice(&output.value.to_le_bytes());
                preimage.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
                preimage.extend_from_slice(&output.script_pubkey);
            }
        },
        SighashType::None => {
            // No outputs
        },
        SighashType::Single => {
            // Include output at same index as input
            if input_index < tx.outputs.len() {
                let output = &tx.outputs[input_index];
                preimage.extend_from_slice(&output.value.to_le_bytes());
                preimage.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
                preimage.extend_from_slice(&output.script_pubkey);
            }
        },
        _ => {
            // AnyoneCanPay combinations
            for output in &tx.outputs {
                preimage.extend_from_slice(&output.value.to_le_bytes());
                preimage.extend_from_slice(&encode_varint(output.script_pubkey.len() as u64));
                preimage.extend_from_slice(&output.script_pubkey);
            }
        }
    }
    
    // 6. Lock time (4 bytes, little endian)
    preimage.extend_from_slice(&tx.lock_time.to_le_bytes());
    
    // 7. SIGHASH type (4 bytes, little endian)
    preimage.extend_from_slice(&(sighash_type as u32).to_le_bytes());
    
    // Calculate double SHA256 hash
    let first_hash = Sha256::digest(&preimage);
    let second_hash = Sha256::digest(first_hash);
    
    let mut result = [0u8; 32];
    result.copy_from_slice(&second_hash);
    
    Ok(result)
}

/// Encode integer as Bitcoin varint
fn encode_varint(value: u64) -> Vec<u8> {
    if value < 0xfd {
        vec![value as u8]
    } else if value <= 0xffff {
        let mut result = vec![0xfd];
        result.extend_from_slice(&(value as u16).to_le_bytes());
        result
    } else if value <= 0xffffffff {
        let mut result = vec![0xfe];
        result.extend_from_slice(&(value as u32).to_le_bytes());
        result
    } else {
        let mut result = vec![0xff];
        result.extend_from_slice(&value.to_le_bytes());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sighash_type_parsing() {
        assert_eq!(SighashType::from_byte(0x01).unwrap(), SighashType::All);
        assert_eq!(SighashType::from_byte(0x02).unwrap(), SighashType::None);
        assert_eq!(SighashType::from_byte(0x03).unwrap(), SighashType::Single);
        assert!(SighashType::from_byte(0x00).is_err());
    }
    
    #[test]
    fn test_varint_encoding() {
        assert_eq!(encode_varint(0), vec![0]);
        assert_eq!(encode_varint(252), vec![252]);
        assert_eq!(encode_varint(253), vec![0xfd, 253, 0]);
        assert_eq!(encode_varint(65535), vec![0xfd, 255, 255]);
        assert_eq!(encode_varint(65536), vec![0xfe, 0, 0, 1, 0]);
    }
    
    #[test]
    fn test_sighash_calculation() {
        // Create a simple transaction for testing
        let tx = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                prevout: OutPoint { hash: [1u8; 32], index: 0 },
                script_sig: vec![0x51], // OP_1
                sequence: 0xffffffff,
            }],
            outputs: vec![TransactionOutput {
                value: 5000000000,
                script_pubkey: vec![0x76, 0xa9, 0x14, 0x89, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0x88, 0xac], // P2PKH
            }],
            lock_time: 0,
        };
        
        let prevouts = vec![TransactionOutput {
            value: 10000000000,
            script_pubkey: vec![0x76, 0xa9, 0x14, 0x89, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0x88, 0xac],
        }];
        
        // Test SIGHASH_ALL
        let sighash = calculate_transaction_sighash(&tx, 0, &prevouts, SighashType::All).unwrap();
        assert_eq!(sighash.len(), 32);
        
        // Test SIGHASH_NONE
        let sighash_none = calculate_transaction_sighash(&tx, 0, &prevouts, SighashType::None).unwrap();
        assert_ne!(sighash, sighash_none);
        
        // Test SIGHASH_SINGLE
        let sighash_single = calculate_transaction_sighash(&tx, 0, &prevouts, SighashType::Single).unwrap();
        assert_ne!(sighash, sighash_single);
    }
    
    #[test]
    fn test_sighash_invalid_input_index() {
        let tx = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![],
            lock_time: 0,
        };
        
        let result = calculate_transaction_sighash(&tx, 0, &[], SighashType::All);
        assert!(result.is_err());
    }
}

