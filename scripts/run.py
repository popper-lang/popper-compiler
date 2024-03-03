#!/usr/bin/env python3
import subprocess
import sys

CODEGEN_MODE = "inkwell"

def run_example(example):
    print(f"[script/run.py] Running example {example} with codegen mode `{CODEGEN_MODE}`")
    if CODEGEN_MODE == "llvm":
        subprocess.run(["cargo", "run", "run", f"./examples/{example}.pop"])
    elif CODEGEN_MODE == "inkwell":
        subprocess.run(["cargo", "run", "run", f"./examples/{example}.pop", "-i"])

def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <example>")
        sys.exit(1)

    run_example(sys.argv[1])

if __name__ == "__main__":
    main()
