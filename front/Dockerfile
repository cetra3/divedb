# Build stage
FROM node:22 AS builder

WORKDIR /front

COPY package.json yarn.lock /front/

RUN yarn install --frozen-lockfile

COPY svelte.config.js vite.config.ts tsconfig.json /front/
COPY static /front/static
COPY src /front/src

RUN yarn build

FROM node:22-slim AS production

WORKDIR /front

COPY package.json yarn.lock /front/

RUN yarn install --frozen-lockfile --production && yarn cache clean

COPY --from=builder /front/build /front/build
COPY --from=builder /front/static /front/static

ENTRYPOINT ["node", "build"]
