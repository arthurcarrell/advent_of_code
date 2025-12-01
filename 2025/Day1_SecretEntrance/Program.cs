using System;
class Program
{
    static int GetDialMovement(string line)
    {
        // clean the line up
        line = line.Trim();

        // get the modifier
        int modifier = 0;
        if (line[0].Equals('L')) modifier = -1;
        else if (line[0].Equals('R')) modifier = 1;

        // now get the rest of the characters in the line
        string stringDigits = line.Substring(1);
        int amount = 0;

        // multiply the amount by the modifier
        if (Int32.TryParse(stringDigits, out amount))
        {
            return amount * modifier;
        }
        return 0;
    }

    static void Main()
    {
        Console.Clear();

        int dial = 50;
        int dialAtZero = 0;

        // run through puzzle input line-by-line and get the dial at zero amount
        using (StreamReader reader = new StreamReader("puzzle_input.txt"))
        {
            string? line;
            while ((line = reader.ReadLine()) != null)
            {
                // get the dial amount
                dial += GetDialMovement(line);

                // is the dial out of bounds? Keep it in bounds
                while (dial < 0) dial += 100;
                while (dial > 99) dial -= 100;

                // is the dial at 0? If so, then record it
                if (dial == 0) dialAtZero++;
            }
        }

        Console.WriteLine($"The password is {dialAtZero}");
    }
}
