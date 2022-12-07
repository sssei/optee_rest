all: tls_server client 

optee_server:
	make -C optee_server-rs

tls_server: 
	make -C tls_server-rs 

client: 
	make -C client-rs 

install:
	install -D tls_server-rs/host/target/aarch64-unknown-linux-gnu/release/tls_server-rs -t out/host/ 
	install -D tls_server-rs/ta/target/aarch64-unknown-optee-trustzone/release/*.ta -t out/ta/ 
	install -D optee_server-rs/host/target/aarch64-unknown-linux-gnu/release/optee_server-rs -t out/host/
	install -D optee_server-rs/ta/target/aarch64-unknown-optee-trustzone/release/*.ta -t out/ta/
	install -D client-rs/target/aarch64-unknown-linux-gnu/release/client-rs -t out/host  

scp:
	scp out/host/* atde:/home/atmark/workspace/optee/out/ca/
	scp out/ta/* atde:/home/atmark/workspace/optee/out/ta/

clean: 
	make -C client-rs clean 
	make -C tls_server-rs clean 
	make -C optee_server-rs clean 
	rm -rf out