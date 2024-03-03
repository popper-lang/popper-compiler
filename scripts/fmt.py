#!/usr/bin/env python3
import subprocess
import git
import fix

def cargo_fmt_lib(lib):
    print(f"[script/fmt.py] Running `cargo fmt` on lib {lib}")
    subprocess.run(["cargo", "fmt", "--lib", "-p", lib])
    git.commit(f"Run `cargo fmt` on lib {lib}")

def cargo_fmt_bin(bin):
    print(f"[script/fmt.py] Running `cargo fmt` on bin {bin}")
    subprocess.run(["cargo", "fmt", "--bin", bin])
    git.commit(f"Run `cargo fmt` on bin {bin}")

def main():
    crates = fix.list_all_crates()
    print(f"[script/fmt.py] Found crates: {', '.join(crate.name for crate in crates)}")
    input(f"[script/fmt.py] Press enter to continue to run `cargo fmt` on all crates({', '.join(crate.name for crate in crates)})")
    for crate in crates:
        cargo_fmt_lib(crate.name)

    cargo_fmt_bin(".")

if __name__ == "__main__":
    main()
