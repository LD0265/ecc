int32 fib(int32 n)
{
    if (n < 2) {
        return n;
    }

    int32 a = 0;
    int32 b = 1;
    
    for (int32 i = 2; i <= n; i++) {
        int32 c = a + b;
        a = b;
        b = c;
    }

    return b;
}

void main()
{
    for (int32 i = 0; i < 10; i++) {
        int32 nth = fib(i);
        iprint(nth);
        sprint("\n");
    }
}
