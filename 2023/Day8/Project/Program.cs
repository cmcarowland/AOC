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
    
    static void StarTwo()
    {
        var kvp1 = nodes.First(x => x.Key.EndsWith('A'));
        var kvp2 = nodes.First(x => x.Key.EndsWith('A') && x.Key != kvp1.Key);

        Console.WriteLine(kvp1);
        Console.WriteLine(kvp2);
        return;        
        // int count = 0;
        // while(kvp.Key != end.Key)
        // {
        //     char nextMove = directions[count % directions.Length];
        //     if(nextMove == 'L')
        //     {
        //         kvp = nodes.First(x => x.Key == kvp.Value.Item1);
        //     }
        //     else
        //     {
        //         kvp = nodes.First(x => x.Key == kvp.Value.Item2);
        //     }

        //     count ++;
        // }

        // Console.WriteLine(count);
    }
}
