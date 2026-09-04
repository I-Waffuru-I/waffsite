default:
	just --list


back:
	cd blogger && cargo run -- "../site" "../tmp/blogs.txt" "../tmp/blogs"
