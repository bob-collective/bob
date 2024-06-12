FROM node:22-slim
EXPOSE 4000

RUN apt-get update \
    && apt-get install --no-install-recommends -y tini curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /home/node/app/node_modules && chown -R node:node /home/node/app
WORKDIR /home/node/app
COPY . ./
RUN corepack enable \
    && corepack prepare pnpm@latest-9 --activate \
    && npm ci \
    && npm run build

USER node
RUN mkdir /home/node/.npm/
COPY --chown=node:node . .

WORKDIR /home/node/app/dist/
ENTRYPOINT [ "tini", "--", "node" ]
