import subprocess
import pathlib
import re
import platform
import os



LLVM_REGEX_VERSION = re.compile(r"16\.(\d+)\.(\d+)")  # check if the version is 16.x.x
RUSTUP_CMD = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable"

def getOS():
    sys = platform.system()
    if sys == "Darwin":
        return "MacOS"
    elif sys == "Windows":
        return "Windows"
    elif sys == "Linux":
        return "Linux"
    else:
        return "Unknown"

def isDebianBased():
    return getOS() == "Linux" and pathlib.Path("/etc/debian_version").exists()

def installCMake():
    print("Installing CMake...")
    if getOS() == "MacOS":
        subprocess.call(["brew", "install", "cmake"])
    elif getOS() == "Windows":
        print("Not supported yet.")
    elif getOS() == "Linux":
        if isDebianBased():
            subprocess.call(["sudo", "apt", "install", "cmake"])

def checkIfLLVM16Installed():
    output = subprocess.run(["llvm-config", "--version"], check=True)
    return output.returncode == 0 and LLVM_REGEX_VERSION.match(output.stdout)

def installLLVM16():
    print("Installing LLVM 16...")
    if getOS() == "MacOS":
        subprocess.call(["brew", "install", "llvm@16"])
    elif getOS() == "Windows":
        print("Not supported yet.")
    elif getOS() == "Linux":
        if isDebianBased():
            subprocess.call(["sudo", "apt", "install", "llvm-16"])

def getLLVMPrefix():
    output = subprocess.run(["llvm-config", "--prefix"], check=True)
    return str(output.stdout)

def checkIfRustInstalled():
    output = subprocess.run(["rustc", "--version"], check=True)
    return output.returncode == 0

def installRust():
    print("Installing Rust...")
    os.system(RUSTUP_CMD)

def main():
    if not checkIfLLVM16Installed():
        installLLVM16()
    if not checkIfRustInstalled():
        installRust()
    print("Building popper-compiler...")
    subprocess.run(["cargo", "build", "--release"], check=True)
    print("setting LLVM_SYS_160_PREFIX variable environment...")
    os.environ["LLVM_SYS_160_PREFIX"] = getLLVMPrefix()
    print("Copying popper-compiler to /usr/local/bin")
    subprocess.run(["cp", "target/release/popper-compiler", "/usr/local/bin/"], check=True)



if __name__ == "__main__":
    main()
