with import <nixpkgs> {};


stdenv.mkDerivation rec {
  name = "WeekendAtJoes";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    libmysql
    sqlite
    postgresql

    emscripten
  ];
  shellHook = ''
    export PGDATA='pgsql'
    export DATABASE_URL='http://localhost'
    pg_ctl init
    pg_ctl -l db.logfile start -o "-h localhost -i"
  '';

}
