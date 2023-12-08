using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public enum HandType : byte
{
    None,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

public class Hand
{
    Dictionary<char, int> cardMapping = new Dictionary<char, int>()
    {
        {'2', 0},
        {'3', 1},
        {'4', 2},
        {'5', 3},
        {'6', 4},
        {'7', 5},
        {'8', 6},
        {'9', 7},
        {'T', 8},
        {'J', 9},
        {'Q', 10},
        {'K', 11},
        {'A', 12}
    };

    public HandType hand;
    public long value;
    public long bid;
    int[] counts = new int[15];

    public Hand(string s, long b)
    {
        foreach(var c in s)
        {
            counts[cardMapping[c]] ++;
        }

        hand = GetHandType();

        s = s.Replace('A', 'E').Replace('K', 'D').Replace('Q', 'C').Replace('J', 'B').Replace('T', 'A');
        value = long.Parse(s, System.Globalization.NumberStyles.HexNumber);
        bid = b;
    }

    HandType GetHandType()
    {
        if(counts.Contains(5))
            return HandType.FiveOfAKind;
        if(counts.Contains(4))
            return HandType.FourOfAKind;
        if(counts.Contains(3))
        {
            if(counts.Contains(2))
                return HandType.FullHouse;
            else
                return HandType.ThreeOfAKind;
        }
        if(counts.Count(x => x == 2) == 2)
            return HandType.TwoPair;
        if(counts.Contains(2))
            return HandType.Pair;

        return HandType.None;
    }

    public override string ToString()
    {
        return $"Type: {hand} Num Val: {value} Cards: {string.Join(",",counts)}";
    }
}

public class Program
{
    static List<Hand> hands = new List<Hand>();

    static void Main(string[] argv)
    {
        var lines = File.ReadAllLines(argv[0]);
        foreach(var line in lines)
        {
            var split = line.Split(' ');
            hands.Add(new Hand(split[0], long.Parse(split[1])));
        }

        StarOne();
        // StarTwo();
    }

    static void StarOne()
    {
        hands = hands.OrderBy(x => x.hand).ThenBy(x => x.value).ToList();
        long score = 0;
        for(int i = 0; i < hands.Count; i++)
        {
            score += hands[i].bid * (i + 1);
        }

        Console.WriteLine(score);
    }
    
    static void StarTwo()
    {
        
            
        Console.WriteLine("NA");
    }
}
