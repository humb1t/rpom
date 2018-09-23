FROM alpine:latest
VOLUME /tmp
EXPOSE 8000
ADD target/release/rpom rpom
ENTRYPOINT ./rpom
