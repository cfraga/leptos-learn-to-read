FROM rust:alpine3.18 AS builder
WORKDIR /build

RUN apk update && \
	apk upgrade --no-cache && \
	apk add pkgconfig libressl-dev musl-dev npm

RUN rustup default nightly 
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos

COPY . .

RUN cargo leptos build --release


FROM alpine:3.18 AS runner
WORKDIR /var/www/app

RUN addgroup -S server && \
	adduser -S www-data -G server && \
	chown -R www-data:server /var/www/app

COPY --chown=www-data:server --from=builder /build/target/release/learn-to-read ./learn-to-read
COPY --chown=www-data:server --from=builder /build/wordlist/wordlist-ao-latest.txt ./wordlist/wordlist-ao-latest.txt
COPY --chown=www-data:server --from=builder /build/target/site ./site

USER www-data

ENV LEPTOS_OUTPUT_NAME "learn-to-read"
ENV LEPTOS_SITE_ROOT "/var/www/app/site"
ENV LEPTOS_ENV "PROD"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./learn-to-read"]