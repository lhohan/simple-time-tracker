{
  description = "Nix flake for Rust project with just support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages.${system};
    structurizrCliFixed = pkgs.stdenvNoCC.mkDerivation {
      pname = "structurizr-cli";
      version = "2025.05.28";

      src = pkgs.fetchzip {
        url = "https://github.com/structurizr/cli/releases/download/v2025.05.28/structurizr-cli.zip";
        hash = "sha256-UYqUZydjsAoy5be/UhAyX/7OvLq8pXA6STwbEnCG7CU=";
        stripRoot = false;
      };

      nativeBuildInputs = [ pkgs.makeBinaryWrapper ];

      installPhase = ''
        runHook preInstall

        mkdir -p $out/bin $out/lib/structurizr-cli
        cp -r . $out/lib/structurizr-cli
        chmod +x $out/lib/structurizr-cli/structurizr.sh

        makeBinaryWrapper $out/lib/structurizr-cli/structurizr.sh $out/bin/structurizr-cli \
          --prefix PATH : "${
            pkgs.lib.makeBinPath [
              pkgs.jre
            ]
          }"

        runHook postInstall
      '';
    };
  in {
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        # cargo-llvm-cov   # currently broken on nix packages... for now install manually
        cargo-nextest
        cargo-fuzz
        graphviz
        rustup
        just
        structurizrCliFixed
        python312  # dependency of Serena MCP
        uv         # dependency of Serena MCP
      ];
    };
  });
}
