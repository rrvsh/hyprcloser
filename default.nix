{
  pkgs ? import <nixpkgs> { },
}:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-EfavY64R9Z3W8PgcbSb8YeeFN5cuP0G+WhdpgOil96Y=";

  meta = {
    description = "Automatically close a window when it loses focus.";
    homepage = "https://github.com/rrvsh/hyprcloser";
    license = pkgs.lib.licenses.mit;
    maintainers = [ "rrvsh" ];
  };
}
