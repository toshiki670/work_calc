install:
	git switch main
	cargo install --path .
	git switch develop
