#!/usr/bin/python3

import os
import re
import requests
import sys

from dotenv import load_dotenv
load_dotenv() 

WORKDIR = os.getenv("GRAPHQL_PATH") or "./graphql"
REGEX = os.getenv("GRAPHQL_FILE_REGEX") or "\w*.graphql"
URL = os.getenv("DGRAPH_DSN")

regex = re.compile(REGEX)

def main() -> int:
    print("Browsing for migration files...")

    scripts = []
    for root, _, files in os.walk(WORKDIR):
        files = filter(lambda filename: regex.match(filename), files)
        files = map(lambda filename: os.path.join(root, filename), files)
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

    target_url = "{}/admin/schema".format(URL)
    print("Applying migrations at {}".format(target_url))
    response = requests.post(target_url,
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
