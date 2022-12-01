
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/38e591dd05ffc8bdf79dc752ba78b05e370416fa.tar.gz") {}}:

pkgs.mkShell {
    buildInputs = [
        pkgs.cargo
        pkgs.rustc
    ];
}
