using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;

public class Program
{
    static string[] lines = null;
    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        

        StarOne();
        StarTwo();
    }

    static int[] SubNext(int[] pattern)
    {
        int[] seq = new int[pattern.Length - 1];
        for(int i = 0; i < seq.Length - 1; i ++)
        {
            seq[i] = pattern[i + 1] - pattern[i];
        }
        Console.WriteLine(string.Join(',', seq));
        if(seq.Count(x => x == 0) == seq.Length)
            return seq;
        
        // Console.WriteLine("{0} {1}", pattern[pattern.Length - 1], SubNext(seq).Last());
        seq[seq.Length - 1] = seq[seq.Length - 2] + SubNext(seq).Last();
    
        Console.WriteLine(string.Join(',', seq));
        return seq;
    }

    static int GetSequence(int[] pattern)
    {
        int[] seq = new int[pattern.Length + 1];
        for(int i = 0; i < pattern.Length; i++)
            seq[i] = pattern[i];

        return pattern.Last() + SubNext(seq).Last(); 
    }

    static void StarOne()
    {
        int total = 0;
        foreach(var line in lines)
        {
            var split = line.Split(' ').Select(x => int.Parse(x)).ToArray();            
            var i = GetSequence(split);
            Console.WriteLine(i);
            total += i;
        }

        Console.WriteLine("Star One: {0}", total);
    }
    
    static void StarTwo()
    {
         int total = 0;
        foreach(var line in lines)
        {
            var split = line.Split(' ').Select(x => int.Parse(x)).ToArray();
            Array.Reverse(split, 0, split.Length);
            var i = GetSequence(split);
            Console.WriteLine(i);
            total += i;
        }

        Console.WriteLine("Star Two: {0}", total);
    }
}
