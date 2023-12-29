#!/bin/sh


$LLVM_PREFIX = "$(llvm-config --prefix)"
platform='unknown'
unamestr=$(uname)
if [[ "$unamestr" == 'Linux' ]]; then
   platform='linux'
elif [[ "$unamestr" == 'FreeBSD' ]]; then
   platform='freebsd'
elif [[ "$unamestr" == 'Darwin' ]]; then 
   platform='macos'
fi


installCMake() {
  echo "Installing cmake... "
  apt-get install cmake 

}

installLLVM() {
  $directory = "$(mktemp -d)"
  echo "install LLVM on $directory"
  cd $directory
  wget https://github.com/llvm/llvm-project/archive/refs/tags/llvmorg-16.0.0.zip > llvm.zip

  unzip "llvmorg-16.0.0.zip"
  cd "llvm-project-llvmorg-16.0.0"
  cmake -S llvm -B build -G "Unix Makefiles" -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;libcxx;libcxxabi;libunwind;lldb;compiler-rt;lld;polly" -DLLVM_ENABLE_RTTI=ON -DLLVM_BUILD_EXAMPLES=ON -DLLVM_TARGETS_TO_BUILD="X86;ARM;AArch64" -DCMAKE_BUILD_TYPE=Release
  make install

  # Add llvm to path
  echo "export PATH=$LLVM_PREFIX/bin:$PATH" >> ~/.bashrc

  export PATH=$LLVM_PREFIX/bin:$PATH

}

installRust() {
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
}

installGit() {
  # Check the operating system
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Check if the operating system is debian based
    if [[ -f /etc/debian_version ]]; then
      # Debian based
      sudo apt-get install git
    elif [[ -f /etc/redhat-release ]]; then
      # Red Hat based
      sudo yum install git
    fi
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    # Mac OSX
    brew install git
  fi
}

setupProject() {
    $tmp_dir = mktemp -d
    cd $tmp_dir
    git clone https://github.com/popper-lang/popper-compiler.git
    cd popper-compiler
    cargo build --release
    cp target/release/popper-compiler /usr/local/bin/popper-compiler
    rm -rf $tmp_dir
    echo "Popper compiler is installed"
}

# Check if git is installed
if ! command -v git >/dev/null 2>&1; then
    echo "Git is not installed. Git install..."
    installGit
else
  echo "Git is installed"
fi


if ! command -v cmake >/dev/null 2>&1; then
    echo "Cmake is not installed. Cmake install..."
    installCMake
else
  echo "Cmake is installed"
fi


# Check if rust is installed
if ! command -v rustc >/dev/null 2>&1; then
    echo "Rust is not installed. Rust install .."
    installRust
else
  echo "Rust is installed"
fi

# Check if llvm is installed
if ! command -v llvm-config >/dev/null 2>&1; then
    echo "LLVM is not installed. Please install it and try again."
    installLLVM
else
  # check if llvm version match 16.*
  if [[ $(llvm-config --version) =~ ^16.* ]]; then
    echo "LLVM version is 16.*"
  else
    echo "LLVM version is not 16.*"
    installLLVM
  fi
fi

# Check if popper-compiler is installed
if ! command -v popper-compiler >/dev/null 2>&1; then
    echo "Popper compiler is not installed. Please install it and try again."
    setupProject
else
  echo "Popper compiler is installed"
fi