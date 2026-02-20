int32 printHint(int32 guess, int32 secret)
{
    if (guess < secret) {
        sprint("Too low!\n");
        return 1;
    }

    if (guess > secret) {
        sprint("Too high!\n");
        return 1;
    }

    return 0;
}

void main()
{
    int32 playAgain = 1;
    int32 upper = 10000;

    while (playAgain > 0) {
        int32 numGuesses = 0;
        int32 res = 1;
        int32 secret = irandrange(40, upper);
        secret++;
        int32 guess = 0;

        while (res != 0) {
            sprint("Guess a number between 1 and ");
            iprint(upper);
            sprint(": ");

            guess = iread();
            res = printHint(guess, secret);
            sprint("\n");

            numGuesses++;
        }

        sprint("You found the number!: ");
        iprint(secret);

        sprint("\nIt took you ");
        iprint(numGuesses);
        sprint(" guesses!\n");

        sprint("\n\nPlay again? (1 for yes, 0 for no): ");
        playAgain = iread();
    }
}