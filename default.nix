with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "WeekendAtJoes";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    libmysql
    sqlite
    postgresql
  ];
  shellHook = ''
    pg_ctl init -D pgsql
    pg_ctl -D pgsql -l db.logfile start
  '';
}