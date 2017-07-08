

lib:
	cargo build
	cp target/debug/libmuon_discovery_net.so java/discovery-network/src/main/resources/linux-x86-64/


publish: version lib
	$(MAKE) -C java package

version:
ifndef VERSION
	$(error VERSION is undefined for Net Discovery Release)
endif
	$(MAKE) -C java version
	echo "done version"
	git commit -m "Update version to $(VERSION )while publishing"
	git push origin

test:
	RUST_TEST_THREADS=1 cargo test

testtc:
	cargo clean
	RUST_TEST_THREADS=1 cargo test-xunit
