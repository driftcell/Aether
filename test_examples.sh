#!/bin/bash
# Test script for compiling and executing Aether examples

set -e

AETHER="./target/release/aether"
EXAMPLES_DIR="examples"

echo "======================================================"
echo "  Aether Bytecode Compiler & VM Test Suite"
echo "======================================================"
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
total_tests=0
passed_tests=0
failed_tests=0

# Function to test an example
test_example() {
    local name=$1
    local ae_file="${EXAMPLES_DIR}/${name}.ae"
    local aeb_file="${EXAMPLES_DIR}/${name}.aeb"
    
    total_tests=$((total_tests + 1))
    
    echo -e "${BLUE}Testing: ${name}${NC}"
    echo "----------------------------------------"
    
    # Show source
    echo "Source code:"
    cat "$ae_file"
    echo ""
    
    # Compile
    echo "Compiling..."
    if ! $AETHER compile "$ae_file" > /dev/null 2>&1; then
        echo -e "${RED}✗ Compilation FAILED${NC}"
        failed_tests=$((failed_tests + 1))
        echo ""
        return 1
    fi
    echo -e "${GREEN}✓ Compilation successful${NC}"
    
    # Check bytecode file size
    local size=$(stat -f%z "$aeb_file" 2>/dev/null || stat -c%s "$aeb_file" 2>/dev/null)
    echo "Bytecode size: ${size} bytes"
    
    # Execute
    echo "Executing bytecode..."
    if ! $AETHER exec "$aeb_file" > /tmp/aether_output.txt 2>&1; then
        echo -e "${RED}✗ Execution FAILED${NC}"
        cat /tmp/aether_output.txt
        failed_tests=$((failed_tests + 1))
        echo ""
        return 1
    fi
    
    # Show result
    echo "Output:"
    grep "Result:" /tmp/aether_output.txt || true
    echo -e "${GREEN}✓ Execution successful${NC}"
    passed_tests=$((passed_tests + 1))
    
    echo ""
}

# Build the project first
echo "Building Aether..."
cargo build --release > /dev/null 2>&1
echo -e "${GREEN}✓ Build successful${NC}"
echo ""

# Test all examples
examples=(
    "hello"
    "number"
    "arithmetic"
    "variable"
    "sequence"
    "power"
    "sqrt"
    "datetime"
    "random"
    "hash"
    "log"
    "complex_math"
)

for example in "${examples[@]}"; do
    test_example "$example"
done

# Summary
echo "======================================================"
echo "  Test Summary"
echo "======================================================"
echo "Total tests:  $total_tests"
echo -e "${GREEN}Passed:       $passed_tests${NC}"
if [ $failed_tests -gt 0 ]; then
    echo -e "${RED}Failed:       $failed_tests${NC}"
else
    echo "Failed:       $failed_tests"
fi
echo ""

if [ $failed_tests -eq 0 ]; then
    echo -e "${GREEN}All tests passed! 🎉${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
