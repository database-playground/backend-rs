# @database-playground/backend

The backend of Database Playground. Currently it acts as the resource server and the gateway of `dbrunner`.

You may need to bring your [Logto](https://logto.io) instance for the authorization server.

## Usage

You should prepare your PostgreSQL database. After that, set the `POSTGRES_URI` environment variable to the URI of your PostgreSQL database.

```bash
export POSTGRES_URI=postgresql://username:password@localhost:5432/database
```

Then, build and run the server:

```bash
cargo run --release
```

You can also build with Nix:

```bash
nix build .
```

## Deployment

Deploy it directly to [Zeabur](https://zeabur.com) with the following command:

```bash
npx zeabur deploy
```

You can also leverage the Nix package manager to build the Docker image for deployment:

```bash
# You should prepare a Linux remote builder.
nix build .#packages.aarch64-linux.docker  # For ARM64
nix build .#packages.x86_64-linux.docker   # For x86_64
docker load < result
```

## Development

We use [Devenv](https://devenv.sh) to manage the development environment, and [VS Code](https://code.visualstudio.com) as the IDE of Rust.

Install the recommended extensions and run `direnv allow && direnv reload` to set up the development environment.

Run `devenv up` to start the PostgreSQL service.

## License

This project is licensed under the AGPL-3.0-or-later license.
