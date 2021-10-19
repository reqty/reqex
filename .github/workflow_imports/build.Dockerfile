FROM ghcr.io/jtagcat/docker_mods:openssl_ubuntu_latest

ARG PACKAGE_NAME
COPY $PACKAGE_NAME .
RUN chmod +x $PACKAGE_NAME

# https://github.com/moby/moby/issues/42937
# hardcode PACKAGE_NAME as reqex
CMD ["/bin/sh", "-c", "./reqex"]

