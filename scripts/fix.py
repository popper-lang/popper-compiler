import subprocess
import pathlib
import git


def cargo_fix_lib(lib):
    print(f"[script/fix.py] Running `cargo fix` on lib {lib}")
    subprocess.run(["cargo", "fix", "--lib", "-p", lib])
    git.commit(f"Run `cargo fix` on lib {lib}")

def cargo_fix_bin(bin):
    print(f"[script/fix.py] Running `cargo fix` on bin {bin}")
    subprocess.run(["cargo", "fix", "--bin", bin])

    git.commit(f"Run `cargo fix` on bin {bin}")

def list_all_crates():
    crates_dir = pathlib.Path("crates")
    crates = [crate for crate in crates_dir.iterdir() if crate.is_dir()]
    return crates

def main():
    crates = list_all_crates()
    print(f"[script/fix.py] Found crates: {",".join(crate.name for crate in crates)}")
    for crate in crates:
        cargo_fix_lib(crate.name)

    cargo_fix_bin(".")

if __name__ == "__main__":
    main()
