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

## Deploying

To deploy you will need to follow these loose steps:

* Create a postgres database
* Have a look at docker compose to see how things talk to each other.  Both the frontend and backend need to be reachable to eachother.
* The backend will create a `store` and `thumbs` directory in the process working directory and will need permission to write images to them.
* Set up a domain and reverse SSL proxy for your Site URL. You can use let's encrypt and nginx or similar.  It should forward traffic to the backend port `3333`
* Configure environment variables as below for the site url, email settings etc..
* Run up the docker containers, both front and backend:
  * `ghcr.io/cetra3/divedb` - backend container
  * `ghcr.io/cetra3/divedb-front` - frontend container

### Backend Environment Variables

Here are env vars you will need to configure:

* `SITE_URL`: The site url, i.e, `https://divedb.net` or similar.  This is used for email links etc..
* `CONNECT_URL`: The postgres database url, i.e, `postgres://divedb:divedb@localhost:5432`
* `FRONTEND_URL`: The url of the frontend process/container.  I.e, `http://frontend:3000` in docker compose, or `http://localhost:3000`
* `ADMIN_EMAIL`: The email address of the admin user.  When a user is registered with this email they will be automatically promoted to admin.
* `SECRET_KEY`: A secret key for session management.  If not set, sessions are invalidated after a restart.  Needs to be 32 characters long. You can generate one with `openssl rand -hex 32`

#### SMTP Settings

You will need a valid smtp server to send emails for email registration and password reset. Configure the following environment variables:

* `SMTP_HOST`: The hostname or IP address of your SMTP server. defaults to `localhost`
* `SMTP_PORT`: The port number of your SMTP server. defaults to `25`
* `SMTP_SECURITY`: The security protocol to use, such as `none`, or `tls`.
* `SMTP_USERNAME`: The username for your SMTP server (if required)
* `SMTP_PASSWORD`: The password for your SMTP server (if required)
* `FROM_ADDR`: The email address to use as the sender for emails. defaults to `contact@divedb.net`

#### Open ID Connect Integration

You can configure an optional OpenID Connect provider for user authentication.

When configuring in your Provider the Redirect URI is always of the form `{SITE_URL}/oauth/callback`.  I.e, `https://divedb.net/oauth/callback`

Configure the following environment variables:

* `OPENID_ISSUER_NAME`: The name of the OpenID issuer.  This is to show a button on the login/register page
* `OPENID_ISSUER_URL`: The URL of the OpenID issuer i.e, `https://example.com`.  The issuer is required to have a `/.well-known/openid-configuration` endpoint.
* `OPENID_CLIENT_ID`: The client ID for your OpenID provider.
* `OPENID_CLIENT_SECRET`: The client secret for your OpenID provider.

#### Facebook Integration

If you want to enable Facebook login, you need to create a Facebook app and configure the following environment variables:

* `FB_APP_ID`: The app ID from facebook
* `FB_APP_SECRET`: The app secret from facebook.

### Frontend Environment Variables

* `BACKEND_URL`: The url of the backend process/container.  I.e, `http://backend:3333` in docker compose, or `http://localhost:3333`.
