using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;

interface Module
{
    public string name
    {
        get; set;
    }
    public void ReceivePulse(Pulse pulse) {}
    public void Process() {}
}

class FlipFlop : Module
{
    bool isOn = false;

    public string name
    {
        get; set;
    }
    public List<Module> connectedModules;
    public Queue<Pulse> pulses;

    public FlipFlop(string n)
    {
        name = n;
        connectedModules = new List<Module>();
        pulses = new Queue<Pulse>();
    }

    public void ReceivePulse(Pulse pulse)
    {
        // Console.WriteLine($"{name} -{pulse.IsLow()}-> {pulse.sentBy.name}");
        pulses.Enqueue(pulse);
        if(!Program.itemsToProcess.Contains(this))
            Program.itemsToProcess.Enqueue(this);
    }

    public void Process()
    {
        if(pulses.Count == 0)
            return;
        // Console.WriteLine("{0} {1}", name, pulses.Count);
        while(pulses.Count > 0)
        {
            Pulse p = pulses.Dequeue();
            // Console.WriteLine("{0} {1}", name, p.isLow);
            if(p.isLow)
            {
                isOn = !isOn;
                foreach(var module in connectedModules)
                {
                    // Console.WriteLine($"{name} -{IsLow(isOn)}-> {module.name}");
                    module.ReceivePulse(new Pulse(!isOn, this));
                }
            }
        }
    }

    string IsLow(bool state)
    {
        if(state)
            return "high";
        else
            return "low";
    }
}


class Conjunction : Module
{
    public string name
    {
        get; set;
    }
    public List<Module> connectedModules;
    public Dictionary<Module, bool> inputPulses;
    public Queue<Pulse> pulses;

    public Conjunction(string n)
    {
        name = n;
        connectedModules = new List<Module>();
        inputPulses = new Dictionary<Module, bool>();
        pulses = new Queue<Pulse>();
    }

    public void ReceivePulse(Pulse pulse)
    {
        pulses.Enqueue(pulse);
        if(!Program.itemsToProcess.Contains(this))
            Program.itemsToProcess.Enqueue(this);
    }

    public void Process()
    {
        if(pulses.Count == 0)
            return;

        // Console.WriteLine("{0} Pulses {1}", name, pulses.Count);
        while(pulses.Count > 0)
        {
            Pulse pulse = pulses.Dequeue();
            // Console.WriteLine("Is Low {0}", pulse.isLow);
            inputPulses[pulse.sentBy] = pulse.isLow;
            // Console.WriteLine(string.Join(", ", inputPulses));
            // Console.WriteLine(inputPulses.Count);
            // Console.WriteLine(inputPulses.Count(x => x.Value == false));
            if(inputPulses.Count(x => x.Value == false) == inputPulses.Count)
            {
                foreach(var module in connectedModules)
                {
                    // Console.WriteLine($"{name} -{IsLow(false)}-> {module.name}");
                    module.ReceivePulse(new Pulse(true, this));
                }
            }
            else
            {
                foreach(var module in connectedModules)
                {
                    // Console.WriteLine($"{name} -{IsLow(true)}-> {module.name}");
                    module.ReceivePulse(new Pulse(false, this));
                }
            }
        }
    }

    string IsLow(bool state)
    {
        if(state)
            return "high";
        else
            return "low";
    }
}

class Broadcast : Module
{
    public string name
    {
        get; set;
    }
    public List<Module> connectedModules;

    public Broadcast()
    {
        name = "broadcaster";
        connectedModules = new List<Module>();
    }

    public void ReceivePulse(Pulse pulse)
    {
        foreach(var module in connectedModules)
        {  
            // Console.WriteLine($"{name} -{pulse.IsLow()}-> {module.name}");
            module.ReceivePulse(new Pulse(pulse.isLow, this));
        }
    }
}

class Button : Module
{
    public string name
    {
        get; set;
    }

    public Broadcast? broadcast = null;

    public Button()
    {
        name = "Button";
    }

    public void ReceivePulse(Pulse pulse)
    {}

    public void PressButton()
    {
        // Console.WriteLine($"Button -low-> broadcaster");
        broadcast.ReceivePulse(new Pulse(true, this));
    }
}

class Undefined : Module
{
    public string name
    {
        get; set;
    }

    public Undefined(string n)
    {
        name = n;
    }

    public void ReceivePulse(Pulse pulse)
    {
        // Console.WriteLine("{0} {1}", name, pulse.IsLow());
        if(name == "rx" && pulse.isLow)
        {
            Console.WriteLine("******************  Low: {0} High: {1}", Pulse.lowPulses, Pulse.highPulses);
            Program.isComplete = true;
        }
    }

    public void Process()
    {

    }
}

class Pulse
{
    public bool isLow = true;
    public Module sentBy;

    static public int lowPulses = 0;
    static public int highPulses = 0;

    public Pulse(bool low, Module m)
    {
        isLow = low;
        sentBy = m;
        if(isLow)
            lowPulses ++;
        else
            highPulses ++;
        // Console.WriteLine($"+{IsLow()} : Pulse Called {lowPulses} {highPulses}");
    }

    public string IsLow()
    {
        if(isLow)
            return "low";
        else
            return "high";
    }
}

class Program
{
    static string[] lines = new string[0];
    static public Queue<Module> itemsToProcess = new Queue<Module>();
    static public bool isComplete = false;

    static void Main(string[] argv)
    {
        lines = File.ReadAllLines(argv[0]);
        
        StarOne();
        //StarTwo();
    }

    static void StarOne()
    {
        Button b = new Button();
        List<Module> modules = new List<Module>();
 
        foreach(var line in lines)
        {
            if(line.StartsWith('%'))
            {
                var splitLine = line.Split(" -> ");
                modules.Add(new FlipFlop(splitLine[0].Replace("%", "")));
            }
            else if(line.StartsWith('&'))
            {
                var splitLine = line.Split(" -> ");
                modules.Add(new Conjunction(splitLine[0].Replace("&", "")));
            }
            else
            {
                b.broadcast = new Broadcast();
            }
        }

        foreach(var line in lines)
        {
            var splitLine = line.Split(" -> ");
            if(line.StartsWith('%'))
            {
                FlipFlop ff = (FlipFlop)modules.First(x => x.name == splitLine[0].Replace("%", ""));
                var connections = splitLine[1].Split(',');
                foreach(var c in connections)
                {
                    try
                    {
                        ff.connectedModules.Add(modules.First(x => x.name == c.Trim()));
                    }
                    catch 
                    {
                        ff.connectedModules.Add(new Undefined(c.Trim()));
                    }
                }
            }
            else if(line.StartsWith('&'))
            {
                Conjunction conjunction = (Conjunction)modules.First(x => x.name == splitLine[0].Replace("&", ""));
                var connections = splitLine[1].Split(',');
                foreach(var c in connections)
                {
                    // Console.WriteLine(c.Trim());
                    try
                    {
                        conjunction.connectedModules.Add(modules.First(x => x.name == c.Trim()));
                    }
                    catch 
                    {
                        conjunction.connectedModules.Add(new Undefined(c.Trim()));
                    }
                }
            }
            else
            {
                var connections = splitLine[1].Split(',');
                foreach(var c in connections)
                {
                    b.broadcast.connectedModules.Add(modules.First(x => x.name == c.Trim()));
                }
            }
        }

        foreach(var module in modules)
        {
            if(module is FlipFlop)
            {
                FlipFlop ff = (FlipFlop)module;
                foreach(var con in ff.connectedModules)
                {
                    if(con is Conjunction)
                    {
                        ((Conjunction)con).inputPulses[module] = true;
                    }
                }
            }
        }

        // Console.Write("{0} ", b.broadcast.name);

        // foreach(var mod in b.broadcast.connectedModules)
        //     Console.Write("{0} ", mod.name);

        // Console.WriteLine();
        // foreach(var m in modules)
        // {
        //     if(m is FlipFlop)
        //     {
        //         FlipFlop ff = (FlipFlop)m;
        //         Console.Write("{0} ", ff.name);

        //         foreach(var mod in ff.connectedModules)
        //             Console.Write("{0} ", mod.name);

        //         Console.WriteLine();
        //     }
        //     else if(m is Conjunction)
        //     {
        //         Conjunction ff = (Conjunction)m;
        //         Console.Write("{0} ", ff.name);

        //         foreach(var mod in ff.connectedModules)
        //             Console.Write("{0} ", mod.name);

        //         Console.WriteLine();
        //     }
        // }

        int presses = 0;
        for(int i = 0; i < 1000000000 && !isComplete; i++)
        {
            b.PressButton();
            presses ++;
            while(itemsToProcess.Count > 0 && !isComplete)
            {
                // foreach(var it in itemsToProcess)
                //     Console.WriteLine(it.name);
                itemsToProcess.Dequeue().Process();
            }
            // Console.WriteLine();
        }

        Console.WriteLine("Presses: {0}", presses);
        Console.WriteLine("Low: {0} High: {1}", Pulse.lowPulses, Pulse.highPulses);
        Console.WriteLine("{0}", Pulse.lowPulses * Pulse.highPulses);
    }

    static void StarTwo()
    {

    }
}