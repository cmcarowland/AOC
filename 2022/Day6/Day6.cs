using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Program
{
    public static void Main()
    {
        var lines = File.ReadAllLines(@"2022\Day6\" + "Input.txt");
        Console.WriteLine(StarOne(lines));
        Console.WriteLine(StarTwo(lines));
    }

    public static int StarOne(string[] lines)
    {
        for(int i = 0; i < lines[0].Length; i++)
        {
            var setChar = lines[0].Skip(i).Take(4).Distinct().ToList();
            if(setChar.Count == 4)
            {
                Console.WriteLine(string.Join(",", setChar));
                return i + 4;
            }
        }

        return -1;
    }

    public static int StarTwo(string[] lines)
    {
        for(int i = 0; i < lines[0].Length; i++)
        {
            var setChar = lines[0].Skip(i).Take(14).Distinct().ToList();
            if(setChar.Count == 14)
            {
                Console.WriteLine(string.Join(",", setChar));
                return i + 14;
            }
        }
        return -1;
    }
}
