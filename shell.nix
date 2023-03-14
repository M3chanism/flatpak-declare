with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc cargo
    rustfmt
    rust-analyzer
    clippy

    # Example Build-time Additional Dependencies
    # pkg-config
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    # openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
