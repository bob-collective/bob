FROM node:22-slim
EXPOSE 4000

RUN apt-get update \
    && apt-get install --no-install-recommends -y tini curl ca-certificates git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /home/node/app
COPY ./sdk ./sdk
COPY --chown=node:node ./script ./sdk/script

WORKDIR /home/node/app/sdk

RUN corepack enable \
    && corepack prepare pnpm@latest-9 --activate \
    && npm ci \
    && npm run build

RUN curl -L https://foundry.paradigm.xyz | bash && \
    . /root/.bashrc && \
    foundryup && \
    mv /root/.foundry/bin/* /usr/local/bin

USER node
RUN mkdir /home/node/.npm/

WORKDIR /home/node/app/sdk/dist/
ENTRYPOINT [ "tini", "--", "node" ]
