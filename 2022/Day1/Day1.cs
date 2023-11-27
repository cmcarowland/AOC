using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Program
{
    public static void Main()
    {
        int maxElf = 0;
        int currentElf = 0;
        List<int> elves = new List<int>();

        var lines = File.ReadAllLines("Day1Input.txt");
        foreach(var line in lines)
        {
            if(string.IsNullOrWhiteSpace(line))
            {
                if(currentElf > maxElf)
                    elves.Add(currentElf);
            
                currentElf = 0;
            }
            else
            {
                currentElf += int.Parse(line);
            }
        }

        var sortedElves = elves.OrderByDescending(x => x).ToList();
        Console.WriteLine(sortedElves[0]);
        Console.WriteLine(sortedElves[1]);
        Console.WriteLine(sortedElves[2]);
        Console.WriteLine(sortedElves[2] + sortedElves[1] + sortedElves[0]);
    }
}