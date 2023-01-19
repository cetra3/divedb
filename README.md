![](front/static/logo.png)

This is the source repository for the DiveDB site running at: https://divedb.net

## What is DiveDB

DiveDB is an open-source web application that allows Scuba Divers to catalogue and record their dives, dive sites, sealife and photos.

The backend is written in `rust` using `actix-web` and the frontend uses `sveltekit`.

## Content Model

There are four _major_ objects that make up the content model of divedb:

- Dives: a logged dive
- Sites: a dive site that you can dive at
- Sealife: a species of sealife
- Photos: photos related to diving

They all interact _mostly_ intuitively: You can log a dive, which can have a related dive site & photos, with each photo potentially being linked to a certain type of sealife.

Your best bet to get a feel for these things is to [look at the live site](https://divedb.net)

## License

All source code within DiveDB is [licensed under AGPL 3.0](LICENSE.md)

## Developing

You will need the following tools installed:

- Rust/Cargo
- Docker/Docker Compose
- NodeJS/SvelteKit

If you just want to take it for a test run on your own dev machine, you can use docker compose. I.e,

```
docker compose up --build -d
```

### Developing The Backend

The backend is a rust web application, so you will need rust installed.

Normally I run up the database in docker with a `docker compose up -d database`, and then do a `cargo run` or `cargo run --release` in this directory.

### Developing the Frontend

You will need `yarn` or `node` installed & then you can `yarn install && yarn dev`
