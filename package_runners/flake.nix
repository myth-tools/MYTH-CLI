# ==============================================================================
# MYTH-CLI : Pure Nix Flake Distribution
# Standard: Deterministic, Hermetic, Reproducible
# 
# Usage: nix run github:myth-tools/MYTH-CLI?dir=package_runners -- scan target
# ==============================================================================
{
  description = "MYTH: SOTA AI-Powered Reconnaissance Operative";

  inputs = {
    # NOTE: Pin to a stable nixpkgs commit for reproducible builds.
    # Update with: nix flake update
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
      in {
        # ─── Artifact Compilation Sequence ───
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;

          src = ../.;
          cargoLock.lockFile = ../Cargo.lock;

          # Enforce pure compilation isolation
          nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];
          buildInputs = [ pkgs.openssl pkgs.zstd ];

          # Dynamic Path Resolution (Injects core dependencies completely invisibly)
          postInstall = let
            # Discrete Browser Engine Provisioning (Deterministic fetch)
            lightpanda = pkgs.stdenv.mkDerivation {
              pname = "lightpanda-bin";
              version = "1.0.0-nightly";
              # NOTE: Lightpanda 'nightly' binaries are updated frequently.
              # The sha256 MUST be updated every time the nightly release changes.
              # To get the correct hash, run:
              #   nix-prefetch-url "https://github.com/lightpanda-io/browser/releases/download/nightly/lightpanda-x86_64-linux"
              # Then paste the resulting hash below and replace the placeholder.
              src = pkgs.fetchurl {
                url = "https://github.com/lightpanda-io/browser/releases/download/nightly/lightpanda-x86_64-linux";
                # Replace with the actual SHA256 from nix-prefetch-url output:
                # sha256 = "sha256-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
                sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
              };
              phases = [ "installPhase" ];
              installPhase = ''
                mkdir -p $out/bin
                cp $src $out/bin/lightpanda
                chmod +x $out/bin/lightpanda
              '';
            };
          in ''
            wrapProgram $out/bin/myth \
              --prefix PATH : ${pkgs.lib.makeBinPath [ 
                pkgs.bubblewrap 
                pkgs.tor 
                pkgs.nmap 
                pkgs.dnsutils 
                pkgs.curl 
                pkgs.git
                lightpanda
              ]} \
              --set MYTH_NIX_MODE "1"
          '';

          meta = with pkgs.lib; {
            description = cargoToml.package.description;
            homepage = cargoToml.package.homepage;
            license = licenses.mit;
            platforms = platforms.linux; # Strict OS confinement
            mainProgram = "myth";
            creator = {
              name = "Shesher Hasan";
              role = "Chief Architect";
              contact = "shesher0llms@gmail.com";
              organization = "myth-tools";
              clearance_level = "OPERATIVE-LEVEL-4 (SENIOR ARCHITECT)";
              system_license = "MYTH-INSTITUTIONAL-COMMERCIAL-2026-BETA";
            };
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        # ─── Elite Developer Environment Shell (nix develop) ───
        devShells.default = pkgs.mkShell {
          # Rust toolchain + development utilities
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
            clippy
            rust-analyzer
            rustfmt
            cargo-watch
          ];
          # Runtime system libraries
          buildInputs = with pkgs; [
            openssl
            bubblewrap
            tor
            nmap
          ];
          shellHook = ''
            export MYTH_DEV_ENV="1"
            echo "⚡ MYTH Neural Development Sandbox Active."
            echo "   Rust toolchain and offensive arsenal synchronized natively."
          '';
        };
      });
}
