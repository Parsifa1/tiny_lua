{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        system = "${system}";
        config.allowUnfree = true;
      };
    in {
      devShells.default = pkgs.mkShell {
        allowUnfree = true;
        packages = [pkgs.lua53Packages.lua];
      };
    });
}
