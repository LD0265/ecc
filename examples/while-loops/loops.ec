void main()
{
    int32 n;
    n = 10;

    int32 n2;

    while (n > 0) {
        n2 = 0;
        
        while (n2 < n) {
            n2 = n2 + 1;
        }

        n = n - 1;
    }
}