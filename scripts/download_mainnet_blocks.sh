#!/bin/bash
# Download Mainnet Blocks for Testing
#
# Downloads Bitcoin mainnet blocks at key consensus-era heights for testing.
# Uses Blockstream API (blockstream.info) for block retrieval.
#
# Usage:
#   ./scripts/download_mainnet_blocks.sh [output_dir]
#
# Output directory defaults to: tests/test_data/mainnet_blocks/

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="${1:-$PROJECT_ROOT/tests/test_data/mainnet_blocks}"

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "=== Downloading Mainnet Blocks ==="
echo "Output directory: $OUTPUT_DIR"
echo ""

# Key consensus-era heights
declare -A BLOCKS=(
    ["0"]="000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"  # Genesis
    ["100000"]=""  # Pre-SegWit era
    ["200000"]=""  # Pre-SegWit era
    ["300000"]=""  # Pre-SegWit era
    ["400000"]=""  # Pre-SegWit era
    ["481824"]=""  # SegWit activation
    ["500000"]=""  # Post-SegWit
    ["600000"]=""  # Post-SegWit
    ["709632"]=""  # Taproot activation
    ["800000"]=""  # Post-Taproot
    ["900000"]=""  # Post-Taproot
)

# Function to get block hash by height using Blockstream API
get_block_hash() {
    local height=$1
    curl -s "https://blockstream.info/api/block-height/$height" | head -c 64
}

# Function to download block by hash
download_block() {
    local hash=$1
    local height=$2
    local output_file="$OUTPUT_DIR/block_${height}.hex"
    
    if [ -f "$output_file" ]; then
        echo "✅ Block $height already exists, skipping..."
        return 0
    fi
    
    echo "Downloading block $height (hash: $hash)..."
    
    # Download block in hex format from Blockstream API
    if curl -s "https://blockstream.info/api/block/$hash/raw" -o "$output_file.tmp"; then
        # Convert binary to hex
        if command -v xxd &> /dev/null; then
            xxd -p -c 0 < "$output_file.tmp" > "$output_file"
            rm "$output_file.tmp"
            echo "✅ Block $height downloaded successfully"
        elif command -v hexdump &> /dev/null; then
            hexdump -ve '1/1 "%.2x"' < "$output_file.tmp" > "$output_file"
            rm "$output_file.tmp"
            echo "✅ Block $height downloaded successfully"
        else
            # Keep as binary if no hex converter available
            mv "$output_file.tmp" "$OUTPUT_DIR/block_${height}.bin"
            echo "✅ Block $height downloaded as binary (no hex converter found)"
        fi
    else
        echo "❌ Failed to download block $height"
        return 1
    fi
}

# Download genesis block (known hash)
echo "Downloading genesis block..."
download_block "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f" 0

# Download blocks at key heights
for height in 100000 200000 300000 400000 481824 500000 600000 709632 800000 900000; do
    echo ""
    echo "Processing block height $height..."
    
    # Get block hash for this height
    hash=$(get_block_hash "$height")
    
    if [ -z "$hash" ] || [ ${#hash} -ne 64 ]; then
        echo "⚠️  Failed to get hash for block $height, skipping..."
        continue
    fi
    
    echo "Block $height hash: $hash"
    
    # Download block
    download_block "$hash" "$height"
done

echo ""
echo "=== Download Summary ==="
echo "Blocks downloaded to: $OUTPUT_DIR"
echo ""
echo "Files created:"
ls -lh "$OUTPUT_DIR" | grep -E "block_.*\.(hex|bin)" || echo "No block files found"

echo ""
echo "=== Next Steps ==="
echo "1. Verify downloaded blocks are valid"
echo "2. Update tests/mainnet_blocks.rs to use these blocks"
echo "3. Run tests to validate blocks"
echo ""

