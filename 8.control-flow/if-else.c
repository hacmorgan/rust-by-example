#include <stdio.h>

int main()
{
    int n = 5;

    if ( n < 0 ) {
        printf( "%d is negative", n );
    } else if ( n > 0 ) {
        printf( "%d is positive", n );
    } else {
        printf( "%d is zero", n );
    }

    int big_n;
    if ( n < 10 && n > -10 ) {
        printf( ", and is a small number, increase ten-fold\n" );
        big_n = 10 * n;
    } else {
        printf( ", and is a big number, halve the number\n" );
        big_n = n / 2;
    }

    printf( "%d -> %d\n", n, big_n );
}
