default:
	just --list


back:
	cd blogger && cargo run -- "../site" "../tmp/blogs"


clean:
	cd blogger && cargo clean
site:
	cd site && live-server
