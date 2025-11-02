derivation {
  name = "simple";
  builder = "${(import <nixpkgs> {}).bash}/bin/bash";
  args = ["-c" "echo foo > $out"];
  src = ./.;
  system = builtins.currentSystem;
}
