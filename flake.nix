{
  description = "givn — single-binary, Gherkin-first spec-driven workflow tool";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        givn = pkgs.rustPlatform.buildRustPackage {
          pname = "givn";
          inherit ((pkgs.lib.importTOML ./Cargo.toml).package) version;
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          # Unit tests shell out to git.
          nativeCheckInputs = [ pkgs.git ];
          # Cucumber runners (features_runner, ready_fixture) need a full project checkout.
          cargoTestFlags = [
            "--lib"
            "--bins"
          ];
          meta = {
            description = "A single-binary, Gherkin-first spec-driven workflow tool";
            homepage = "https://github.com/buster/givn";
            license = with pkgs.lib.licenses; [
              mit
              asl20
            ];
            mainProgram = "givn";
          };
        };
        default = givn;
      });

      overlays.default = final: _prev: {
        givn = self.packages.${final.stdenv.hostPlatform.system}.default;
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
          ];
        };
      });
    };
}
