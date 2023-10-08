#!/usr/bin/env python3

import os
import sys
import json
import git
import re
import requests

if __name__ == "__main__":
    repo = git.Repo(os.getcwd(), search_parent_directories=True)
    os.chdir(repo.git.rev_parse("--show-toplevel"))

    origin = repo.remotes.origin.url[:-4]

    try:
        with open("open-source.fern", "r") as f:
            fern = json.load(f)
    except FileNotFoundError:
        technologies = list(set(map(lambda x: x.strip(), input("Input technologies separated by a comma (','):").split(","))))
        difficulty = -1
        try:
            difficulty = int(input("How difficult is your project to contribute to (1-5)? "))
            if not 0 < difficulty < 6:
                raise ValueError()
        except ValueError as e:
            raise ValueError("Error: difficulty should be a number from 1-5")
        
        desc = input("Enter a brief description of your project: ")

        recommended_issue_labels = list(set(map(lambda x: x.strip(), input("Input any tag/label that would be a good first issue separated by a comma (','):").split(","))))

        fern = {"name": re.search(r"/([^/]*/[^/]*)$",origin).group(1), "technologies": technologies, "difficulty": difficulty, "description": desc, "recommended_issue_labels": recommended_issue_labels}

    fern['repo_origin'] = origin
    fern['fern_branch'] = repo.active_branch.name

    print(fern)

    # Make api call
    r = requests.post("http://104.248.58.127/api/repos", json=fern) # We put the hack in hackathon
    if not 200 <= r.status_code < 300:
        print("Error registering project")
    else:
        with open("open-source.fern", "w+") as f:
            del fern['repo_origin']
            del fern['fern_branch']
            json.dump(fern, f)
