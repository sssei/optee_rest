tls_client: tls_client.c 
	gcc -Wall tls_client.c -o tls_client -lcrypto -lssl 

http_client: http_client.c 
	gcc -Wall http_client.c -o http_client -lcrypto -lssl 	

socket_client: socket_client.c 
	gcc -Wall socket_client.c -o socket_client 	

relay: relay.c 
	gcc relay.c -o relay -lpthread 
	
clean: 
	rm -rf http_client tls_client 