from rust:1.61.0-alpine as builder


WORKDIR /app

RUN cargo init
COPY Cargo.* /app/
RUN cargo fetch

RUN apk add --no-cache musl-dev
RUN cargo build --release

COPY src /app/src

RUN cargo build --release

from alpine as runtime

COPY --from=builder /app/target/release/pinbot-rs /app/pinbot-rs

CMD ["/app/pinbot-rs"]