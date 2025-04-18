FROM oven/bun:1.2.10-alpine AS base
WORKDIR /usr/src/minizord

FROM base AS install

RUN mkdir -p /tmp/dev
COPY package.json bun.lock /tmp/dev/
RUN cd /tmp/dev && bun install --frozen-lockfile

RUN mkdir -p /tmp/prod
COPY package.json bun.lock /tmp/prod/
RUN cd /tmp/prod && bun install --frozen-lockfile --production

FROM install AS prerelease
COPY --from=install /tmp/dev/node_modules node_modules
COPY . .

ENV NODE_ENV=production

RUN bun --bun run vite build

FROM base AS release

ARG PORT=3000
ENV PORT=$PORT

WORKDIR /minizord

RUN adduser -DH minizord
USER minizord

COPY --from=install /tmp/prod/node_modules node_modules
COPY --from=prerelease /usr/src/minizord/build .

EXPOSE $PORT/tcp

ENTRYPOINT [ "bun", "--bun", "run", "/minizord/index.js" ]
