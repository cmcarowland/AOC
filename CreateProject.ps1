param($folderName, $path)
if (Test-Path -Path $path"\"$folderName)
{
    Write-Error("Project Already Exists")
    Exit
}

New-Item -Path $path -Name $folderName -ItemType Directory

$filePath = Join-Path -Path $path -ChildPath $folderName"\"$folderName".cs"
Write-Output $filePath
New-Item $filePath -ItemType File
Add-Content $filePath @"
using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

public class Program
{
    public static void Main()
    {
        var lines = File.ReadAllLines(@"$path\$folderName\" + "Input.txt");
    }

    public static int StarOne(string[] lines)
    {
        return -1;
    }

    public static int StarTwo(string[] lines)
    {
        return -1;
    }
}
"@


$filePath = Join-Path -Path $path -ChildPath $folderName"\Input.txt"
New-Item $filePath -ItemType File