FROM node:16

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && \
    apt-get install -y inotify-tools && \
    rm -rf /var/lib/apt/lists/*
    

WORKDIR /svelte

COPY docker.sh package.json yarn.lock /svelte


RUN yarn install

COPY svelte.config.js vite.config.ts tsconfig.json /svelte
COPY static /svelte/static
COPY src /svelte/src

ENTRYPOINT ["/svelte/docker.sh"]