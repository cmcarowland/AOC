param($folderName, $path)

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
        
    }
}
"@