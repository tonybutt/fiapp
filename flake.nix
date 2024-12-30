{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-trusted-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        devenv-test = self.devShells.${system}.default.config.test;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  processes = {
                    frontend.exec = ''
                      bun install --cwd frontend;
                      bun run --cwd frontend dev;
                    '';
                    backend.exec = ''
                      cargo watch -C backend -x 'run';
                    '';
                  };
                  git-hooks.hooks.treefmt = {
                    enable = true;
                    settings.formatters = [
                      pkgs.nixpkgs-fmt
                    ];
                  };
                  languages = {
                    javascript = {
                      enable = true;
                      bun.enable = true;
                    };
                    typescript.enable = true;
                    rust = {
                      enable = true;
                      channel = "nightly";
                    };
                  };
                }
              ];
            };
          });
    };
}
