using System;
using System.IO;
using System.Collections.Generic;

public class Program
{
    static Dictionary<string, Tuple<string, string>> nodes = new Dictionary<string, Tuple<string, string>>();
    static string directions = "";

    static void Main(string[] argv)
    {
        var lines = File.ReadAllLines(argv[0]);
        directions = lines[0].Trim();
        foreach(var node in lines.Skip(2))
        {
            var split = node.Split(" = (");
            var lr = split[1].Replace(")", "").Split(", ");
            nodes.Add(split[0].Trim(), new Tuple<string, string>(lr[0], lr[1]));
        }

        // StarOne();
        StarTwo();
    }

    static void StarOne()
    {
        var kvp = nodes.First(x => x.Key == "AAA");
        var end = nodes.First(x => x.Key == "ZZZ");
        
        int count = 0;
        while(kvp.Key != end.Key)
        {
            char nextMove = directions[count % directions.Length];
            if(nextMove == 'L')
            {
                kvp = nodes.First(x => x.Key == kvp.Value.Item1);
            }
            else
            {
                kvp = nodes.First(x => x.Key == kvp.Value.Item2);
            }

            count ++;
        }

        Console.WriteLine(count);
    }

    static long CalculateGCD (long smaller, long larger)
    {
        if (larger < smaller)
        {
            long tmp = smaller;
            smaller = larger;
            larger = tmp;
        }
    
        //Euclid's Division
        while (true)
        {
            long remainder = larger % smaller;
            if (remainder == 0)
                return smaller;

            larger = smaller;
            smaller = remainder;
        }
    }

    static long CalculateLCM (long A, long B)
    {
        // LCM(A,B) = (A/GCD(A,B))*B
        return (A / CalculateGCD(A,B)) * B;
    }
    
    static void StarTwo()
    {
        var kvps = nodes.Where(x => x.Key.EndsWith('A')).Select(x => x).ToList();

        int[] counts = new int[kvps.Count];
        for(int i = 0; i < counts.Length; i++)
        {
            var currentKvp = kvps.Skip(i).Take(1).First();
            while(!currentKvp.Key.EndsWith('Z'))
            {
                char nextMove = directions[counts[i] % directions.Length];
                if(nextMove == 'L')
                {
                    currentKvp = nodes.First(x => x.Key == currentKvp.Value.Item1);
                }
                else
                {
                    currentKvp = nodes.First(x => x.Key == currentKvp.Value.Item2);
                }

                counts[i] ++;
            }
        }

        long lcm = counts[0];
        for(int i = 1; i < counts.Length; i ++)
        {
            lcm = CalculateLCM(lcm, counts[i]);
        }
            
        Console.WriteLine(lcm);
    }
}
