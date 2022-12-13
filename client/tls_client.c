#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <openssl/ssl.h>
#include <time.h>
#include <errno.h>

#define MAX_SIZE 256 * 72 
#define ITER 100

static void measure(SSL* ssl, char* buf, size_t size){
    for(int i = 0; i < 100; i++){
        int len = SSL_write(ssl, buf, size);
        if(len != size){
            perror("SSL_write");
            exit(1);
        }
        len = 0;
        while(1){
            int res = SSL_read(ssl, buf, size);
            len += res;
            if(res < 0){
                perror("SSL_read");
                exit(1);
            }else if(len == size){
                break;
            }
        }
    }
}

static void log_file(long long* elapsed_time){
    FILE* fp;
    char file[50];
    time_t t = time(NULL);
    struct tm *local = localtime(&t);
    sprintf(file, "%04d-%02d-%02d_%02d-%02d", local->tm_year + 1900,
        local->tm_mon + 1, local->tm_mday, local->tm_hour, local->tm_min);
    
    printf("%s\n", file);
    fp = fopen(file, "w");
    for(int i = 1; i <= 62; i++){
        fprintf(fp, "size = %d\n", 256 * i);
        fprintf(fp, "%lld ns\n", elapsed_time[i]);
    }
    fclose(fp);
}

int main (int argc, char *argv[])
{
    int s, result;
    struct sockaddr_in srv_addr;
    char buf[MAX_SIZE];
    SSL_CTX *ctx;
    SSL *ssl;
    struct timespec start, end;
    long long elapsed_time[72];

    ctx = SSL_CTX_new(TLS_client_method());
    SSL_CTX_use_certificate_file(ctx, "cacert.pem", SSL_FILETYPE_PEM);
 
    srv_addr.sin_family = AF_INET;
    srv_addr.sin_port = htons(8089);
    inet_pton(AF_INET, "192.168.12.24", &srv_addr.sin_addr);

    s = socket(AF_INET, SOCK_STREAM, 0);
    ssl = SSL_new(ctx);
    SSL_set_fd(ssl, s);

    result = connect(s, (struct sockaddr *)&srv_addr, sizeof(srv_addr));
    if (result != 0) {
        perror("connect");
        return 1;
    } else {
        memset(buf, 0, MAX_SIZE);
        result = SSL_connect(ssl);
        if (result == 1) {
            for(int i = 1; i <= 62; i++){
                int size = 256 * i;
                printf("size = %d\n", size);
                clock_gettime(CLOCK_MONOTONIC, &start);
                measure(ssl, buf, size);
                clock_gettime(CLOCK_MONOTONIC, &end);
                elapsed_time[i] = 1000 * 1000 * 1000 * (end.tv_sec - start.tv_sec);
                elapsed_time[i] += end.tv_nsec - start.tv_nsec;
                elapsed_time[i] = elapsed_time[i] / ITER;
                printf("elapsed_time = %lld\n", elapsed_time[i]);
            }
        }
    }

    log_file(elapsed_time);
    close(s);
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    return 0;

}