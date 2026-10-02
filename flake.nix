{
  description = "shortest-sum — CLI + Leptos web interface";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (
        pkgs:
        let
          pkg = pkgs.rustPlatform.buildRustPackage {
            pname = "shortest-sum";
            version = "0.1.0";
            src = self;

            cargoLock.lockFile = ./Cargo.lock;

            # cargo-leptos (built with no_downloads) shells out to these from PATH:
            # wasm-bindgen must match the pin in Cargo.toml, wasm-opt comes from
            # binaryen, and nixpkgs rustc ships without rust-lld so the wasm target
            # links through lld's wasm-ld (same as devenv.nix).
            nativeBuildInputs = with pkgs; [
              cargo-leptos
              wasm-bindgen-cli
              binaryen
              lld
              makeWrapper
            ];

            buildPhase = ''
              runHook preBuild
              cargo leptos build --release
              cargo build --release --offline --bin shortest-sum
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin $out/share/shortest-sum
              cp target/release/shortest-sum target/release/shortest-sum-web $out/bin/
              cp -r target/site $out/share/shortest-sum/site
              wrapProgram $out/bin/shortest-sum-web \
                --set LEPTOS_SITE_ROOT $out/share/shortest-sum/site
              runHook postInstall
            '';

            meta = {
              description = "Shortest path solver on a bounded number line, with a Minecraft-style web UI";
              mainProgram = "shortest-sum-web";
              platforms = pkgs.lib.platforms.unix;
            };
          };
        in
        {
          default = pkg;
          shortest-sum = pkg;
        }
      );

      nixosModules = rec {
        default =
          {
            config,
            lib,
            pkgs,
            ...
          }:
          let
            cfg = config.services.shortest-sum;
          in
          {
            options.services.shortest-sum = {
              enable = lib.mkEnableOption "the shortest-sum web service";

              package = lib.mkOption {
                type = lib.types.package;
                default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
                defaultText = lib.literalExpression "self.packages.\${pkgs.stdenv.hostPlatform.system}.default";
                description = "The shortest-sum package to run.";
              };

              user = lib.mkOption {
                type = lib.types.str;
                default = "shortest-sum";
                description = "User account under which the service runs.";
              };

              group = lib.mkOption {
                type = lib.types.str;
                default = "shortest-sum";
                description = "Group owning the files of the service.";
              };

              address = lib.mkOption {
                type = lib.types.str;
                default = "127.0.0.1";
                description = "Address the server listens on.";
              };

              port = lib.mkOption {
                type = lib.types.port;
                default = 3000;
                description = "Port the server listens on.";
              };

              openFirewall = lib.mkOption {
                type = lib.types.bool;
                default = false;
                description = "Whether to open the port in the firewall.";
              };

              extraEnvironment = lib.mkOption {
                type = lib.types.attrsOf lib.types.str;
                default = { };
                description = "Additional environment variables for the service.";
              };
            };

            config = lib.mkIf cfg.enable {
              users.users.${cfg.user} = {
                isSystemUser = true;
                group = cfg.group;
                description = "shortest-sum service user";
              };
              users.groups.${cfg.group} = { };

              systemd.services.shortest-sum = {
                description = "shortest-sum web service";
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];

                serviceConfig = {
                  User = cfg.user;
                  Group = cfg.group;
                  ExecStart = "${cfg.package}/bin/shortest-sum-web";
                  Restart = "on-failure";
                };

                environment = {
                  LEPTOS_SITE_ADDR = "${cfg.address}:${toString cfg.port}";
                  LEPTOS_ENV = "PROD";
                }
                // cfg.extraEnvironment;
              };

              networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ cfg.port ];
            };
          };
        shortest-sum = default;
      };
    };
}
