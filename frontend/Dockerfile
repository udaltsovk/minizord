FROM oven/bun:1.2.9-alpine AS base
WORKDIR /usr/src/minizord

FROM base AS install

RUN mkdir -p /temp/dev
COPY package.json bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY package.json bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM install AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY . .

ENV NODE_ENV=production

RUN bun test
RUN bun --bun run vite build

FROM base AS release
COPY --from=install /temp/prod/node_modules node_modules
COPY --from=prerelease /usr/src/minizord/build .

ARG PORT=3000

ENV PORT=$PORT

USER bun
EXPOSE $PORT/tcp

ENTRYPOINT [ "bun", "--bun", "run", "index.js" ]
