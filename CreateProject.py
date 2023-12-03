import requests
import os
import subprocess

year = 2023
day = 3
url =  '/'.join(['https://adventofcode.com',str(year),'day',str(day),'input'])
with open('Secret.txt', 'r') as iFile:
    cookie = iFile.readline()

cookies = {'session':cookie}
data = requests.get(url, cookies=cookies)       

fileName = '/'.join([str(year), ('Day' + str(day)),'Input.txt'])
os.makedirs(os.path.dirname(fileName), exist_ok=True)

with open(fileName, 'w') as oFile:
    oFile.write(data._content.decode())

projectFileName = '/'.join([str(year), ('Day' + str(day)),'Project'])
print(projectFileName)
subprocess.run(['dotnet', 'new', 'console', '-o', projectFileName])
