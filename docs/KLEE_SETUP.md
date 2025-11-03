# KLEE Symbolic Execution Setup (Phase 2.1)

## Overview

KLEE is a symbolic execution engine that systematically explores program execution paths to generate high-coverage test cases. This can achieve +10-12% verification coverage (bringing total to 97%).

## Prerequisites

### 1. Install LLVM
```bash
# On Ubuntu/Debian
sudo apt-get install llvm llvm-dev clang

# On Arch
sudo pacman -S llvm clang

# Verify installation
llvm-config --version
```

### 2. Install KLEE

Follow the official KLEE installation guide:
https://klee.github.io/getting-started/

Quick summary:
```bash
# Clone KLEE
git clone https://github.com/klee/klee.git
cd klee
mkdir build && cd build
cmake ..
make -j$(nproc)
sudo make install
```

### 3. Verify KLEE Installation
```bash
klee --version
```

## Usage

### Option 1: C Wrapper Approach (Recommended)

Since Rust doesn't directly support KLEE intrinsics, create C wrappers:

1. **Create C wrapper for consensus functions**:
   ```c
   // klee_wrapper.c
   #include <klee/klee.h>
   #include "consensus_ffi.h" // Generated Rust FFI bindings
   
   int klee_check_transaction() {
       Transaction tx;
       klee_make_symbolic(&tx, sizeof(Transaction), "tx");
       return rust_check_transaction(&tx);
   }
   ```

2. **Generate Rust FFI bindings**:
   ```bash
   cargo build --features klee-ffi
   ```

3. **Compile to LLVM bitcode**:
   ```bash
   clang -c -emit-llvm klee_wrapper.c -o klee_wrapper.bc
   ```

4. **Run KLEE**:
   ```bash
   klee --libc=uclibc klee_wrapper.bc
   ```

### Option 2: Rust + LLVM Bitcode (Future)

When Rust LLVM backend supports KLEE intrinsics:
```rust
#[klee]
fn klee_test() {
    let mut tx = Transaction::default();
    klee_make_symbolic(&mut tx);
    check_transaction(&tx);
}
```

## Current Status

- ✅ Infrastructure framework created in `tests/klee/mod.rs`
- ⏳ Requires KLEE installation and C wrapper implementation
- ⏳ Needs Rust FFI bindings for consensus functions
- ⏳ Test case generation from KLEE output (ktest format)

## Expected Coverage Gains

- Transaction validation: +5-7% coverage
- Block validation: +3-4% coverage
- Script execution: +2-3% coverage
- **Total: +10-12% → 97% total coverage**

## Next Steps

1. Install KLEE (manual step)
2. Create C wrapper functions
3. Generate Rust FFI bindings for consensus functions
4. Compile and run KLEE
5. Integrate generated test cases into test suite

