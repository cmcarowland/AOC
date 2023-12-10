using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Security.Permissions;
using System.Security.Cryptography.X509Certificates;

public enum PipeType : byte
{
    Invalid,
    Vertical,
    Horizontal,
    L,
    F,
    J,
    Seven,
    Start
}

public enum LastMove : byte
{
    None,
    Down,
    Right,
    Up,
    Left
}

public class Grid
{
    public List<char[]> rows = new List<char[]>();

    public Grid(string[] lines)
    {
        foreach(var line in lines)
        {
            rows.Add(line.ToCharArray());
        }
    }

    public PipeType GetCharPipeType(char c)
    {
        switch(c)
        { 
            case '|':
                return PipeType.Vertical;
            case '-':
                return PipeType.Horizontal;
            case 'L':
                return PipeType.L;
            case 'F':
                return PipeType.F;
            case '7':
                return PipeType.Seven;
            case 'J':
                return PipeType.J;
            case 'S':
                return PipeType.Start;
        }

        return PipeType.Invalid;
    }

    public char GetCharAtPos(int row, int col)
    {
        return rows[row][col];
    }

    public int GetStartRow()
    {
        for(int i = 0; i < rows.Count; i++)
        {
            if(rows[i].Contains('S'))
                return i;
        }

        return 0;
    }

    public int GetStartCol()
    {
        int row = GetStartRow();
        return Array.FindIndex(rows[row], x => x == 'S');
    }

    public bool IsAboveValid(int row, int col)
    {
        if(row == 0)
            return false;

        var myType = GetCharPipeType(rows[row][col]);
        if(myType != PipeType.Start && (myType != PipeType.Vertical && myType != PipeType.L && myType != PipeType.J))
            return false;

        var pt = GetCharPipeType(rows[row - 1][col]);
        if(pt != PipeType.Invalid)
        {
            if(pt == PipeType.Start || pt == PipeType.Vertical || pt == PipeType.F || pt == PipeType.Seven)
            {
                return true;
            }
        }

        return false;
    }
    
    public bool IsRightValid(int row, int col)
    {
        if(col == rows[0].Length - 1)
            return false;

        var myType = GetCharPipeType(rows[row][col]);
        if(myType != PipeType.Start && (myType != PipeType.Horizontal && myType != PipeType.F && myType != PipeType.L))
            return false;

        var pt = GetCharPipeType(rows[row][col + 1]);
        if(pt != PipeType.Invalid)
        {
            if(pt == PipeType.Start || pt == PipeType.Horizontal || pt == PipeType.J || pt == PipeType.Seven)
            {
                return true;
            }
        }

        return false;
    }
    
    public bool IsDownValid(int row, int col)
    {
        if(row == rows[0].Length - 1)
            return false; 

        var myType = GetCharPipeType(rows[row][col]);
        if(myType != PipeType.Start && (myType != PipeType.Vertical && myType != PipeType.F && myType != PipeType.Seven))
            return false;

        var pt = GetCharPipeType(rows[row + 1][col]);
        if(pt != PipeType.Invalid)
        {
            if(pt == PipeType.Start || pt == PipeType.Vertical || pt == PipeType.J || pt == PipeType.L)
            {
                return true;
            }
        }

        return false;
    }

    public bool IsLeftValid(int row, int col)
    {
        if(col == 0)
            return false;

        var myType = GetCharPipeType(rows[row][col]);
        if(myType != PipeType.Start && (myType != PipeType.Horizontal && myType != PipeType.J && myType != PipeType.Seven))
            return false;

        var pt = GetCharPipeType(rows[row][col - 1]);
        if(pt != PipeType.Invalid)
        {
            if(pt == PipeType.Start || pt == PipeType.Horizontal || pt == PipeType.F || pt == PipeType.L)
            {
                return true;
            }
        }

        return false;
    }
}

public class Program
{
    static string[] lines = null;
    static Grid grid;

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        grid = new Grid(lines);

        StarOne();
        StarTwo();
    }

    static void StarOne()
    {
        int total = 0;
        int row = grid.GetStartRow();
        int col = grid.GetStartCol();
        var last = LastMove.None;
        Console.WriteLine("Row: {0} Col: {1}", row, col);

        while(total == 0 || grid.GetCharPipeType(grid.GetCharAtPos(row, col)) != PipeType.Start)
        {
            // Console.WriteLine("{0} {1} {2} {3}",grid.IsAboveValid(row, col), grid.IsDownValid(row, col), grid.IsLeftValid(row, col), grid.IsRightValid(row, col));
            if(last != LastMove.Down && grid.IsAboveValid(row, col))
            {
                row --;
                last = LastMove.Up;
            }
            else if(last != LastMove.Up && grid.IsDownValid(row, col))
            {
                row ++;
                last = LastMove.Down;
            }
            else if(last != LastMove.Left && grid.IsRightValid(row, col))
            {
                col ++;
                last = LastMove.Right;
            }
            else if(last != LastMove.Right && grid.IsLeftValid(row, col))
            {
                col --;
                last = LastMove.Left;
            }

            total ++;
            // Console.WriteLine("Row: {0} Col: {1}", row, col);
            // Console.WriteLine(" {0}", grid.GetCharPipeType(grid.GetCharAtPos(row, col)), col);
        }

        Console.WriteLine("Star One: {0}", total / 2f);
    }
    
    static void StarTwo()
    {
        int total = 0;

        Console.WriteLine("Star Two: {0}", total);
    }
}
