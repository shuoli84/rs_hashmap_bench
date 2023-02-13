#! /bin/bash

set -ex

cargo build --release
mkdir -p dis

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  objdump='llvm-objdump'
fi

echo $objdump

$objdump --disassemble-symbols=std_ahash_fixed --demangle target/release/hash_perf > dis/std_ahash_fixed
$objdump --disassemble-symbols=std_ahash --demangle target/release/hash_perf > dis/std_ahash
$objdump --disassemble-symbols=std_fxhash --demangle target/release/hash_perf > dis/std_fxhash
$objdump --disassemble-symbols=hashbrown_13_ahash_fixed --demangle target/release/hash_perf > dis/hashbrown_13_ahash_fixed
$objdump --disassemble-symbols=hashbrown_13_ahash --demangle target/release/hash_perf > dis/hashbrown_13_ahash
$objdump --disassemble-symbols=hashbrown_13_fxhash --demangle target/release/hash_perf > dis/hashbrown_13_fxhash
$objdump --disassemble-symbols=hashbrown_12_ahash_fixed --demangle target/release/hash_perf > dis/hashbrown_12_ahash_fixed
$objdump --disassemble-symbols=hashbrown_12_ahash --demangle target/release/hash_perf > dis/hashbrown_12_ahash
$objdump --disassemble-symbols=hashbrown_12_fxhash --demangle target/release/hash_perf > dis/hashbrown_12_fxhash
