
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
        int depth = 0;
        int forward = 0;

        foreach (var line in lines)
        {
            var split = line.Split(' ');
            if(split[0] == "forward")
                forward += int.Parse(split[1]);
            else
            {
                if(split[0] == "down")
                    depth += int.Parse(split[1]);
                else
                    depth -= int.Parse(split[1]);
            }
        }

        Console.WriteLine("Depth : {0} Forward : {1} Position {2}", depth, forward, depth * forward);
    }

    public static void StarTwo()
    {
        int depth = 0;
        int forward = 0;
        int aim = 0;

        foreach (var line in lines)
        {
            var split = line.Split(' ');
            var distance = int.Parse(split[1]);
            if(split[0] == "forward")
            {
                forward += distance;
                if(aim > 0)
                    depth += aim * distance;
            }
            else
            {
                if(split[0] == "down")
                    aim += distance;
                else
                    aim -= distance;
            }
        }

        Console.WriteLine("Depth : {0} Forward : {1} Position {2:N0}", depth, forward, depth * forward);
    }
}

