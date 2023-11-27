using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Program
{
    // a - z = 1 - 26
    // A - Z = 27 - 52
    public static void Main()
    {
        var lines = File.ReadAllLines(@"2022\Day3\" + "Input.txt");
        int score = 0;
        for(int i = 0; i < lines.Length; i += 3)
        {
            score += RoundTwo(lines.Skip(i).Take(3).ToArray());
        }

        Console.WriteLine(score);
    }
    
    public static int RoundTwo(string[] lines)
    {
        foreach(char c in lines[0])
        {
            if(lines[1].Contains(c) && lines[2].Contains(c))
                return GetPriority(c);
        }

        return -1;
    }

    public static int GetPriority(char c)
    {
        int current = (int)c;
        if(current >= 0x61)
        {
            return current - 0x60;
        }
        else
        {
            return current - 0x40 + 26;
        }
    }

    public static int RoundOne(string line)
    {
        List<char> likeItems = new List<char>();

        int bagContents = line.Length;
        string first = line.Substring(0, bagContents / 2);
        string second = line.Substring(bagContents / 2, bagContents / 2);
        
        foreach(var c in first)
        {
            if(second.Contains(c))
            {
                likeItems.Add(c);
                break;
            }
        }

        int score = 0;
        foreach(var c in likeItems)
        {
            score += GetPriority(c);
        }

        return score;
    }
}
