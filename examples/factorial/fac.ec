// We need a mult function because
// the language only supports 
// addition and subtraction
int32 mult(int32 a, int32 b)
{
    int32 sum = 0;
    for (int32 i = 0; i < b; i++) {
        sum = sum + a;
    }
    return sum;
}

int32 fac(int32 n)
{
    int32 sum2 = 1;
    for (int32 i = 1; i <= n; i++) {
        sum2 = mult(sum2, i);
    }
    return sum2;
}

void main()
{
    sprint("Input a number: ");

    // Integer read
    int32 input = iread();

    int32 input_fac = fac(input);

    sprint("\n");

    iprint(input);
    sprint("!: ");
    iprint(input_fac);
}