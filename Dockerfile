FROM rust:1.56 as rust-builder
WORKDIR /app
COPY ./backend .
RUN cargo build --release

FROM node:14 as angular-builder
WORKDIR /app
COPY ./frontend .
RUN npm install
RUN npm run build

FROM debian:buster-slim
COPY --from=rust-builder /app/target/release/stream_processor /usr/local/bin/
COPY --from=angular-builder /app/dist/streamdesk /var/www/html
EXPOSE 8080
CMD ["stream_processor"]
