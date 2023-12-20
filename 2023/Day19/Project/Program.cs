using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.ComponentModel;

class Setting
{
    public char itemType;
    public bool lessThan;
    public int value;
    public string nextWorkflow;
    public Workflow parent;

    public Setting(string s, Workflow wf)
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

        parent = wf;
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
            settings.Add(new Setting(set, this));
    }

    public void CalculatePermutations(Setting setting, Range r)
    {        
        int setIndex = settings.FindIndex(x => x == setting);
        // Console.WriteLine(settings[setIndex]);
        if(settings[setIndex].value != -1)
        {
        
            // total = settings[setIndex].value - 1;
            if(settings[setIndex].itemType == 'x')
            {
                if(settings[setIndex].lessThan)
                    r.maxX = Math.Min(settings[setIndex].value - 1, r.maxX);
                else
                    r.minX = Math.Max(settings[setIndex].value + 1, r.minX);
            }
            else if(settings[setIndex].itemType == 'm')
            {
                if(settings[setIndex].lessThan)
                    r.maxM = Math.Min(settings[setIndex].value - 1, r.maxM);
                else
                    r.minM = Math.Max(settings[setIndex].value + 1, r.minM);
            }
            else if(settings[setIndex].itemType == 'a')
            {
                if(settings[setIndex].lessThan)
                    r.maxA = Math.Min(settings[setIndex].value - 1, r.maxA);
                else
                    r.minA = Math.Max(settings[setIndex].value + 1, r.minA);
            }
            else if(settings[setIndex].itemType == 's')
            {
                if(settings[setIndex].lessThan)
                    r.maxS = Math.Min(settings[setIndex].value - 1, r.maxS);
                else
                    r.minS = Math.Max(settings[setIndex].value + 1, r.minS);
            }
            // else
            // {
            //     // total = 4000 - (settings[setIndex].value + 1);
            // }
        }
        // Console.WriteLine("{1} {0} After Initial Value", total, name);
        
        for(int i = setIndex - 1; i > -1; i--)
        {
            if(settings[i].itemType == 'x')
            {
                if(!settings[i].lessThan)
                    r.maxX = Math.Min(settings[i].value, r.maxX);
                else
                    r.minX = Math.Max(settings[i].value, r.minX);
            }
            else if(settings[i].itemType == 'm')
            {
                if(!settings[i].lessThan)
                    r.maxM = Math.Min(settings[i].value, r.maxM);
                else
                    r.minM = Math.Max(settings[i].value, r.minM);
            }
            else if(settings[i].itemType == 'a')
            {
                if(!settings[i].lessThan)
                    r.maxA = Math.Min(settings[i].value, r.maxA);
                else
                    r.minA = Math.Max(settings[i].value, r.minA);
            }
            else if(settings[i].itemType == 's')
            {
                if(!settings[i].lessThan)
                    r.maxS = Math.Min(settings[i].value, r.maxS);
                else
                    r.minS = Math.Max(settings[i].value, r.minS);
            }
        }
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

        return "XXX";
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

    public Part(int x, int m, int a, int s)
    {
        this.x = x;
        this.m = m;
        this.a = a;
        this.s = s;
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

class Range
{
    public int minX = 1;
    public int maxX = 4000;
    public int minM = 1;
    public int maxM = 4000;
    public int minA = 1;
    public int maxA = 4000;
    public int minS = 1;
    public int maxS = 4000;

    public ulong GetX()
    {
        return (ulong)(maxX - minX + 1);
    }

    public ulong GetM()
    {
        return (ulong)(maxM - minM + 1);
    }

    public ulong GetA()
    {
        return (ulong)(maxA - minA + 1);
    }

    public ulong GetS()
    {
        return (ulong)(maxS - minS + 1);
    }

    public ulong Product()
    {
        // Console.WriteLine(GetX());
        // Console.WriteLine(GetM());
        // Console.WriteLine(GetA());
        // Console.WriteLine(GetS());
        return GetX() * GetM() * GetA() * GetS();
    }

    public override string ToString()
    {
        return string.Format("X: {0} {1}\nM: {2} {3}\nA: {4} {5}\nS: {6} {7}", minX, maxX, minM, maxM, minA, maxA, minS, maxS);
    }
}

class Program
{
    static string[] lines = new string[0];
    static public List<Workflow> workflows = new List<Workflow>();
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
       
        //StarOne();
        StarTwo();
    }

    static void StarOne()
    {
        int total = 0;
        
        foreach(var part in parts)
        {
            CheckPart(part);
        }

        foreach(var part in parts.Where(x => x.isAccepted).Select(x => x))
            total += part.Sum();

        Console.WriteLine(total);
    }


    static void StarTwo()
    {
        ulong total = 0;
        List<Range> ranges = new List<Range>();

        foreach(var workflow in workflows)
        {
            if(workflow.settings.Count(x => x.nextWorkflow == "A") > 0)
            {
                foreach(var setting in workflow.settings.Where(x => x.nextWorkflow == "A").Select(x => x))
                {
                    Range r = new Range();
                    workflow.CalculatePermutations(setting, r);
                    Workflow? nextWf = workflows.First(x => x.settings.Any(y => y.nextWorkflow == workflow.name));
                    string lastWfName = workflow.name;
                    
                    do
                    {
                        nextWf.CalculatePermutations(nextWf.settings.First(x => x.nextWorkflow == lastWfName), r);
                        lastWfName = nextWf.name;
                        nextWf = workflows.FirstOrDefault(x => x.settings.Any(y => y.nextWorkflow == lastWfName));
                    }while(nextWf != null);

                    ranges.Add(r);
                }
            }
        }

        foreach(var r in ranges)
        {
            total += r.Product();
        }

        Console.WriteLine(total.ToString("N0"));      
    }

    static public bool CheckPart(Part p)
    {
        var currentWorkflow = workflows.First(x => x.name == "in");
        string nextWorkflow = "";
        do
        {
            nextWorkflow = currentWorkflow.GetNextWorkflow(p);
            // Console.WriteLine(currentWorkflow.name);
            currentWorkflow = workflows.FirstOrDefault(x => x.name == nextWorkflow);
        }while(currentWorkflow != null);
        if(nextWorkflow == "A")
        {
            p.isAccepted = true;
            return true;
        }

        return false;
    }
}