{
  inputs = {
    fenix.url = "github:nix-community/fenix/monthly";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs, gitignore }:
  let
    inherit (gitignore.lib) gitignoreSource;
  in
  flake-utils.lib.eachDefaultSystem (
    system: let
      pkgs = (import nixpkgs) {
        inherit system;
      };

      toolchain = with fenix.packages.${system};
      combine [
        minimal.rustc
        minimal.cargo
        targets.x86_64-unknown-linux-musl.latest.rust-std
      ];

      naersk' = naersk.lib.${system}.override {
        cargo = toolchain;
        rustc = toolchain;
      };

    in rec {
      defaultPackage = naersk'.buildPackage {
        pname = "boh-cli";
        # root = gitignoreSource ./.;

        #gitSubmodules = true;
        # src = gitignoreSource ./.;
	src = ./.;
        doCheck = false;

	#cargoWorkspaceDir = "BookOfHours/boh-cli";
	#additionalCargoLock = "BookOfHours/boh-cli/Cargo.lock";

        buildInputs = with pkgs; [
          pkgsStatic.openssl
        ];

        nativeBuildInputs = (with pkgs; [
          pkgsStatic.stdenv.cc
          pkg-config
        ]); # ++ (with pkgs.nodejs-18_x.pkgs; [
        #]);
        
        #postInstall = ''
        #  mkdir $out/conf
        #  cp ${./server-sample.conf} $out/conf
        #  cp ${./server-sample-prosys.conf} $out/conf
        #'';


        # Tells Cargo that we're building for musl.
        # (https://doc.rust-lang.org/cargo/reference/config.html#buildtarget)
        CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";

        # Tells Cargo to enable static compilation.
        # (https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags)
        #
        # Note that the resulting binary might still be considered dynamically
        # linked by ldd, but that's just because the binary might have
        # position-independent-execution enabled.
        # (see: https://github.com/rust-lang/rust/issues/79624#issuecomment-737415388)
        CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
      };

      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          #nodejs-18_x
          #zip
          #openssl
          bashInteractive
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
          toolchain
        ];
      };
    }
    );
  }
