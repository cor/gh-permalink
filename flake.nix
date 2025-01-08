{
  description = "gh-permaklink";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "gh-permalink";
      version = "0.1.0";
      src = ./.;

      cargoHash = "sha256-nioZFsZ4sFBH5/IpWlDwzA49FJFuOL7BWrrmbxOvxvY=";
    };

    devShell = pkgs.mkShell {
      nativeBuildInputs = [
        pkgs.cargo
        pkgs.rustc
      ];
    };
  });
}

