#!/usr/bin/python3

import os
import re
import requests
import sys

from dotenv import load_dotenv
load_dotenv()

WORKDIR = os.getenv("GRAPHQL_PATH") or "/etc/graphql"
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
        print(f"-\t{path}")

        fo = open(path, "r")
        query += f"{fo.read()}\n"
        fo.close()

    target_url = f"{URL}/admin/schema"
    print(f"Applying migrations at {target_url}")

    try:
        response = requests.post(target_url,
            data=query.encode(encoding='utf-8'),
            headers={
                "Content-Type": "application/json"
            }
        )
    except Exception as e:
        print(f"Cannot perform POST request: {e}")
        return 1


    data = response.json()
    if 'errors' in data:
        print(f"Response: {data['errors']}")
        print("FAILED")
        return 1
    
    print("SUCCESS")
    return 0

if __name__ == '__main__':
    sys.exit(main())
