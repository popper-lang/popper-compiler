#!/usr/bin/env python3
import subprocess
import git
import fix

def cargo_fmt_lib(lib):
    print(f"[script/fmt.py] Running `cargo fmt` on lib {lib}")
    subprocess.run(["cargo", "fmt", "-p", lib])
    git.commit(f"Run `cargo fmt` on lib {lib}")


def main():
    crates = fix.list_all_crates()
    print(f"[script/fmt.py] Found crates: {', '.join(crate.name for crate in crates)}")
    input(f"[script/fmt.py] Press enter to continue to run `cargo fmt` on all crates...")
    for crate in crates:
        cargo_fmt_lib(crate.name)


if __name__ == "__main__":
    main()
