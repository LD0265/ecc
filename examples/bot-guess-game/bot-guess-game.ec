int32 abs(int32 n)
{
    if (n < 0) {
        n = n * -1;
    }

    return n;
}

int32 getGuess(int32 min, int32 max)
{
    int32 sum = min + max;

    // Hacky fix for negatives
    if (sum < 0) {
        sum = abs(sum);
        sum>>;
        sum = sum * -1;
        return sum;
    }

    // Right shift acts like unary
    // sum >> 1; will be valid in the future

    sum>>;
    return sum;
}

int32 askQuestion(int32 guess)
{
    sprint("Is the number ");
    iprint(guess);
    sprint("? (0 for yes, 1 for lower, 2 for higher): ");
    int32 input = iread();
    return input;
}

int32 promptMin()
{
    sprint("Enter range min: ");
    int32 n = iread();
    return n;
}

int32 promptMax()
{
    sprint("Enter range max: ");
    int32 n = iread();
    return n;
}

void main()
{
    int32 RANGE_MIN = promptMin();
    int32 RANGE_MAX = promptMax();

    bool playAgain = true;

    sprint("\n");

    while (playAgain) {
        int32 leftBound = RANGE_MIN;
        int32 rightBound = RANGE_MAX;

        int32 numGuesses = 0;

        sprint("Think of a number between ");
        iprint(leftBound);
        sprint(" and ");
        iprint(rightBound);
        sprint("\n");

        int32 questionRes = 1;
        int32 guess = getGuess(leftBound, rightBound);

        while (questionRes != 0) {
            questionRes = askQuestion(guess);

            if (questionRes == 1) {
                rightBound = guess - 1;
            }

            if (questionRes == 2) {
                leftBound = guess + 1;
            }

            guess = getGuess(leftBound, rightBound);
            numGuesses++;
        }

        sprint("Hooray, I guessed the number!\n");
        sprint("It took me ");
        iprint(numGuesses);
        sprint(" guesses\n\nWould you like to play again? (1 for yes, 0 for no): ");
        int32 playAgainInput = iread();

        playAgain = true;
        if (playAgainInput == 0) {
            playAgain = false;
        }

        sprint("\n");
    }
}