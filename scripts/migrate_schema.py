#!/usr/bin/python3

import os
import re
import requests

from dotenv import load_dotenv
load_dotenv() 

WORKDIR = os.getenv("GRAPHQL_PATH")
REGEX = os.getenv("GRAPHQL_FILE_REGEX")
URL = os.getenv("DGRAPH_ADMIN_URL")

regex = re.compile(REGEX)

scripts = []
for root, dirs, files in os.walk(WORKDIR):
    for file in files:
        if regex.match(file):
            path = os.path.join(root, file)
            scripts.append(path)

query = ""
for path in sorted(scripts):
    print("Reading content from {}".format(path))

    fo = open(path, "r")
    query += "{}\n".format(fo.read())
    fo.close()

print("Query:\n{}".format(query))
response = requests.post(URL, data=query, headers={
    "Content-Type": "application/json"
})

print("Migration response: {}".format(response))