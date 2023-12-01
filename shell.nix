with import <nixpkgs> { };
mkShell {
  nativeBuildInputs = with pkgs; [
    rustup

  ];
  PATH = "${builtins.getEnv "PATH"}:${builtins.getEnv "HOME"}/.cargo/bin";
}
