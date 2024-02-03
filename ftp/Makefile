.PHONY: 4700ftp
4700ftp:
	curl https://sh.rustup.rs -sSf | sh -s -- -y \
	&& ~/.cargo/bin/rustup install --profile=minimal 1.75.0 \
	&& ~/.cargo/bin/rustup default 1.75.0 \
	&& ~/.cargo/bin/cargo build