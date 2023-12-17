using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Security.Cryptography.X509Certificates;
using System.Runtime.CompilerServices;

class Cell
{
    public char cellType;
    public bool isEnergized;

    public Cell(char c)
    {
        cellType = c;
        isEnergized = false;
    }
}

class Row
{
    public List<Cell> cells;

    public Row(string s)
    {
        cells = new List<Cell>();
        foreach(var c in s)
        {
            cells.Add(new Cell(c));
        }
    }
}

class Grid
{
    public List<Row> rows;

    public Grid(string[] lines)
    {
        rows = new List<Row>();
        foreach(var line in lines)
        {
            rows.Add(new Row(line));
        }
    }

    public Cell GetCell(Beam beam)
    {
        return rows[beam.row].cells[beam.col];
    }

    public int CalculateEnergized()
    {
        int total = 0;
        foreach(var row in rows)
        {
            total += row.cells.Count(x => x.isEnergized);
        }

        return total;
    }

    public override string ToString()
    {
        StringBuilder sb = new StringBuilder();
        foreach(var row in rows)
        {
            foreach(var cell in row.cells)
            {
                sb.Append(cell.isEnergized ? '#' : cell.cellType);
            }
            sb.AppendLine();
        }

        return sb.ToString();
    }
}

class Beam
{
    public enum Direction
    {
        Up,
        Down,
        Right,
        Left
    }

    public Direction direction;
    public int row;
    public int col;
    public bool isComplete;

    public Beam(Direction dir)
    {
        direction = dir;
        row = 0;
        col = 0;
    }
   
    public Beam(Beam b, Direction dir)
    {
        direction = dir;
        row = b.row;
        col = b.col;
    }

    public void Move()
    {
        if(isComplete)
            return;

        switch(direction)
        {
            case Direction.Up:
                row --;
                break;
            case Direction.Down:
                row ++;
                break;
            case Direction.Right:
                col ++;
                break;
            case Direction.Left:
                col --;
                break;
        }

        if(row < 0 || row == Program.g.rows.Count || col < 0 || col == Program.g.rows[0].cells.Count)
        {
            isComplete = true;
        }
    }
}

class Program
{
    static string[] lines = new string[0];
    static public Grid g;
    static List<Beam> beams = new List<Beam>();

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        
        StarOne();
        //StarTwo();
    }

    static void ProcessCell(Beam b, Cell c)
    {
        if(c.cellType == '.')
            return;

        if(c.cellType == '/')
        {
            switch(b.direction)
            {
                case Beam.Direction.Up:
                    b.direction = Beam.Direction.Right;
                    break;
                case Beam.Direction.Down:
                    b.direction = Beam.Direction.Left;
                    break;
                case Beam.Direction.Right:
                    b.direction = Beam.Direction.Up;
                    break;
                case Beam.Direction.Left:
                    b.direction = Beam.Direction.Down;
                    break;
            }
        }
        else if(c.cellType == '\\')
        {
            switch(b.direction)
            {
                case Beam.Direction.Up:
                    b.direction = Beam.Direction.Left;
                    break;
                case Beam.Direction.Down:
                    b.direction = Beam.Direction.Right;
                    break;
                case Beam.Direction.Right:
                    b.direction = Beam.Direction.Down;
                    break;
                case Beam.Direction.Left:
                    b.direction = Beam.Direction.Up;
                    break;
            }
        }
        else if(c.cellType == '|')
        {
            if(b.direction == Beam.Direction.Up || b.direction == Beam.Direction.Down)
                return;

            b.direction = Beam.Direction.Up;
            beams.Add(new Beam(b, Beam.Direction.Down));
        }
        else if(c.cellType == '-')
        {
            if(b.direction == Beam.Direction.Right || b.direction == Beam.Direction.Left)
                return;

            b.direction = Beam.Direction.Left;
            beams.Add(new Beam(b, Beam.Direction.Right));
        }
    }

    static void StarOne()
    {
        int total = 0;
        g = new Grid(lines);
        beams.Add(new Beam(Beam.Direction.Right));
        Cell c = g.GetCell(beams.First());
        c.isEnergized = true;
        ProcessCell(beams.First(), c);
        
        Console.WriteLine(g);
        while(beams.Count(x => x.isComplete) != beams.Count)
        {
            total = g.CalculateEnergized();
            for(int i = 0; i < beams.Count; i++)
            {
                beams[i].Move();
                if(beams[i].isComplete)
                    continue;

                c = g.GetCell(beams[i]);
                c.isEnergized = true;
                ProcessCell(beams[i], c);
            }
            Console.WriteLine(g);

            Console.WriteLine(total);
        }

        Console.WriteLine(g.CalculateEnergized());
    }


    static void StarTwo()
    {
        

        Console.WriteLine(0);
    }
}
