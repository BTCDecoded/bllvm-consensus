//! Fuzzing test harnesses using Bolero
//!
//! These tests use property-based fuzzing to explore edge cases and ensure
//! robustness across a wide range of inputs.
//!
//! To run fuzzing tests:
//! ```bash
//! cargo test --features bolero fuzz_
//! ```

#[cfg(feature = "bolero")]
mod transaction_validation;

#[cfg(feature = "bolero")]
mod block_validation;

// Arbitrary trait implementations for property-based testing
mod arbitrary_impls;

