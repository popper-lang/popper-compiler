import subprocess
import sys

def commit(msg):
    print(f"[script/git.py] Committing with message: {msg}")
    subprocess.run(["git", "add", "."])
    subprocess.run(["git", "commit", "-m", msg])

def push():
    print(f"[script/git.py] Pushing to remote")
    subprocess.run(["git", "push"])

def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <message>")
        sys.exit(1)

    commit(sys.argv[1])
    push()
