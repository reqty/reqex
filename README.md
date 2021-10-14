# reqex

## Running
```sh
docker run -e TELEGRAM_BOT_TOKEN=xyz ghcr.io/reqty/reqex:1
```

## Installing
friendly reminder not to run foreign scripts without looking through 'em.

Install rust, openssl dev libraries and usual build tools


In ubuntu 20.04 lxc container, something along the lines of
```
apt install build-essential pkg-config curl && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
worked for me.

`install.sh` builds the bot and installs a global systemd service.
