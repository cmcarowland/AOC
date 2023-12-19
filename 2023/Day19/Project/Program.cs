using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;

class Setting
{
    public char itemType;
    public bool lessThan;
    public int value;
    public string nextWorkflow;

    public Setting(string s)
    {
        if(s.Contains(':'))
        {
            var splitS = s.Split(":");
            nextWorkflow = splitS[1];
            itemType = splitS[0][0];
            if(splitS[0][1] == '<')
                lessThan = true;
            else
                lessThan = false;
            
            value = int.Parse(string.Join("", splitS[0].Skip(2)));
        }
        else
        {
            itemType = ' ';
            value = -1;
            nextWorkflow = s;
        }
    }

    public bool CheckValue(int itemValue)
    {
        if(lessThan)
        {
            return itemValue < value;
        }
        else
        {
            return itemValue > value;
        }
    }

    public override string ToString()
    {
        return string.Format("Type: {0} LessThan: {1} Value: {2} Next: {3}", itemType, lessThan, value, nextWorkflow);
    }
}

class Workflow
{
    public string name;
    public List<Setting> settings = new List<Setting>();

    public Workflow(string s)
    {
        var split = s.Split("{");
        var settingString = split[1].Replace("}", "");

        name = split[0];
        var splitSettngs = settingString.Split(',');
        foreach(var set in splitSettngs)
            settings.Add(new Setting(set));
    }

    public string GetNextWorkflow(Part value)
    {
        foreach(var set in settings)
        {
            if(set.value == -1)
            {
                return set.nextWorkflow;
            }

            int checkValue = 0;
            switch(set.itemType)
            {
                case 'x':
                    checkValue = value.x;
                    break;
                case 'm':
                    checkValue = value.m;
                    break;
                case 'a':
                    checkValue = value.a;
                    break;
                case 's':
                    checkValue = value.s;
                    break;
            }

            if(set.CheckValue(checkValue))
            {
                return set.nextWorkflow;
            }
        }

        return "FUCKING FAILED";
    }
}

class Part
{
    public int x;
    public int m;
    public int a;
    public int s;
    public bool isAccepted;

    public Part(string str)
    {
        var split = str.Split(',');
        foreach(var rating in split)
        {
            var typeAndRating = rating.Split('=');
            var val = int.Parse(typeAndRating[1].Replace("}", ""));
            switch(typeAndRating[0].Replace("{", ""))
            {
                case "x":
                    x = val;
                    break;
                case "m":
                    m = val;
                    break;
                case "a":
                    a = val;
                    break;
                case "s":
                    s = val;
                    break;
            }
        }
    }

    public override string ToString()
    {
        return string.Format("{0} {1} {2} {3}", x, m, a, s);
    }

    public int Sum()
    {
        return x + m + a + s;
    }
}

class Program
{
    static string[] lines = new string[0];
    static List<Workflow> workflows = new List<Workflow>();
    static List<Part> parts = new List<Part>();

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        bool inParts = false;
        foreach(var line in lines)
        {
            if(string.IsNullOrEmpty(line))
            {
                inParts = true;
                continue;
            }

            if(inParts)
                parts.Add(new Part(line));
            else
                workflows.Add(new Workflow(line));
        }
       
        StarOne();
        //StarTwo();
    }

    static void StarOne()
    {
        int total = 0;
        
        foreach(var part in parts)
        {
            var currentWorkflow = workflows.First(x => x.name == "in");
            string nextWorkflow = "";
            do
            {
                nextWorkflow = currentWorkflow.GetNextWorkflow(part);
                currentWorkflow = workflows.FirstOrDefault(x => x.name == nextWorkflow);
            }while(currentWorkflow != null);
            if(nextWorkflow == "A")
            {
                total += part.Sum();
                part.isAccepted = true;
            }
        }

        Console.WriteLine(total);
    }


    static void StarTwo()
    {
        

        Console.WriteLine(0);
    }
}