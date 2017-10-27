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
    export PGDATA='pgsql'
    pg_ctl init
    pg_ctl -l db.logfile start
  '';
}