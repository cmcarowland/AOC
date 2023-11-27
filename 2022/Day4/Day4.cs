using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Pair
{
    int min;
    int max;

    public Pair(string input)
    {
        var splitString = input.Split('-');
        min = int.Parse(splitString[0]);
        max = int.Parse(splitString[1]);
    }

    public override string ToString()
    {
        return string.Format("Min {0} Max {1}", min, max);
    }

    public bool Contains(Pair other)
    {
        return min <= other.min && max >= other.max;
    }

    public bool Overlap(Pair other)
    {
        return (min >= other.min && min <= other.max) || (max >= other.min && max <= other.max);
    }
}

public class Program
{
    public static void Main()
    {
        var lines = File.ReadAllLines(@"2022\Day4\" + "Input.txt");
        Console.WriteLine(RoundOne(lines));
        Console.WriteLine(StarTwo(lines));
    }

    public static int RoundOne(string[] lines)
    {
        int count = 0;
        foreach(var line in lines)
        {
            var splitLines = line.Split(',');
            Pair p1 = new Pair(splitLines[0]);
            Pair p2 = new Pair(splitLines[1]);
            
            if(p1.Contains(p2) || p2.Contains(p1))
                count ++;
        }

        return count;
    }

    public static int StarTwo(string[] lines)
    {
        int count = 0;
        foreach(var line in lines)
        {
            var splitLines = line.Split(',');
            Pair p1 = new Pair(splitLines[0]);
            Pair p2 = new Pair(splitLines[1]);
            
            if(p1.Overlap(p2) || p2.Overlap(p1))
                count ++;
        }

        return count;
    }
}
