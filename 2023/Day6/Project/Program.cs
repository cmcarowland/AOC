using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Collections.ObjectModel;
using System.Text.RegularExpressions;

public class Race
{
    public long raceTimeInMs;
    public long recordDistanceInMM;

    public Race(long rt, long rd)
    {
        raceTimeInMs = rt;
        recordDistanceInMM = rd;
    }

    public long RunRace(long hold)
    {
        long runTime = raceTimeInMs - hold;
        return runTime * hold;
    }

    public long CountRecordSettingRaces()
    {
        long records = 0;
        long minSpeedToSetRecord = (long)Math.Floor(recordDistanceInMM / (float)raceTimeInMs);
        for(long i = minSpeedToSetRecord; i < raceTimeInMs - minSpeedToSetRecord; i ++)
        {
            if(RunRace(i) > recordDistanceInMM)
                records ++;
        }

        return records;
    }

    public override string ToString()
    {
        return $"{raceTimeInMs} {recordDistanceInMM}";
    }
}

public class Program
{
    static string[] lines = new string[0];
    static List<Race> races = new List<Race>();

    static void ParseRaces(string t, string d)
    {
        var times = t.Split(',').Select(x => long.Parse(x)).ToArray();
        var recs = d.Split(',').Select(x => long.Parse(x)).ToArray();

        for(long i = 0; i < times.Length; i++)
            races.Add(new Race(times[i], recs[i]));
    }

    public static void Main()
    {
        lines = File.ReadAllLines(@"Input.txt");

        Console.WriteLine(StarOne());
        Console.WriteLine(StarTwo(lines));
    }

    static long TotalRecords(List<long> recs)
    {
        if(recs.Count == 0)
            return 0;
        
        long total = 1;
        foreach(var record in recs)
            total *= record;

        return total;
    }

    public static long StarOne()
    {
        var times = Regex.Replace(lines[0].Split(':')[1].Trim(), @"\s+", ",");
        var distanceRecords = Regex.Replace(lines[1].Split(':')[1].Trim(), @"\s+", ",");
        ParseRaces(times, distanceRecords);
        
        List<long> records = new List<long>();
        foreach(var race in races)
        {
            records.Add(race.CountRecordSettingRaces());
        }

        return TotalRecords(records);
    }

    public static long StarTwo(string[] lines)
    {
        var times = lines[0].Split(':')[1].Trim().Replace(" " , "");
        var distanceRecords = lines[1].Split(':')[1].Trim().Replace(" ", "");
        Race race = new Race(long.Parse(times), long.Parse(distanceRecords));
        return race.CountRecordSettingRaces();
    }
}
