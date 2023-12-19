using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;

class Program
{
    static string[] lines = new string[0];
    static public Grid g;

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        int lr = 0;
        int ud = 0;
        int maxLeft = 0;
        int maxRight = 0;
        int maxUp = 0;
        int maxDown = 0;
        foreach(var line in lines)
        {
            var split = line.Split(' ');
            switch(split[0])
            {
                case "R":
                    lr += int.Parse(split[1]);
                    maxRight = Math.Max(maxRight, lr);
                    break;
                case "D":
                    ud -= int.Parse(split[1]);
                    maxDown = Math.Min(maxDown, ud);
                    break;
                case "L":
                    lr -= int.Parse(split[1]);
                    maxLeft = Math.Min(maxLeft, lr);
                    break;
                case "U":
                    ud += int.Parse(split[1]);
                    maxUp = Math.Max(maxUp, ud);
                    break;
            }
        }
        // Console.WriteLine("Left :{0} Right :{1} Up :{2} Down:{3}\nWidth :{4}  Height :{5}", maxLeft, maxRight, maxUp, maxDown, Math.Abs(maxLeft) + maxRight, Math.Abs(maxDown) + maxUp);
        g = new Grid((Math.Abs(maxLeft) + maxRight + 1) * 2, (Math.Abs(maxDown) + maxUp + 1) * 2);

        StarOne();
        //StarTwo();
    }

    static void StarOne()
    {
        int total = 0;
        int currentRow = g.height / 2;
        int currentCol = g.width / 2;
        
        foreach(var line in lines)
        {
            var split = line.Split(' ');
            int i = int.Parse(split[1]);
            string hex = split[2].Replace("(", "").Replace("#", "").Replace(")", "");
            Cell cell = g.GetCell(currentRow, currentCol);

            switch(split[0])
            {
                case "R":
                    while(i > 0)
                    {
                        currentCol ++;
                        cell = g.GetCell(currentRow, currentCol);
                        cell.color = new Color(hex);
                        cell.isBorder = true;
                        i--;
                    }
                    break;
                case "D":
                    while(i > 0)
                    {
                        currentRow ++;
                        cell = g.GetCell(currentRow, currentCol);
                        cell.color = new Color(hex);
                        cell.isBorder = true;
                        i--;
                    }
                    break;
                case "L":
                    while(i > 0)
                    {
                        currentCol --;
                        cell = g.GetCell(currentRow, currentCol);
                        cell.color = new Color(hex);
                        cell.isBorder = true;
                        i--;
                    }
                    break;
                case "U":
                    while(i > 0)
                    {
                        currentRow --;
                        cell = g.GetCell(currentRow, currentCol);
                        cell.color = new Color(hex);
                        cell.isBorder = true;
                        i--;
                    }
                    break;
            }
            
            // Console.WriteLine("{0} {1}", currentRow, currentCol);
            g.GetCell(currentRow, currentCol).color = new Color(split[2].Replace("(", "").Replace("#", "").Replace(")", ""));
        }

        g.FillArea();
        StringBuilder sb = new StringBuilder();
        sb.AppendLine(g.ToString());
        File.WriteAllText("Out.txt", sb.ToString());

        foreach(var cell in g.cells)
        {
            if(cell.color != null)
                total ++;
        }

        Console.WriteLine(total);
    }


    static void StarTwo()
    {
        

        Console.WriteLine(0);
    }
}