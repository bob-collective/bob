FROM node:22-slim
EXPOSE 4000

RUN apt-get update \
    && apt-get install --no-install-recommends -y tini curl ca-certificates git \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L https://foundry.paradigm.xyz | bash && \
    . /root/.bashrc && \
    /root/.foundry/bin/foundryup && \
    cp /root/.foundry/bin/* /usr/local/bin/

WORKDIR /home/node/app
COPY --chown=node:node . .

RUN corepack enable \
    && corepack prepare pnpm@latest-9 --activate \
    && chown -R node:node /home/node/app

USER node
RUN forge install

WORKDIR /home/node/app/sdk
RUN npm ci && npm run build

ENTRYPOINT [ "tini", "--", "node" ]
