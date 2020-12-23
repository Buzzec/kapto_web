FROM node:12 AS website
COPY . /app
WORKDIR /app/website
RUN npm install
RUN npm run build:prod

FROM rust:1.48
WORKDIR /app
COPY . /app
COPY --from=website /app/website/dist /app/website/dist
RUN cargo install --path .
EXPOSE 3030:3030

CMD kapto_web
