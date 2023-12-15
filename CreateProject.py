import requests
import os
import sys
import subprocess

year = int(sys.argv[1])
day = int(sys.argv[2])
url =  '/'.join(['https://adventofcode.com',str(year),'day',str(day),'input'])
# with open('Secret.txt', 'r') as iFile:
#     cookie = iFile.readline()

# cookies = {'session':cookie}
# data = requests.get(url, cookies=cookies)       



projectFileName = '/'.join([str(year), ('Day' + str(day)), 'Project'])
print(projectFileName)
subprocess.run(['dotnet', 'new', 'console', '-o', projectFileName])

# fileName = '/'.join([str(year), ('Day' + str(day)), 'Project', 'Input.txt'])
# with open(fileName, 'w') as oFile:
#     oFile.write(data._content.decode())
