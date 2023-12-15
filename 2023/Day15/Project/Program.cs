using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Runtime.Versioning;
using System.Data;

class Lense
{
    public string label;
    public int boxNumber;
    public int focalLength;

    public Lense(string l, int bn, int fl)
    {
        label = l;
        boxNumber = bn;
        focalLength = fl;
    }

    public int FocusingPower(int slot)
    {
        return (boxNumber + 1) * slot * focalLength;
    }

    public override string ToString()
    {
        return string.Format("[{0} {1}]", label, focalLength);
    }
}

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
        
        StarOne();
        StarTwo();
    }

    static void StarOne()
    {
        int total = 0;
        foreach(var config in lines[0].Split(','))
            total += Hash(config);

        Console.WriteLine(total);
    }

    static string GetLabel(string s)
    {
        if(s.Contains('-'))
            return s.Replace("-", "");
        
        var splitLine = s.Split("=");
        return splitLine[0];
    }

    static void StarTwo()
    {
        Dictionary<int, List<Lense>> boxes = new Dictionary<int, List<Lense>>();
        for(int i = 0; i < 256; i++)
            boxes.Add(i, new List<Lense>());

        foreach(var config in lines[0].Split(','))
        {
            int box = Hash(GetLabel(config));
            if(config.EndsWith('-'))
            {
                try
                {
                    var lense = boxes[box].First(x => x.label == config.Replace("-", ""));
                    boxes[box].Remove(lense);
                }
                catch {}
            }
            else
            {
                var splitLine = config.Split("=");
                var lense = boxes[box].FirstOrDefault(x => x.label == splitLine[0]);
                if(lense == null)
                {
                    boxes[box].Add(new Lense(splitLine[0], box, int.Parse(splitLine[1])));
                }
                else
                {
                    lense.focalLength = int.Parse(splitLine[1]);
                }
            }
        }

        int total = 0;
        foreach(var kvp in boxes)
        {

            if(kvp.Value.Count > 0)
            {
                for(int i = 0; i < kvp.Value.Count; i++)
                    total += kvp.Value[i].FocusingPower(i + 1);
            }
        }

        Console.WriteLine(total);
    }
}
