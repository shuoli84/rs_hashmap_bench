make:
	cargo build --release
	mkdir -p dis
	
	objdump --disassemble=std_ahash_fixed -C target/release/hash_perf > dis/std_ahash_fixed
	objdump --disassemble=std_ahash -C target/release/hash_perf > dis/std_ahash
	objdump --disassemble=std_fxhash -C target/release/hash_perf > dis/std_fxhash
	objdump --disassemble=hashbrown_13_ahash_fixed -C target/release/hash_perf > dis/hashbrown_13_ahash_fixed
	objdump --disassemble=hashbrown_13_ahash -C target/release/hash_perf > dis/hashbrown_13_ahash
	objdump --disassemble=hashbrown_13_fxhash -C target/release/hash_perf > dis/hashbrown_13_fxhash
	objdump --disassemble=hashbrown_12_ahash_fixed -C target/release/hash_perf > dis/hashbrown_12_ahash_fixed
	objdump --disassemble=hashbrown_12_ahash -C target/release/hash_perf > dis/hashbrown_12_ahash
	objdump --disassemble=hashbrown_12_fxhash -C target/release/hash_perf > dis/hashbrown_12_fxhash
