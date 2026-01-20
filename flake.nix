{
  description = "Rust SSR Blog with Axum, Tera, and Tailwind CSS";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Build dependencies
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          tailwindcss
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        # Development shell packages
        devPackages = with pkgs; [
          cargo-watch
          tailwindcss
          bacon
          nodejs_22
          playwright-driver.browsers
        ];

        # Build the blog-server package
        blogPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "blog-server";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          # Compile Tailwind CSS before building
          preBuild = ''
            mkdir -p static/css
            tailwindcss -i static/css/input.css -o static/css/tailwind.css
          '';

          # Copy static assets after build (use ./static from build dir to include compiled CSS)
          postInstall = ''
            mkdir -p $out/share/blog-server
            cp -r ${./templates} $out/share/blog-server/templates
            cp -r ./static $out/share/blog-server/static
            cp -r ${./content} $out/share/blog-server/content
          '';

          meta = with pkgs.lib; {
            description = "A server-side rendered blog built with Rust";
            homepage = "https://github.com/yourusername/blog";
            license = with licenses; [ mit asl20 ];
            maintainers = [];
          };
        };

      in {
        packages = {
          default = blogPackage;
          blog-server = blogPackage;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ devPackages;

          shellHook = ''
            echo "🦀 Rust SSR Blog Development Environment"
            echo ""
            echo "Quick start:"
            echo "  cargo build        - Build the project"
            echo "  cargo watch -x run - Run with hot reload"
            echo "  ./scripts/watch-tailwind.sh - Watch Tailwind CSS"
            echo ""
            echo "Testing:"
            echo "  npm run test:e2e   - Run Playwright E2E tests"
            echo "  npm run test:e2e:ui - Run tests with UI"
            echo ""
            echo "Available tools:"
            echo "  rustc $(rustc --version | cut -d' ' -f2)"
            echo "  cargo $(cargo --version | cut -d' ' -f2)"
            echo "  node $(node --version)"
            echo "  tailwindcss $(tailwindcss --help | head -1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo 'available')"
            echo ""
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
          PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
        };

        # NixOS module
        nixosModules.default = import ./nixos/module.nix;
      }
    ) // {
      # System-independent outputs
      nixosModules.default = import ./nixos/module.nix;
    };
}
