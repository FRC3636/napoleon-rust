{
    pkgs ? import <nixpkgs> {
        localSystem = "x86_64-linux";
    },
}:
pkgs.callPackage (
    {
        mkShell,
        pkg-config,
        openssl,
        libudev-zero,
        llvm,
        clang,
        libclang,
        gcc,
        cxx-rs,
    }:
    mkShell {
        strictDeps = true;

        nativeBuildInputs = [
            clang
            pkg-config
            llvm
            gcc
            cxx-rs
        ];

        buildInputs = [
            libclang
            libudev-zero
            openssl
        ];
        LIBCLANG_PATH = ~/.nix-profile/lib;
        HOST_CC = ~/.nix-profile/bin/gcc;
        HOST_CXX = ~/.nix-profile/bin/g++;
    }
) { }
