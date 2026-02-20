void stars(int32 height, bool backwards)
{
    for (int32 i = 0; i <= height; i++) {

        if (!backwards) {
            for (int32 j = 0; j < i; j++) {
                sprint("*");
            }
        }

        if (backwards) {
            for (int32 j = height - i; j >= 0; j--) {
                sprint(" ");
            }

            for (int32 j = 0; j < i; j++) {
                sprint("*");
            }
        }

        sprint("\n");
    }
}

void main()
{
    sprint("Enter stars height: ");
    int32 h = iread();

    sprint("Backwards? (1 for yes, 0 for no): ");
    int32 b_input = iread();

    bool b = false;

    if (b_input == 1) {
        b = true;
    }

    sprint("\n");

    stars(h, b);
}