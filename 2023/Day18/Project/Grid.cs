using System.Text;
using System.Collections.Generic;
using System.Linq;
using System.Diagnostics;

class Color
{
    public int red;
    public int green;
    public int blue;

    public Color(string hex)
    {
        red = int.Parse(string.Join("", hex.Take(2)), System.Globalization.NumberStyles.HexNumber);
        green = int.Parse(string.Join("", hex.Skip(2).Take(2)), System.Globalization.NumberStyles.HexNumber);
        blue = int.Parse(string.Join("", hex.Skip(4).Take(2)), System.Globalization.NumberStyles.HexNumber);
    }

    public override string ToString()
    {
        return string.Format("{0}:{1}:{2}", red, green, blue);
    }
}

class Cell
{
    public Color? color;
    public bool isBorder;

    public Cell()
    {
        color = null;
    }
}

class Row
{
    public List<Cell> cells;

    public Row(int width)
    {
        cells = new List<Cell>();
        for(int i = 0; i < width; i++)
        {
            cells.Add(new Cell());
        }
    }
}

class Grid
{
    public Cell[,] cells;
    public int width;
    public int height;

    public Grid(int width, int height)
    {
        this.width = width;
        this.height = height;
        cells = new Cell[height, width];
        for(int i = 0; i < height * width; i++)
        {
            cells[i / width, i % width] = new Cell();
        }
    }

    public Cell GetCell(int row, int col)
    {
        return cells[row, col];
    }
    
    public Cell GetCell(int index)
    {
        return cells[index / width, index % width];
    }

    public int CountTrenches(int r, int c)
    {
        Cell cell = GetCell(r, c);
        if(cell.color == null)
            return 0;

        int count = 0;
        while(c < width && GetCell(r, c).color != null)
        {
            count ++;
            c++;
        }

        return count;
    }
    
    public bool IsTrechPresent(int r, int c)
    {
        for(int i = c; i < width; i ++)
        {
            if(GetCell(r, i).color != null)
                return true;
        }

        return false;
    }

    public void FillArea()
    {
        bool isDigging = false;
        bool isInside = false;

        for(int r = 0; r < height; r++)
        {
            isDigging = false;
            isInside = false;

            for(int c = 0; c < width; c++)
            {
                int numberOfTrenches = CountTrenches(r, c);
                if(!isDigging && GetCell(r, c).color == null && c + 1 < width && GetCell(r, c + 1).color != null)
                {
                    isDigging = true;
                    isInside = true;
                    c++;
                }
                else if(isDigging && GetCell(r, c).color == null)
                {
                    GetCell(r, c).color = new Color("000000");
                }
                else if(isDigging)
                {
                    if(isInside && numberOfTrenches == 1 && GetCell(r, c).isBorder)//End of a inside
                    {
                        isInside = false;
                        isDigging = false;
                    }
                    else if(isInside)
                    {
                        c += numberOfTrenches - 1;
                    }
                    else
                    {
                        c += numberOfTrenches - 1;
                        if(isDigging && GetCell(r, c).color != null && c + 1 < width && GetCell(r, c + 1).color == null)
                            isDigging = false;
                    }
                }
            }
        }
    }

    public int CountArea()
    {
        int total = 0;
        foreach(var cell in cells)
        {
            if(cell.color != null)
                total ++;
        }

        return total;
    }

    public override string ToString()
    {
        StringBuilder sb = new StringBuilder();
        for(int i = 0; i < height * width; i++)
        {
            sb.Append(cells[i / width, i % width].color == null ? "." : "#");
            if((i + 1) % width == 0)
                sb.AppendLine();
        }

        return sb.ToString();
    }
}