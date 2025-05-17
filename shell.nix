{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
      ];
    },
}: let
  packages = with pkgs; let
    jre = pkgs.zulu17;
    kotlin = pkgs.kotlin.override {inherit jre;};
  in [
    (rust-bin.nightly.latest.default.override {
      extensions = ["rust-src"];
    })

    watchexec
    surrealdb-migrations
    protobuf
    cargo-udeps
    cargo-audit
    cargo-expand

    jre
    kotlin

    bun
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
