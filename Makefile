APP=`cat Cargo.toml | grep name | cut -c 8- | sed 's/"//g'`

release:
	git push
	./build_exec.sh
	echo "What version will this be tagged as?"
	read tag
	git tag $tag
	git push origin --tags

build:
	cargo build --release
	mv ./target/release/$(APP) ./$(APP)

install:
	mv $(APP) /usr/local/bin/$(APP)

clean:
	cargo clean
	rm -rf ./target ./$(APP)
