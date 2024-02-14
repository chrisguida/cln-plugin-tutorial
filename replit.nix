{ pkgs }: {
  deps = [
    pkgs.vim
    pkgs.openssh
    pkgs.python310Packages.pip
    pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
    pkgs.rust-analyzer
    pkgs.go
    pkgs.gopls
    pkgs.busybox
    pkgs.htop
    pkgs.less
    pkgs.python310Full
    pkgs.bitcoind
    pkgs.clightning
    pkgs.pstree
  ];
  env = {
    PYTHON_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    
    ];
    PYTHONBIN = "${pkgs.python310Full}/bin/python3.10";
    LANG = "en_US.UTF-8";
  };
}
