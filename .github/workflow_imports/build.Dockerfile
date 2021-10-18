FROM ghcr.io/jtagcat/docker_mods:openssl_ubuntu_latest

ARG PACKAGE_NAME
COPY $PACKAGE_NAME .
RUN chmod +x $PACKAGE_NAME

# https://github.com/moby/moby/issues/42937
# PACKAGE_NAME can be overwritten by runtime here, but it isn't of concern
CMD ["/bin/sh", "-c", "./$PACKAGE_NAME"]

