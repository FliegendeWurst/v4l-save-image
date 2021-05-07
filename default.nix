{ pkgs ? import <nixpkgs> {},
fetchFromGitHub ? pkgs.fetchFromGitHub,
lib ? pkgs.lib,
rustPlatform ? pkgs.rustPlatform,
llvmPackages ? pkgs.llvmPackages,
libv4l ? pkgs.libv4l,
pkg-config ? pkgs.pkg-config,
openssl ? pkgs.openssl,
diffutils ? pkgs.diffutils
}:

rustPlatform.buildRustPackage rec {
  pname = "v4l-save-image";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "FliegendeWurst";
    repo = pname;
    # TODO: replace this with local directory
    rev = "ff1ddc952f36e6eeb2ff3991d252cf922ae8b860";
    sha256 = "0pz6yz4d04ilv7h28a2kcgmil84lwi04z7818h11zasqq1arxm66";
  };

  cargoSha256 = "1sc05qxs1fnwlnm16mw9481jfba1lnhnrcr9i3bm8b3q2a31lfka";

  nativeBuildInputs = [
    # TODO: Workaround for llvmPackages.bintools shadowing `diff`. Remove once
    # buildRustPackage is fixed.
    diffutils
    llvmPackages.libclang
    llvmPackages.clang
    llvmPackages.bintools
    pkg-config
    libv4l
  ];

  buildInputs = [
    libv4l
  ];

  LIBCLANG_PATH = llvmPackages.libclang + "/lib";

  meta = with lib; {
    homepage = "https://github.com/FliegendeWurst/v4l-save-image/";
    description = "Capture one webcam image using video4linux";
    license = licenses.gpl3;
  };
}
