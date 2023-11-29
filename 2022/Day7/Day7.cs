using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;

public class Directory
{
    public static int tabLevel = 0;
    public string name;
    public int size
    {
        get {
            return directories.Sum(x => x.size) + files.Sum(x => x.size);
        }
    }
    public Directory parent;
    public List<Directory> directories;
    public List<FileObject> files;

    public Directory(string n, Directory p = null)
    {
        name = n;
        directories = new List<Directory>();
        files = new List<FileObject>();
        parent = p;
    }

    public override string ToString()
    {
        StringBuilder sb = new StringBuilder();
        sb.AppendLine(string.Format("{0, -5} {1, -10} {2, -5}", "DIR", name, size.ToString("N0")));
        // foreach(var f in files)
        // {
        //     f.tabLevel = tabLevel;
        //     sb.AppendLine(f.ToString());
        // }

        // foreach(var dir in directories)
        // {
        //     tabLevel ++;
        //     sb.AppendLine(dir.ToString());
        // }
        

        tabLevel--;
        return sb.ToString();
    }
}

public class FileObject
{
    public string name;
    public int size;
    public int tabLevel = 0;

    public FileObject(string n, int s)
    {
        name = n;
        size = s;
    }

    public override string ToString()
    {
        return string.Format("  {0, -10} {1, -10} {2, -5}", "RWXR--R--", name, size);
    }
}

public class Program
{
    static public Directory root;
    static public Directory currentDirectory;

    public static void Main()
    {
        var lines = File.ReadAllLines(@"2022\Day7\" + "Input.txt");
        Console.WriteLine(StarOne(lines));
        Console.WriteLine(StarTwo(lines).ToString("N0"));
    }

    public static void ProcessCommand(string[] commandString)
    {
        if(commandString[1] == "cd")
        {
            if(commandString[2] == "/")
            {
                root = new Directory(commandString[2]);
                currentDirectory = root;
            }
            else if(commandString[2] == "..")
            {
                currentDirectory = currentDirectory.parent;
            }
            else
            {
                currentDirectory = currentDirectory.directories.First(x => x.name == commandString[2]);
            }
        }
    }
    
    public static void ProcessResult(string resultString)
    {
        var splitLine = resultString.Split(' ');
        int size = 0;
        if(int.TryParse(splitLine[0], out size))
        {
            currentDirectory.files.Add(new FileObject(splitLine[1], size));
        }
        else
        {
            currentDirectory.directories.Add(new Directory(splitLine[1], currentDirectory));
        }
    }

    public static int SumDirectories(Directory dir)
    {
        int result = dir.size <= 100000 ? dir.size : 0;
        foreach(var d in dir.directories)
        {
            result += SumDirectories(d);
        }

        return result;
    }

    public static int StarOne(string[] lines)
    {
        foreach(var line in lines)
        {
            if(line.StartsWith("$"))
            {
                ProcessCommand(line.Split(' '));
            }
            else
            {
                ProcessResult(line);
            }
        }

        return SumDirectories(root);
    }

    public static List<Directory> GetDirectories(Directory dir)
    {
        List<Directory> dirs = new List<Directory>();
        dirs.AddRange(dir.directories);

        foreach(var d in dir.directories)
            dirs.AddRange(GetDirectories(d));

        return dirs;
    }

    public static int StarTwo(string[] lines)
    {
        int hdCapacity = 70000000;
        int neededSize = 30000000;
        int diff = neededSize - (hdCapacity - root.size);
        List<Directory> allDirectories = GetDirectories(root).OrderBy(x => x.size).ToList();
        // foreach(var d in allDirectories)
        //     Console.Write(d);

        Console.WriteLine("{0}", diff.ToString("N0"));
        
        return allDirectories.First(x => x.size >= diff).size;
    }
}
