FROM ubuntu
MAINTAINER Enguerran Colson <engcolson@gmail.com>

RUN apt-get update && apt-get install -y curl gcc
RUN curl -s https://static.rust-lang.org/rustup.sh | sudo sh

COPY . /src
WORKDIR /src
RUN cargo build
CMD ["/src/target/run"]
