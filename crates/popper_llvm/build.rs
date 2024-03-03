#![allow(clippy::upper_case_acronyms)]
struct LLVM {
    version: String,
    prefix: String,
}

fn found_llvm() -> Option<LLVM> {
    let llvm_prefix = std::process::Command::new("llvm-config")
        .arg("--prefix")
        .output();

    if llvm_prefix.is_err() {
        None
    } else {
        let llvm_prefix = llvm_prefix.unwrap();
        let llvm_prefix = std::str::from_utf8(&llvm_prefix.stdout).unwrap();
        let llvm_prefix = llvm_prefix.trim();
        let llvm_version = std::process::Command::new("llvm-config")
            .arg("--version")
            .output()
            .unwrap();

        let llvm_version = std::str::from_utf8(&llvm_version.stdout).unwrap();
        let llvm_version = llvm_version.trim();
        let llvm = LLVM {
            version: llvm_version.to_string(),
            prefix: llvm_prefix.to_string(),
        };
        Some(llvm)
    }
}

fn main() {
    let llvm = found_llvm();
    if llvm.is_none() {
        panic!(
            r#"
        ==============================
        LLVM not found
        ==============================
        please install LLVM first (https://llvm.org/)
        "#
        )
    }

    let llvm = llvm.unwrap();
    let llvm_version = llvm.version;
    let llvm_prefix = llvm.prefix;
    let major_version = llvm_version.split('.').next().unwrap();
    if major_version != "17" {
        panic!(
            r#"
        ==============================
        LLVM version not supported
        ==============================
        please install LLVM 17 (https://llvm.org/)
        "#
        )
    }

    std::env::set_var("LLVM_SYS_170_PREFIX", llvm_prefix);
}
