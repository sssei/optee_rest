
.PHONY: optee_server-rs optee_tls_server client-rs http_client-rs server-rs install scp clean 

optee_server-rs: 
	make -C optee_server-rs

optee_tls_server-rs: 
	make -C optee_tls_server-rs 

client-rs: 
	make -C client-rs 

http_client-rs:
	make -C http_client-rs	

server-rs:
	make -C server-rs	

install:
	install -D optee_tls_server-rs/host/target/aarch64-unknown-linux-gnu/release/optee_tls_server-rs -t out/host/ || :
	install -D optee_tls_server-rs/ta/target/aarch64-unknown-optee-trustzone/release/*.ta -t out/ta/  || :
	install -D optee_socket-rs/host/target/aarch64-unknown-linux-gnu/release/optee_socket-rs -t out/host/ || :
	install -D optee_socket-rs/ta/target/aarch64-unknown-optee-trustzone/release/*.ta -t out/ta/  || :
	install -D optee_server-rs/host/target/aarch64-unknown-linux-gnu/release/optee_server-rs -t out/host/
	install -D optee_server-rs/ta/target/aarch64-unknown-optee-trustzone/release/*.ta -t out/ta/
	install -D optee_file-rs/host/target/aarch64-unknown-linux-gnu/release/optee_file-rs -t out/host/
	install -D optee_file-rs/ta/target/aarch64-unknown-optee-trustzone/release/*.ta -t out/ta/
	install -D http_client-rs/target/aarch64-unknown-linux-gnu/release/http_client-rs -t out/host  || :	
	install -D client-rs/target/aarch64-unknown-linux-gnu/release/client-rs -t out/host  || :

scp:
	scp out/host/* atde:/home/atmark/workspace/optee/out/ca/
	scp out/ta/* atde:/home/atmark/workspace/optee/out/ta/

clean: 
	make -C client-rs clean 
	make -C optee_tls_server-rs clean 
	make -C optee_server-rs clean 
	make -C server-rs clean 
	make -C http_client clean 
	rm -rf out