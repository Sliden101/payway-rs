{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "payway-rs";

  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo

    # Build dependencies
    pkg-config
    openssl
    libiconv
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages_18.libclang}/lib";

  RUST_BACKTRACE = "1";

  shellHook = ''
    echo "PayWay Rust SDK Development Environment"
    echo "======================================="
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
  '';

  env = {
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
  };
}
