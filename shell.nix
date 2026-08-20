{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    cargo
    rustc
    pkgsCross.riscv64-embedded.buildPackages.binutils
    gnumake
  ];

  shellHook = ''
    echo "=================================================="
    echo "  XZXT RISC-V Bare-Metal Firmware Environment"
    echo "=================================================="
    
    # Automatically set a default toolchain if none exists
    if ! rustup toolchain list | grep -q "stable"; then
      echo "Setting default rustup toolchain to stable..."
      rustup default stable
    fi
    
    # Automatically ensure the bare-metal target is installed via rustup
    rustup target add riscv64imac-unknown-none-elf 2>/dev/null || true
    
    echo "Target ready: riscv64imac-unknown-none-elf"
    echo "Toolchain active: $(rustc --version)"
    echo "--------------------------------------------------"
  '';
}
