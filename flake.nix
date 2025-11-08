{
  description = "Chatbot Volt Crédito - Middleware ClickMassa";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
            
            # Ferramentas adicionais
            pkg-config
            libiconv
            
            # Para debugging e testes
            cargo-watch
            cargo-edit
            
            # Outros tools úteis
            git
          ];

          shellHook = ''
            echo "🚀 Ambiente Rust carregado!"
            echo "Versão do Rust:"
            rustc --version
            echo ""
            echo "Cargo disponível:"
            cargo --version
            echo ""
            echo "💡 Comandos úteis:"
            echo "  cargo build        - Compilar projeto"
            echo "  cargo run          - Executar projeto"
            echo "  cargo test         - Rodar testes"
            echo "  cargo watch -x run - Executar com hot reload"
          '';

          # Variáveis de ambiente
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
        };
      }
    );
}
