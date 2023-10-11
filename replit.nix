{ pkgs }: {
  deps = [
    pkgs.openssh
    pkgs.python38Packages.pip
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
    pkgs.python38Full
    pkgs.bitcoind
    pkgs.clightning
    pkgs.pstree
  ];
  env = {
    PYTHON_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    
    ];
    PYTHONBIN = "${pkgs.python38Full}/bin/python3.8";
    LANG = "en_US.UTF-8";
  };
}
