FROM rust AS builder

RUN apt-get update -qq; apt-get upgrade -y -qq
RUN apt-get install -qq -y g++-mingw-w64-x86-64 

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --target x86_64-unknown-linux-gnu --release
RUN cargo build --target x86_64-pc-windows-gnu --release


FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/todo /
COPY --from=builder /usr/src/app/target/x86_64-pc-windows-gnu/release/todo.exe /

ENTRYPOINT  ["/todo", "/todo.exe"]