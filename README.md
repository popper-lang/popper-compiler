<h1 align="center" id="title">Popper-Lang</h1>

<p align="center"><img src="https://img.shields.io/badge/Langage-Rust-blue?style=for-the-badge&amp;logo=rust&amp;logoColor=white" alt="shields"><img src="https://img.shields.io/github/stars/popper-lang/popper-compiler.svg?style=for-the-badge&amp;logo=github" alt="shields"><img src="https://img.shields.io/github/forks/popper-lang/popper-compiler.svg?style=for-the-badge&amp;logo=github" alt="shields"><img src="https://img.shields.io/github/issues/popper-lang/popper-compiler.svg?style=for-the-badge&amp;logo=github" alt="shields"><img src="https://img.shields.io/github/issues-pr/popper-lang/popper-compiler.svg?style=for-the-badge&amp;logo=github" alt="shields"><img src="https://img.shields.io/github/license/popper-lang/popper-compiler.svg?style=for-the-badge&amp;logo=gnu" alt="shields"></p>

## Table Of Content (TOC)
 - [🧐 Features](https://github.com/popper-lang/popper-compiler#feature)
 - [🛠️ Installation Steps](https://github.com/popper-lang/popper-compiler#installation)
  
<h2 id="feature"> 🧐 Features</h2>

Here're some of the project's best features:

*   Easy to use
*   Fast
*   Simple code

<h2 id="installation">🛠️ Installation Steps:</h2>

<h3>1. Install LLVM(v16) </h3>

**WARNING** : Use the correct version of llvm otherwise the compiler will not run correctly : LLVM 16.x.x

<h4> For MacOS and Linux </h4>
you can install llvm with <a href="https://brew.sh/">brew</a> by running this command:

 ```sh
brew install llvm@16
```

to set up the llvm prefix you need to run this command :
```sh
export LLVM_SYS_160_PREFIX=$("$(brew --prefix llvm@16)"/bin/llvm-config --prefix)
```

<h4> For Debian based </h4>

You can install llvm with apt:
```sh
apt install llvm-16
```

**NOTE**: All llvm command are with the -16 suffix
to set up the llvm prefix you need to run this command :
```sh
export LLVM_SYS_160_PREFIX=$(llvm-config-16 --prefix)
```

<h4> Building from source </h4>

You can do there step to install LLVM from source: [llvm getting started](https://releases.llvm.org/16.0.0/docs/GettingStarted.html#overview)

<h3> Installing Rust </h3>

you can look at [rustup](https://rustup.rs/)

<h3> Download the Popper Compiler with git </h3>

```sh
git clone https://github.com/popper-lang/popper-compiler.git && cd popper-compiler
```
<h3> Building Source </h3>

I use [cargo](https://github.com/rust-lang/cargo) as the pkg manager for rust 

```sh
cargo build --release
```
<h3> Copying The binary </h3>

```
sudo cp target/releases/popper_compiler /bin
```

