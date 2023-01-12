#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <openssl/ssl.h>
#include <time.h>
#include <errno.h>

#define MAX_SIZE 256 * 62
#define ITER 100

static void log_file(long long* elapsed_time){
    FILE* fp;
    char file[50];
    time_t t = time(NULL);
    struct tm *local = localtime(&t);
    sprintf(file, "%04d-%02d-%02d_%02d-%02d_http", local->tm_year + 1900,
        local->tm_mon + 1, local->tm_mday, local->tm_hour, local->tm_min);
    
    printf("%s\n", file);
    fp = fopen(file, "w");
    for(int i = 1; i < 62; i++){
        fprintf(fp, "size = %d\n", 256 * i);
        fprintf(fp, "%lld ns\n", elapsed_time[i * 3]);
        fprintf(fp, "%lld ns\n", elapsed_time[i * 3 + 1]);
        fprintf(fp, "%lld ns\n", elapsed_time[i * 3 + 2]);                
    }
    fclose(fp);
}

long long post_request(SSL *ssl, int body_size){
    struct timespec start, end;    
    char post_buf[MAX_SIZE];
    char response_buf[MAX_SIZE];
    memset(post_buf, 0, MAX_SIZE);
    int header_size = sprintf(post_buf, "POST /config HTTP\r\nContent-Length: %5d\r\n\r\n", body_size);
    int request_size = header_size + body_size;
    long long elapsed_time;

    clock_gettime(CLOCK_MONOTONIC, &start);        
    int len = SSL_write(ssl, post_buf, request_size);
    if (len != request_size){
        perror("SSL_write");
        exit(1);
    }
    int res = SSL_read(ssl, response_buf, MAX_SIZE);
    if(res < 0){
        perror("SSL_read");
        exit(1);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);        
    elapsed_time = 1000 * 1000 * 1000 * (end.tv_sec - start.tv_sec);
    elapsed_time += end.tv_nsec - start.tv_nsec;         
    return elapsed_time;
}

long long get_request(SSL *ssl){
    struct timespec start, end;    
    char response_buf[MAX_SIZE];
    memset(response_buf, 0, MAX_SIZE);
    char header[] = "GET /config HTTP\r\nContent-Length: 0\r\n\r\n";
    int header_size = strlen(header);
    long long elapsed_time;

    clock_gettime(CLOCK_MONOTONIC, &start);        
    int len = SSL_write(ssl, header, header_size);
    if (len != header_size){
        perror("SSL_write");
        exit(1);
    }
    int res = SSL_read(ssl, response_buf, MAX_SIZE);
    if(res < 0){
        perror("SSL_read");
        exit(1);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);        
    elapsed_time = 1000 * 1000 * 1000 * (end.tv_sec - start.tv_sec);
    elapsed_time += end.tv_nsec - start.tv_nsec;         
    return elapsed_time;
}

long long delete_request(SSL *ssl){
    struct timespec start, end;
    char response_buf[MAX_SIZE];
    memset(response_buf, 0, MAX_SIZE);
    char header[] = "DELETE /config HTTP\r\nContent-Length: 0\r\n\r\n";
    int header_size = strlen(header);
    long long elapsed_time;

    clock_gettime(CLOCK_MONOTONIC, &start);    
    int len = SSL_write(ssl, header, header_size);
    if (len != header_size){
        perror("SSL_write");
        exit(1);
    }
    int res = SSL_read(ssl, response_buf, MAX_SIZE);
    if(res < 0){
        perror("SSL_read");
        exit(1);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);   
    elapsed_time = 1000 * 1000 * 1000 * (end.tv_sec - start.tv_sec);
    elapsed_time += end.tv_nsec - start.tv_nsec;         
    return elapsed_time;
}

int main (int argc, char *argv[])
{
    int s, result;
    struct sockaddr_in srv_addr;
    SSL_CTX *ctx;
    SSL *ssl;
    long long elapsed_time[3 * 63];

    ctx = SSL_CTX_new(TLS_client_method());
    SSL_CTX_use_certificate_file(ctx, "cacert.pem", SSL_FILETYPE_PEM);
 
    srv_addr.sin_family = AF_INET;
    srv_addr.sin_port = htons(8090);
//    inet_pton(AF_INET, "192.168.12.24", &srv_addr.sin_addr);
    inet_pton(AF_INET, "127.0.0.1", &srv_addr.sin_addr);

    s = socket(AF_INET, SOCK_STREAM, 0);
    ssl = SSL_new(ctx);
    SSL_set_fd(ssl, s);

    result = connect(s, (struct sockaddr *)&srv_addr, sizeof(srv_addr));
    if (result != 0) {
        perror("connect");
        return 1;
    } 
    result = SSL_connect(ssl);
    if (result == 1) {
        for(int i = 1; i < 62; i++){
            int size = i * 256;
            printf("size = %d\n", size);
            elapsed_time[i * 3] = 0;
            elapsed_time[i * 3 + 1] = 0;            
            elapsed_time[i * 3 + 2] = 0;                        
            for(int j = 0; j < 100; j++){
                elapsed_time[i * 3] += post_request(ssl, size);
                elapsed_time[i * 3 + 1] += get_request(ssl);
                elapsed_time[i * 3 + 2] += delete_request(ssl);
            }
            elapsed_time[i * 3] = elapsed_time[i * 3] / 100;
            elapsed_time[i * 3 + 1] = elapsed_time[i * 3 + 1] / 100;            
            elapsed_time[i * 3 + 2] = elapsed_time[i * 3 + 2] / 100;  
            printf("%lld ns\n", elapsed_time[i * 3]);
            printf("%lld ns\n", elapsed_time[i * 3 + 1]);
            printf("%lld ns\n", elapsed_time[i * 3 + 2]);                        
        }
    }

    log_file(elapsed_time);
    close(s);
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    return 0;

}