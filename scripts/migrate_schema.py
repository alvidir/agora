#!/usr/bin/python3

import os
import re
import requests

from dotenv import load_dotenv
load_dotenv() 

WORKDIR = os.getenv("GRAPHQL_PATH")
REGEX = os.getenv("GRAPHQL_FILE_REGEX")
URL = os.getenv("DGRAPH_SCHEMA_URL")

regex = re.compile(REGEX)

print("Browsing for migration files...")

scripts = []
for root, dirs, files in os.walk(WORKDIR):
    for file in files:
        if regex.match(file):
            path = os.path.join(root, file)
            scripts.append(path)

if not scripts:
    print("No migration files has been found")
    exit()

query = ""
for path in sorted(scripts):
    print("-\t{}".format(path))

    fo = open(path, "r")
    query += "{}\n".format(fo.read())
    fo.close()

print("Applying migrations at {}".format(URL))
response = requests.post(URL, data=query, headers={
    "Content-Type": "application/json"
})

data = response.json()
if 'errors' in data:
    print("Response: {}".format(data['errors']))
    print("FAILED")
else:
    print("SUCCESS")
