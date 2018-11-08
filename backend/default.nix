with import <nixpkgs> {};


stdenv.mkDerivation rec {
  name = "WeekendAtJoes";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    libmysql
    sqlite
    postgresql


    openssl 
    # these two are optional, but they help with installing some rust Programs 
    pkgconfig
    zlib
    # used for compiling the frontend to js
  #    emscripten
  #  llvm
    libstdcxx5
    lld
  ];
  shellHook = ''
    export PGDATA='pgsql'
    # to set the password, run `psql` and enter `\password` and set it to the password below
    export DATABASE_URL='postgres://hzimmerman:password@localhost/weekend'
    export TEST_DATABASE_URL='postgres://hzimmerman:password@localhost/weekend_test'
    export DROP_DATABASE_URL='postgres://hzimmerman:password@localhost/postgres'

    pg_ctl init
    pg_ctl -l db.logfile start -o "-h localhost -i"

    alias docs='cargo rustdoc --bins --open -- --document-private-items'
  '';

}
