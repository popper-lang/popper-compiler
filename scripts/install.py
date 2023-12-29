import tempfile
import subprocess
import pathlib


def getOS():
    import platform
    return platform.system()

def installLLVM():
    print("Installing LLVM 16...")
    if getOS() == "MacOS":
        subprocess.call(["brew", "install", "llvm@16"])
    elif getOS() == "Windows":
        print("Not supported yet.")
    elif getOS() == "Linux":

        # create a temporary directory
        tmpdir = tempfile.mkdtemp()
        path = pathlib.Path(tmpdir)

        # download the LLVM 16 tarball
        subprocess.call(["wget", "https://github.com/llvm/llvm-project/archive/refs/tags/llvmorg-16.0.0.zip"], cwd=path)

        path.joinpath("llvmorg-16.0.0.zip")


        # unzip the tarball
        subprocess.call(["unzip", path], cwd=path)
        # build LLVM 16




if __name__ == "__main__":
    print("Installing popper-compiler...")
