{
  description = "livediff — real-time terminal diffs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "livediff";
          version = "3.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            description = "Real-time file monitoring with beautiful diff visualization in the terminal";
            homepage = "https://socket7.github.io/Livediff/";
            license = [ licenses.mit licenses.asl20 ];
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            cargo
            clippy
            rustfmt
          ];
        };
      });
}
