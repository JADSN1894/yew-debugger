{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell
{
  nativeBuildInputs = with pkgs.buildPackages;
    [
      busybox
      curl
      vim
      tmux
      rsync
      cacert
      git
      tree
      unzip
      zstd
      iproute2
      jq
      clang
      llvm
      lldb
      glibc
      rustup
      musl
      cargo-sort
      cargo-audit
      cargo-deny
      cargo-vet
      cargo-generate
      trunk
      wasm-bindgen-cli
      dart-sass
      helix
      cocogitto
      difftastic
      watchexec
      hexyl
      vscodium
      #nixd
      nixpkgs-fmt
      sqlite
    ];
  shellHook = ''
    rustup default stable
    rustup component add rust-analyzer
    rustup component add rustfmt
    rustup component add clippy
    rustup target add $(arch -m)-unknown-linux-musl 
    alias cx='cargo xtask'
  '';
}


