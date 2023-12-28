{ stdenv,
  fetchFromGitHub,
  llvmPackages_16,
  rustc,
  cargo

}:

stdenv.mkDerivation {
  name = "popper-compiler";
  src = fetchFromGitHub {
    owner = "popper-lang";
    repo = "popper-compiler";
    rev = "v1";
    sha256 = "G79NAe2cgfEfDvJL0JSY2bnIpvJA6OSzL6kQ/WMRCZA=";
  };

  buildInputs = [ llvmPackages_16.libllvm rustc cargo];

  installPhase = ''
      export LLVM_SYS_160_PREFIX="$(llvm-config  --prefix)"
      cargo build --release
      mkdir -p $out/bin
      cp target/release/popper-compiler $out/bin/popper-compiler
      rm -rf target/release
    '';

}