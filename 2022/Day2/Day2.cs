using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Program
{
    //Rock     1
    //Paper    2
    //Scissors 3
    //win      6
    //Tie      3
    //Loss     0

    //X Rock Y Paper Z Scissors
    static Dictionary<string, int> scores = new Dictionary<string, int>(){{"X" , 1}, {"Y" , 2}, {"Z", 3}};
    
    public static void Main()
    {
        int score = 0;
        var lines = File.ReadAllLines(@"2022\Day2\Day2Input.txt");
        foreach(var line in lines)
        {
            //score += RoundOne(line);
            score += RoundTwo(line);
            //System.Threading.Thread.Sleep(200);
        }

        Console.WriteLine(score);
    }

    public static int Select(string theirValue, string winLoseOrDraw)
    {
        if(theirValue == "A")
        {
            if(winLoseOrDraw == "X")
                return scores["Z"];
            else if(winLoseOrDraw == "Y")
                return scores["X"];
            else
                return scores["Y"];       
        }
        if(theirValue == "B")
        {
            if(winLoseOrDraw == "X")
                return scores["X"];
            else if(winLoseOrDraw == "Y")
                return scores["Y"];
            else
                return scores["Z"];
        }
        if(theirValue == "C")
        {
            if(winLoseOrDraw == "X")
                return scores["Y"];
            else if(winLoseOrDraw == "Y")
                return scores["Z"];
            else
                return scores["X"];
        }

        throw new NullReferenceException();
    }

    public static int RoundTwo(string line)
    {
        //Console.WriteLine(line);
        var splitLine = line.Split(' ');
        int score = 0;
        if(splitLine[1] == "X")//Lose
        {
            score += Select(splitLine[0], splitLine[1]);
        }
        if(splitLine[1] == "Y")//Tie
        {
            score += 3;
            score += Select(splitLine[0], splitLine[1]);
        }
        if(splitLine[1] == "Z")//Win
        { 
            score += 6;
            score += Select(splitLine[0], splitLine[1]);
        }

        //Console.WriteLine(score);
        return score;
    }

    public static int RoundOne(string line)
    {
        var splitLine = line.Split(' ');
        int score = scores[splitLine[1]];

        if(splitLine[0] == "A")
        {
            if(splitLine[1] == "Y")
                score += 6;
            else if(splitLine[1] == "X")
                score += 3;
        }
        else if(splitLine[0] == "B")
        {
            if(splitLine[1] == "Z")
                score += 6;
            else if(splitLine[1] == "Y")
                score += 3;
        }
        else if(splitLine[0] == "C")
        {
            if(splitLine[1] == "X")
                score += 6;
            else if(splitLine[1] == "Z")
                score += 3;
        }

        return score;
    }
}
