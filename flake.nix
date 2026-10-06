{
  description = "givn — single-binary, Gherkin-first spec-driven workflow tool";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

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
          version = "${(pkgs.lib.importTOML ./Cargo.toml).package.version}-${
            self.shortRev or self.dirtyShortRev or "dirty"
          }";
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
              gpl3Plus
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
        default =
          let
            opencode =
              let
                lock = builtins.fromJSON (builtins.readFile ./tests/fixtures/opencode/runtime-lock.json);
                platform = pkgs.stdenv.hostPlatform.system;
                fetchNpm = name: release:
                  let
                    metadata = lock.runtimes.${name};
                    source = metadata.platforms.${platform};
                    archive = pkgs.fetchurl { url = source.url; hash = source.integrity; };
                  in pkgs.runCommand "${name}-${release}" { nativeBuildInputs = [ pkgs.gnutar pkgs.gzip ]; } ''
                    mkdir -p $out/bin
                    tar -xzf "${archive}" -C "$TMPDIR"
                    install -m755 "$TMPDIR/package/bin/opencode" "$out/bin/opencode"
                  '';
                v1Floor = fetchNpm "opencode_v1_floor" "1.18.29";
                v1Latest = fetchNpm "opencode_v1_latest" "1.18.34";
                v2Latest = fetchNpm "opencode_v2_latest" "2.0.22";
                node = pkgs.nodejs_24;
                moduleLock = builtins.fromJSON (builtins.readFile ./tests/fixtures/opencode/modules/package-lock.json);
                modulePackages = pkgs.lib.filterAttrs
                  (path: entry: path != "" && pkgs.lib.hasPrefix "node_modules/" path && entry ? resolved && entry ? integrity)
                  moduleLock.packages;
                moduleSources = pkgs.lib.mapAttrs (_: entry: pkgs.fetchurl { url = entry.resolved; hash = entry.integrity; }) modulePackages;
                moduleInstall = pkgs.lib.concatStringsSep "\n" (pkgs.lib.mapAttrsToList (path: archive:
                  let destination = pkgs.lib.removePrefix "node_modules/" path;
                  in ''
                    mkdir -p "$out/lib/node_modules/${destination}"
                    tar -xzf "${archive}" --strip-components=1 -C "$out/lib/node_modules/${destination}"
                  '') moduleSources);
                modules = pkgs.runCommand "givn-opencode-fixture-modules" { nativeBuildInputs = [ pkgs.gnutar pkgs.gzip ]; } ''
                  mkdir -p "$out/lib/node_modules"
                  ${moduleInstall}
                '';
              in {
                packages = with pkgs; [ node fish cargo-nextest git fossil git-filter-repo
                  (if pkgs.stdenv.hostPlatform.isDarwin then expect else util-linux)
                  openssl curl jq gnutar gzip coreutils v1Floor v1Latest v2Latest modules ];
                paths = {
                  GIVN_OPENCODE_V1_FLOOR = "${v1Floor}/bin/opencode";
                  GIVN_OPENCODE_V1_LATEST = "${v1Latest}/bin/opencode";
                  GIVN_OPENCODE_V2_LATEST = "${v2Latest}/bin/opencode";
                  GIVN_OPENCODE_NODE = "${node}/bin/node";
                  GIVN_OPENCODE_FISH = "${pkgs.fish}/bin/fish";
                  GIVN_OPENCODE_GIT = "${pkgs.git}/bin/git";
                  GIVN_OPENCODE_MODULES = "${modules}/lib/node_modules";
                };
              };
            llvmTools = pkgs.rustc.llvmPackages.llvm;
          in
          pkgs.mkShell {
            packages = with pkgs; [
              cargo
              cargo-llvm-cov
              clippy
              rustc
              rustfmt
              llvmTools
            ] ++ opencode.packages;
            # cargo-llvm-cov finds no rustup llvm-tools component here, so it is told
            # the matching LLVM tools of the Nix rustc (same LLVM version as rustc).
            shellHook = builtins.concatStringsSep "\n" ((pkgs.lib.mapAttrsToList (name: value:
              "export ${name}=${pkgs.lib.escapeShellArg value}") (opencode.paths // {
                LLVM_COV = "${llvmTools}/bin/llvm-cov";
                LLVM_PROFDATA = "${llvmTools}/bin/llvm-profdata";
              })));
          };
      });
    };
}
