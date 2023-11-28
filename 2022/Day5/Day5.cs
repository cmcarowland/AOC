using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

/*
                [M]     [V]     [L]
[G]             [V] [C] [G]     [D]
[J]             [Q] [W] [Z] [C] [J]
[W]         [W] [G] [V] [D] [G] [C]
[R]     [G] [N] [B] [D] [C] [M] [W]
[F] [M] [H] [C] [S] [T] [N] [N] [N]
[T] [W] [N] [R] [F] [R] [B] [J] [P]
[Z] [G] [J] [J] [W] [S] [H] [S] [G]
 1   2   3   4   5   6   7   8   9 
*/

public class Program
{
    public static void Main()
    {
        var lines = File.ReadAllLines(@"2022\Day5\" + "Input.txt");
        List<Stack<char>> stacks = new List<Stack<char>>(){
            new Stack<char>(new char[]{'Z','T','F','R','W','J','G'}),
            new Stack<char>(new char[]{'G','W','M'}),
            new Stack<char>(new char[]{'J','N','H','G'}),
            new Stack<char>(new char[]{'J','R','C','N','W'}),
            new Stack<char>(new char[]{'W','F','S','B','G','Q','V','M'}),
            new Stack<char>(new char[]{'S','R','T','D','V','W','C'}),
            new Stack<char>(new char[]{'H','B','N','C','D','Z','G','V'}),
            new Stack<char>(new char[]{'S','J','N','M','G','C'}),
            new Stack<char>(new char[]{'G','P','N','W','C','J','D','L'})
        };

        // StarOne(lines, stacks);
        StarTwo(lines, stacks);
    }

    /*
    move 1 from 5 to 2
    move 7 from 7 to 1
    move 1 from 1 to 7
    */

    public static void PrintStacks(List<Stack<char>> stacks)
    {
        int i = 1;
        foreach(var stack in stacks)
        {
            Console.WriteLine("[{0}] : {1}", i, string.Join(",", stack.Reverse()));
            i++;
        }
    }

    public static int StarOne(string[] lines, List<Stack<char>> stacks)
    {
        foreach(var line in lines)
        {
            var sub = line.Substring(5);
            sub = sub.Replace("from ", "").Replace("to ", "");
            var splitNums = sub.Split(' ');
            int amount = int.Parse(splitNums[0]);
            int from = int.Parse(splitNums[1]) - 1;
            int to = int.Parse(splitNums[2]) - 1;

            while(amount > 0)
            {    
                var crate = stacks[from].Pop();
                stacks[to].Push(crate);
                amount --;
            }

            // PrintStacks(stacks);
        }

        var x = "";
        foreach(var stack in stacks)
            x += stack.Peek();
        
        Console.WriteLine(x);
        return -1;
    }

    public static int StarTwo(string[] lines, List<Stack<char>> stacks)
    {
        foreach(var line in lines)
        {
            var sub = line.Substring(5);
            sub = sub.Replace("from ", "").Replace("to ", "");
            var splitNums = sub.Split(' ');
            int amount = int.Parse(splitNums[0]);
            int from = int.Parse(splitNums[1]) - 1;
            int to = int.Parse(splitNums[2]) - 1;


            var crates = stacks[from].Take(amount).ToList();
            crates.Reverse();
            foreach(var crate in crates)
            {
                stacks[to].Push(crate);
                stacks[from].Pop();
            }
            
            // PrintStacks(stacks);
        }

        var x = "";
        foreach(var stack in stacks)
            x += stack.Peek();
        
        Console.WriteLine(x);
        return -1;
    }
}
