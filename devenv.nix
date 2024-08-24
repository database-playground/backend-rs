{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";
  env.DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres";
  # env.DBRUNNER_ADDR = "";
  # env.FRONTEND_CORS_ORIGIN = "https://host1 https://host2";

  # https://devenv.sh/packages/
  packages =
    [
      pkgs.git
      pkgs.sqlx-cli
      pkgs.cargo-edit
      pkgs.cargo-nextest
      pkgs.protobuf
      pkgs.sqlfluff
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin (
      with pkgs.darwin.apple_sdk; [ frameworks.SystemConfiguration ]
    );

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.rust.channel = "nightly";

  # https://devenv.sh/processes/
  processes.backend = {
    exec = "cargo run --release";
    process-compose = {
      depends_on = {
        postgres.condition = "process_healthy";
      };
      environment = [
        "PORT=30000"
      ];
    };
  };

  # https://devenv.sh/services/
  services.postgres.enable = true;
  services.postgres.listen_addresses = "127.0.0.1";
  services.postgres.initialScript = ''
    CREATE ROLE postgres SUPERUSER LOGIN PASSWORD 'postgres';
  '';

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks = {
    shellcheck.enable = true;
    rustfmt.enable = true;
    clippy.enable = true;

    sqlx-prepare = {
      enable = true;
      name = "Run sqlx prepare";

      entry = "cargo sqlx prepare";
      types = [ "rust" ];

      pass_filenames = false;
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
