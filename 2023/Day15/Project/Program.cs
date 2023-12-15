using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Runtime.Versioning;
using System.Data;


class Program
{
    static string[] lines = new string[0];

    static int Hash(string s)
    {
        int hash = 0;
        foreach(var c in s)
        {
            hash += (int)c;
            hash *= 17;
            hash %= 256;
        }

        return hash;
    }

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        int total = 0;
        foreach(var config in lines[0].Split(','))
            total += Hash(config);

        Console.WriteLine(total);
        StarOne();
        //StarTwo();
    }

    static void StarOne()
    {
        
    }

    
    static void StarTwo()
    {
       
    }
}
