APP=`cat Cargo.toml | grep name | cut -c 8- | sed 's/"//g'`
TAG :=

release: TAG := $(shell read -p "Tag: " tag; echo $$tag)

release:
	@git push | :
	#./build_exec.sh
	@git tag ${TAG} | :
	@git push origin --tags | :
	curl -X POST --data-binary "@debian" "https://uploads.github.com/repos/TheMindCompany/kubessh/releases/${TAG}/assets?name=debian"
	curl -X POST --data-binary "@darwin" "https://uploads.github.com/repos/TheMindCompany/kubessh/releases/${TAG}/assets?name=darwin"

build:
	cargo build --release
	mv ./target/release/$(APP) ./$(APP)

install:
	mv $(APP) /usr/local/bin/$(APP)

clean:
	cargo clean
	rm -rf ./target ./$(APP)
