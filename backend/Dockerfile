FROM rust:latest

RUN apt update

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y -q \
  ca-certificates \
  locales \
  apt-transport-https\
  libssl-dev \
  libpq-dev \
  pkg-config \
  curl \
  build-essential \
  libdbus-1-dev \
  libsqlite3-dev \
  mariadb-client \
  git \
  wget \
  && locale-gen \
  && echo "install rust tools" \
  && rustup component add rustfmt \
  && cargo install cargo-watch cargo-make

RUN apt update

RUN mkdir -p /myapp/backend

WORKDIR /myapp/backend

COPY Cargo.toml /myapp/backend/Cargo.toml
COPY Cargo.lock /myapp/backend/Cargo.lock

COPY . /myapp/backend/