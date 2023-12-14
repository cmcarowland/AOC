using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Runtime.Versioning;
using System.Data;

class RoundStone
{
    public int col, row = 0;

    public RoundStone(int r, int c)
    {
        row = r;
        col = c;
    }

    public void RollUp(Grid g)
    {
        g.rows[row][col] = '.';
        while(row > 0 && g.rows[row - 1][col] == '.')
            row --;

        g.rows[row][col] = 'O';
    }
    
    public void RollLeft(Grid g)
    {
        g.rows[row][col] = '.';
        while(col > 0 && g.rows[row][col - 1] == '.')
            col --;

        g.rows[row][col] = 'O';
    }
    
    public void RollDown(Grid g)
    {
        g.rows[row][col] = '.';
        while(row < g.rows.Count - 1 && g.rows[row + 1][col] == '.')
            row ++;

        g.rows[row][col] = 'O';
    }
    
    public void RollRight(Grid g)
    {
        g.rows[row][col] = '.';
        while(col < g.rows[row].Length - 1 && g.rows[row][col + 1] == '.')
            col ++;

        g.rows[row][col] = 'O';
    }
}

class Grid
{
    public List<char[]> rows;
    public List<RoundStone> stones = new List<RoundStone>();
    
    public Grid(string[] lines)
    {
        rows = new List<char[]>();
        foreach(var line in lines)
        {
            rows.Add(line.ToCharArray());
        }

        FindRoundStones();
    }

    public void FindRoundStones()
    {
        stones = new List<RoundStone>();
        for(int row = 0; row < rows.Count; row++)
        {
            for(int col = 0; col < rows[row].Length; col++)
            {
                if(rows[row][col] == 'O')
                    stones.Add(new RoundStone(row, col));
            }
        }
    }

    public void ShiftUp()
    {
        stones = stones.OrderBy(x => x.row).ToList();
        foreach(var stone in stones)
            stone.RollUp(this);
    }
    
    public void ShiftWest()
    {
        stones = stones.OrderBy(x => x.col).ToList();
        foreach(var stone in stones)
            stone.RollLeft(this);
    }

    public void ShiftEast()
    {
        stones = stones.OrderByDescending(x => x.col).ToList();
        foreach(var stone in stones)
            stone.RollRight(this);
    }
    
    public void ShiftDown()
    {
        stones = stones.OrderByDescending(x => x.row).ToList();
        foreach(var stone in stones)
            stone.RollDown(this);
    }

    public int CalculateLoad()
    {
        int rowValue = rows.Count;
        int totalLoad = 0;

        for(int row = 0; row < rows.Count; row++)
        {
            for(int col = 0; col < rows[row].Length; col++)
            {
                if(rows[row][col] == 'O')
                {
                    totalLoad += rowValue;
                }
            }
            rowValue--;
        }

        return totalLoad;
    }

    public override string ToString()
    {
        StringBuilder sb = new StringBuilder();

        foreach(var row in rows)
        {
            sb.AppendLine(string.Join("", row));
        }

        return sb.ToString();
    }
}

class Program
{
    static string[] lines = new string[0];

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        
        //StarOne();
        StarTwo();
    }

    static void StarOne()
    {
        Grid g = new Grid(lines);
        g.ShiftUp();
        Console.WriteLine(g);
        Console.WriteLine(g.CalculateLoad());
    }

    //The pattern repeats every 35th time.
    //Just find the pattern that ends in 999999999 and start at a number divisible by 35
    static void StarTwo()
    {
        Grid g = new Grid(lines);
        List<V> loads = new List<V>();
        for(int i = 999999000; i < 1000000000; i++)
        {
            g.ShiftUp();
            g.ShiftWest();
            g.ShiftDown();
            g.ShiftEast();
            loads.Add(new V(g.CalculateLoad(), i));
        }

        var groups = loads.GroupBy(x => x.value).Select(x => new {Value = x.Key, Index=x.Select(y => y.index).ToList(), Count = x.Count()}).Where(x => x.Index.Contains(999999999));
        foreach(var group in groups.OrderBy(x => x.Value))
        {
            Console.WriteLine("{0} {1} {2}", group.Value, string.Join(",", group.Index), group.Count);
        }
        //Console.WriteLine(g);
        //Console.WriteLine(g.CalculateLoad());
    }
}

public struct V
{
    public int value;
    public int index;

    public V(int v, int i)
    {
        value = v;
        index = i;
    }
}