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

    public int CalculatePermutations(Setting setting)
    {
        int total = 0;
        
        int setIndex = settings.FindIndex(x => x == setting);
        Console.WriteLine(settings[setIndex]);
        if(settings[setIndex].value == -1)
        {

        }
        else
        {
            if(settings[setIndex].lessThan)
                total = settings[setIndex].value - 1;
            else
                total = 4000 - (settings[setIndex].value + 1);
        }
        Console.WriteLine("{1} {0} After Initial Value", total, name);
        
        for(int i = setIndex - 1; i > -1; i--)
        {
            Console.WriteLine("{0} Before Setting {1}", settings[i].value, settings[i].itemType);
            if(!settings[i].lessThan)
                total = total == 0 ? settings[i].value : total * settings[i].value;
            else
                total = total == 0 ? settings[i].value : total * (4000 - settings[i].value);
            
            Console.WriteLine("{0} After Setting {1}", total, settings[i].itemType);
        }

        return total;
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
    public ulong minX = 1;
    public ulong maxX = 4000;
    public ulong minM = 1;
    public ulong maxM = 4000;
    public ulong minA = 1;
    public ulong maxA = 4000;
    public ulong minS = 1;
    public ulong maxS = 4000;

    public ulong GetX()
    {
        return maxX - minX;
    }

    public ulong GetM()
    {
        return maxM - minM;
    }

    public ulong GetA()
    {
        return maxA - minA;
    }

    public ulong GetS()
    {
        return maxS - minS;
    }

    public ulong Product()
    {
        return GetX() * GetM() * GetA() * GetS();
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
        ulong total = 0;
        List<Range> ranges = new List<Range>();

        foreach(var workflow in workflows)
        {
            if(workflow.settings.Count(x => x.nextWorkflow == "A") > 0)
            {
                foreach(var setting in workflow.settings.Where(x => x.nextWorkflow == "A").Select(x => x))
                {
                    Range r = new Range();
                    ulong s = (ulong)workflow.CalculatePermutations(setting);
                    Workflow? nextWf = workflows.First(x => x.settings.Any(y => y.nextWorkflow == workflow.name));
                    string lastWfName = workflow.name;
                    
                    do
                    {
                        Console.WriteLine(s);
                        s *= (ulong)nextWf.CalculatePermutations(nextWf.settings.First(x => x.nextWorkflow == lastWfName));
                        lastWfName = nextWf.name;
                        nextWf = workflows.FirstOrDefault(x => x.settings.Any(y => y.nextWorkflow == lastWfName));
                    }while(nextWf != null);
                    Console.WriteLine("Workflow {0}", s.ToString("N0"));
                    ranges.Add(range);
                }
            }
        }

        Console.WriteLine(total.ToString("N0"));
    }
}