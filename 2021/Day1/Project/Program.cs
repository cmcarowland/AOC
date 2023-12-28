
using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Program
{
    static string[] lines = new string[0];

    public static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        StarOne();
        StarTwo();
    }

    public static void StarOne()
    {
        int count = 0;
        var numbers = lines.Select(x => int.Parse(x)).ToArray();
        for(int i = 1; i < numbers.Length; i++)
        {
            if(numbers[i - 1] < numbers[i])
                count++;
        }

        Console.WriteLine("Increases : {0}", count);
    }

    public static void StarTwo()
    {
        
    }
}

