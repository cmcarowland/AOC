using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Collections.ObjectModel;
using System.Threading;
using System.Resources;
using System.Security.Permissions;
using System.Text;
using System.Runtime.InteropServices;
using System.Security.Cryptography;

public class Gear
{
    public int row;
    public int index;
    public List<Part> adjacentParts = new List<Part>();

    public Gear(int r, int i)
    {
        row = r;
        index = i;
    }

    public int GetRatio()
    {
        int ratio = 1;
        foreach(var ap in adjacentParts)
            ratio *= ap.value;

        return ratio;
    }

    public override string ToString()
    {
        return $"Row: {row} Index: {index} Ratio: {GetRatio()}";
    }
}

public class Part
{
    public int row; 
    public int startIndex = 0;
    public int endIndex = 0;
    public int value;
    public Gear gear;

    public Part(int r, int start, int end, int v, Gear g = null)
    {
        row = r;
        startIndex = start;
        endIndex = end;
        value = v;
        gear = g;
    }

    public bool IsEnginePart()
    {
        return false;
    }

    public override string ToString()
    {
        return string.Format($"Start: {startIndex} EndIndex: {endIndex} Row: {row} Value: {value}");
    }
}

public class Grid
{
    public List<char[]> rows = new List<char[]>();
    public List<Part> parts = new List<Part>();
    public List<Gear> gears = new List<Gear>();

    public Grid()
    {
        rows = new List<char[]>();
        foreach(var line in Program.lines)
        {
            rows.Add(line.ToCharArray());
        }

        for(int r = 0; r < rows.Count; r++)
        {
            for(int i = 0; i < rows[r].Length; i++)
            {
                if(rows[r][i] == '*')
                    gears.Add(new Gear(r, i));

                int charVal = (int)rows[r][i];
                List<char> partNum = new List<char>();
                if(charVal >= 0x30 && charVal <= 0x39)
                {
                    int startIndex = i;
                    partNum.Add(rows[r][i]);
                    int j = i + 1;
                    while(true)
                    {
                        if(j < rows[r].Length)
                            charVal = (int)rows[r][j];

                        if(j < rows[r].Length && charVal >= 0x30 && charVal <= 0x39)
                        {
                            partNum.Add(rows[r][j]);
                            j++;
                        }
                        else
                        {
                            if(partNum.Count > 0)
                            {
                                parts.Add(new Part(r, startIndex, j - 1, int.Parse(string.Join("", rows[r].Skip(i).Take(j - i)))));
                            }
                            i = j - 1;
                            break;
                        }
                    }
                }

            }
        }
    }

    public override string ToString()
    {
        StringBuilder sb = new StringBuilder();
        foreach(var row in rows)
        {
            sb.AppendLine(string.Join(' ', row));
        }
        return sb.ToString();
    }

    public bool CheckTopLeft(int row, int index, Part p)
    {
        if(index == 0 || row == 0)
            return false;

        int charVal = (int)rows[row - 1][index - 1];
        if(rows[row - 1][index - 1] == '*')
        {
            gears.First(x => x.row == row - 1 && x.index == index - 1).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }
   
    public bool CheckTopRight(int row, int index, Part p)
    {
        if(index == rows[row].Length - 1 || row == 0)
            return false;

        int charVal = (int)rows[row - 1][index + 1];
         if(rows[row - 1][index + 1] == '*')
        {
            gears.First(x => x.row == row - 1 && x.index == index + 1).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }

    public bool CheckBottomLeft(int row, int index, Part p)
    {
        if(index == 0 || row == rows.Count - 1)
            return false;

        int charVal = (int)rows[row + 1][index - 1];
         if(rows[row + 1][index - 1] == '*')
        {
            gears.First(x => x.row == row + 1 && x.index == index - 1).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }
   
    public bool CheckBottomRight(int row, int index, Part p)
    {
        if(index == rows[row].Length - 1 || row == rows.Count - 1)
            return false;

        int charVal = (int)rows[row + 1][index + 1];
         if(rows[row + 1][index + 1] == '*')
        {
            gears.First(x => x.row == row + 1 && x.index == index + 1).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }

    public bool CheckLeft(int row, int index, Part p)
    {
        if(index == 0)
            return false;

        int charVal = (int)rows[row][index - 1];
         if(rows[row][index - 1] == '*')
        {
            gears.First(x => x.row == row && x.index == index - 1).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }

    public bool CheckRight(int row, int index, Part p)
    {
        if(index == rows[row].Length - 1)
            return false;

        int charVal = (int)rows[row][index + 1];
         if(rows[row][index + 1] == '*')
        {
            gears.First(x => x.row == row && x.index == index + 1).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }

    public bool CheckAbove(int row, int index, Part p)
    {
        if(row == 0)
            return false;

        int charVal = (int)rows[row - 1][index];
         if(rows[row - 1][index] == '*')
        {
            gears.First(x => x.row == row - 1 && x.index == index).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }
    
    public bool CheckBelow(int row, int index, Part p)
    {
        if(row == rows[row].Length - 1)
            return false;

        int charVal = (int)rows[row + 1][index];
         if(rows[row + 1][index] == '*')
        {
            gears.First(x => x.row == row + 1 && x.index == index).adjacentParts.Add(p);
        }

        return charVal != 0x2e && (charVal < 0x30 || charVal > 0x39);
    }

    public bool IsSymbolAdjacent(Part p)
    {
        for(int i = p.startIndex; i <= p.endIndex; i++)
        {
            if(CheckAbove(p.row, i, p))
                return true;

            if(CheckBelow(p.row, i, p))
                return true;
        }

        if(CheckLeft(p.row, p.startIndex, p) || 
            CheckRight(p.row, p.endIndex, p) || 
            CheckBottomLeft(p.row, p.startIndex, p) || 
            CheckTopLeft(p.row, p.startIndex, p) ||
            CheckBottomRight(p.row, p.endIndex, p) || 
            CheckTopRight(p.row, p.endIndex, p)
        )
            return true;

        return false;
    }
}

public class Program
{
    public static string[] lines = new string[0];

    public static void Main()
    {
        lines = File.ReadAllLines(@"Input.txt");
        // Console.WriteLine(StarOne());
        Console.WriteLine(StarTwo());
    }

    public static int StarOne()
    {
        Grid g = new Grid();
        List<Part> goodParts = new List<Part>();
        foreach(var p in g.parts)
        {
            if(g.IsSymbolAdjacent(p))
            {
                //Console.WriteLine(p);
                goodParts.Add(p);
            }
        }

        return goodParts.Sum(x => x.value);
    }

    public static int StarTwo()
    {
        Grid g = new Grid();
        List<Gear> gears = new List<Gear>();
        foreach(var p in g.parts)
            g.IsSymbolAdjacent(p);

        foreach(var gear in g.gears)
        {
            if(gear.adjacentParts.Count == 2)
                gears.Add(gear);
        }

        return gears.Sum(x => x.GetRatio());
    }
}
