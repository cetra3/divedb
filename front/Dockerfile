FROM node:16

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && \
    apt-get install -y inotify-tools && \
    rm -rf /var/lib/apt/lists/*
    

WORKDIR /front

COPY docker.sh package.json yarn.lock /front


RUN yarn install

COPY svelte.config.js vite.config.ts tsconfig.json /front
COPY static /front/static
COPY src /front/src

ENTRYPOINT ["/front/docker.sh"]