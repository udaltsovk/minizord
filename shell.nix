{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
      ];
    },
}: let
  packages = with pkgs; [
    rust-bin.nightly.latest.default

    watchexec
    surrealdb-migrations
    protobuf
    cargo-udeps
    cargo-audit

    (nodejs_22.override {enableNpm = false;})
    pnpm
  ];

  libraries = with pkgs; [
    pkg-config
    openssl
  ];
in
  with pkgs;
    mkShell {
      name = "minizord";
      buildInputs = packages ++ libraries;

      DIRENV_LOG_FORMAT = "";
      LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
    }
