{ lib
, stdenv
, fetchzip
}:

stdenv.mkDerivation {
  name = "popper-compiler";
  src = fetchzip {

    url = "https://github.com/popper-lang/popper-compiler/archive/refs/tags/v1.zip";

    sha256 = "a5167f00ca8047c240240cba6d539c36e8b5270a";

  };

}