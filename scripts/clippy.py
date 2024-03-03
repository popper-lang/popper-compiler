import subprocess
import git
import fix

def cargo_clippy_fix(lib):
    print(f"[script/clippy.py] Run cargo clippy --fix for {lib}...")
    subprocess.run(["cargo", "clippy", "--fix", "--lib", "-p", lib])
    git.commit(f"fix: clippy for {lib}")

def main():
    crates = [c.name for c in fix.list_all_crates()]

    crates.remove("popper_parser")
    for lib in crates:
        cargo_clippy_fix(lib)


if __name__ == "__main__":
    main()
