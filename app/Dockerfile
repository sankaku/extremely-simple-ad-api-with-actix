FROM rust:1.50.0 as build-env
WORKDIR /app

# Add necessary files only
ADD ./Cargo.toml /app/Cargo.toml
ADD ./Cargo.lock /app/Cargo.lock
ADD ./src /app/src

RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/extremely-simple-ad-api-with-actix /
CMD ["./extremely-simple-ad-api-with-actix"]