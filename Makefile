APP=`cat Cargo.toml | grep name | cut -c 8- | sed 's/"//g'`

build:
	cargo build --release
	mv ./target/release/$(APP) ./$(APP)

clean:
	cargo clean
	rm -rf ./target ./$(APP)
