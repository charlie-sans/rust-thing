import requests
import sys
import argparse

argparser = argparse.ArgumentParser(description='Get list of organisaions git repos and print the name plus the link to the repo. Then get the list of all the users that have contributed to the repo and print their names and a link to there github profile.')
argparser.add_argument('org_name', help='The name of the organisation')
args = argparser.parse_args()


# get the list of organisations repos
def get_org_repos(org_name):
    url = "https://api.github.com/orgs/" + org_name + "/repos"
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    else:
        print("Error: " + str(response.status_code))
        return None
    
# get the list of users that have contributed to the repo
def get_repo_contributors(repo_name):
    url = "https://api.github.com/repos/" + repo_name + "/contributors"
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    else:
        print("Error: " + str(response.status_code))
        return None
    
# print the list of organisations repos
def print_org_repos(org_name):
    repos = get_org_repos(org_name)
    if repos != None:
        for repo in repos:
            print(repo["name"] + " " + repo["html_url"])
            print_repo_contributors(repo["full_name"])
            print()

# print the list of users that have contributed to the repo
def print_repo_contributors(repo_name):
    contributors = get_repo_contributors(repo_name)
    if contributors != None:
        for contributor in contributors:
            print(contributor["login"] + " " + contributor["html_url"])


def main():
    if len(sys.argv) != 2:
        print("Usage: python3 gitgetter.py <org_name>")
        return
    
    org_name = sys.argv[1]
    print_org_repos(org_name)

if __name__ == "__main__":
    main()