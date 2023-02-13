#! /bin/bash

set -ex

cargo build --release
mkdir -p dis

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  objdump='llvm-objdump'
  sym_prefix=''
else
  objdump='objdump'
  sym_prefix='_'
fi

echo $objdump

$objdump --disassemble-symbols=${sym_prefix}std_ahash_fixed --demangle target/release/hash_perf > dis/std_ahash_fixed
$objdump --disassemble-symbols=${sym_prefix}std_ahash --demangle target/release/hash_perf > dis/std_ahash
$objdump --disassemble-symbols=${sym_prefix}std_fxhash --demangle target/release/hash_perf > dis/std_fxhash
$objdump --disassemble-symbols=${sym_prefix}hashbrown_13_ahash_fixed --demangle target/release/hash_perf > dis/hashbrown_13_ahash_fixed
$objdump --disassemble-symbols=${sym_prefix}hashbrown_13_ahash --demangle target/release/hash_perf > dis/hashbrown_13_ahash
$objdump --disassemble-symbols=${sym_prefix}hashbrown_13_fxhash --demangle target/release/hash_perf > dis/hashbrown_13_fxhash
$objdump --disassemble-symbols=${sym_prefix}hashbrown_12_ahash_fixed --demangle target/release/hash_perf > dis/hashbrown_12_ahash_fixed
$objdump --disassemble-symbols=${sym_prefix}hashbrown_12_ahash --demangle target/release/hash_perf > dis/hashbrown_12_ahash
$objdump --disassemble-symbols=${sym_prefix}hashbrown_12_fxhash --demangle target/release/hash_perf > dis/hashbrown_12_fxhash
