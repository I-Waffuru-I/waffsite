default:
	just --list


back:
	cd blogger && cargo run -- "../site" "../tmp"

site:
	cd site && live-server


clean:
	cd blogger && cargo clean
