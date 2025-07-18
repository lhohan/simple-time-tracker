{
  description = "Nix flake for Rust project with just support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShell = pkgs.mkShell {
      buildInputs = with pkgs; [
        # cargo-llvm-cov   # currently broken on nix packages... for now install manually
        cargo-nextest
        rustup
        just
      ];
    };
  });
}
