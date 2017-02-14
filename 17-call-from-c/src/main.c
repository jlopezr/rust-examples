#include<stdio.h>

extern char* hello_rust();

int main() {
    printf("Next a Hello World from RUST!\r\n");
    char* buff = hello_rust();
    printf("%s\r\n", buff);
    return 0;
}
