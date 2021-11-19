#!/usr/bin/python3

import os
import re
import requests
import sys

from dotenv import load_dotenv
load_dotenv() 

WORKDIR = os.getenv("GRAPHQL_PATH")
REGEX = os.getenv("GRAPHQL_FILE_REGEX")
URL = os.getenv("DGRAPH_ALTER_URL")

regex = re.compile(REGEX)

def is_migration_files(filename) -> bool:
    return regex.match(filename)

def main() -> int:
    print("Browsing for migration files...")

    scripts = []
    for root, _, files in os.walk(WORKDIR):
        files = filter(is_migration_files, files)
        
        def make_absolute_path(filename) -> str:
            return os.path.join(root, filename)

        files = map(make_absolute_path, files)
        scripts += list(files)

    if not scripts:
        print("No migration files where found")
        return 1

    query = ""
    for path in sorted(scripts):
        print("-\t{}".format(path))

        fo = open(path, "r")
        query += "{}\n".format(fo.read())
        fo.close()

    print("Applying migrations at {}".format(URL))
    response = requests.post(URL,
        data=query.encode(encoding='utf-8'),
        headers={
            "Content-Type": "application/json"
        }
    )

    data = response.json()
    if 'errors' in data:
        print("Response: {}".format(data['errors']))
        print("FAILED")
        return 1
    
    print("SUCCESS")
    return 0

if __name__ == '__main__':
    sys.exit(main())
