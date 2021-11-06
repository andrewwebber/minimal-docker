libs:
	mkdir -p ./libs
	ldd ./target/release/app | grep "=>" | awk '{print $$3}' | while read lib; do cp $$lib ./libs/ ; done;
	cp /usr/lib/libdns_sd.so ./libs
	cp /usr/lib/libresolv.so.2 ./libs
	cp /usr/lib/libnss_files.so.2 ./libs
	cp /usr/lib/libnss_dns.so.2 ./libs

target/release/app:
	cargo build --release

docker-image: target/release/app libs
	docker build -t app .

