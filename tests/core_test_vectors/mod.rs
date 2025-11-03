//! Bitcoin Core Test Vector Integration
//!
//! Integrates Bitcoin Core's test vectors to provide free verification coverage.
//! Test vectors are extracted from Bitcoin Core's test suite and used to verify
//! consensus correctness.
//!
//! Source: Bitcoin Core test data (`bitcoin/src/test/data/*.json`)

mod block_tests;
mod transaction_tests;
mod script_tests;
mod integration_test;

pub use block_tests::*;
pub use transaction_tests::*;
pub use script_tests::*;

