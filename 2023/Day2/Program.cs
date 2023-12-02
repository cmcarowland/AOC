using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Collections.ObjectModel;

public class Game
{
    public int id;
    int maxRed;
    int maxBlue;
    int maxGreen;

    int redCubes = 12;
    int greenCubes = 13;
    int blueCubes = 14;

    public Game(string line)
    {
        var splitLine = line.Split(':');
        id = int.Parse(splitLine[0].Substring(5));
        splitLine = splitLine[1].Split(';');
        foreach(var play in splitLine)
        {
            var playSplit = play.Split(',');
            foreach(var color in playSplit)
            {
                int parsedCube = int.Parse(color.Trim().Split(' ')[0]);
                if(color.Contains("red"))
                {
                    maxRed = parsedCube > maxRed ? parsedCube : maxRed;
                }
                else if(color.Contains("green"))
                {
                    maxGreen = parsedCube > maxGreen ? parsedCube : maxGreen;
                }
                else if(color.Contains("blue"))
                {
                    maxBlue = parsedCube > maxBlue ? parsedCube : maxBlue; 
                }
            }
        }
    }

    public bool IsImpossible()
    {
        if(maxRed > redCubes || maxGreen > greenCubes || maxBlue > blueCubes)
            return true;

        return false;
    }

    public int Power()
    {
        return maxBlue * maxGreen * maxRed;
    }

    public override string ToString()
    {
        return $"Red: {maxRed} Green: {maxGreen} Blue:{maxBlue} Impossible?: {IsImpossible()} Power: {Power()}";
    }
}

public class Program
{
    static string[] lines = new string[0];

    public static void Main()
    {
        lines = File.ReadAllLines(@"Input.txt");
        Console.WriteLine(StarOne());
        Console.WriteLine(StarTwo());
    }

    public static int StarOne()
    {
        List<Game> possibleGames = new List<Game>();

        foreach(var line in lines)
        {
            Game g = new Game(line);
            
            if(!g.IsImpossible())
                possibleGames.Add(g);
        }

        return possibleGames.Sum(x => x.id);
    }

    public static int StarTwo()
    {
        List<Game> games = new List<Game>();

        foreach(var line in lines)
        {
            Game g = new Game(line);
            games.Add(g);
        }

        return games.Sum(x => x.Power());
    }
}
