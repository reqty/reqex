# reqex

## Running
### Hosted
[@reqexbot](https://t.me/reqexbot)

### Docker
```sh
docker run -e TELEGRAM_BOT_TOKEN=xyz ghcr.io/reqty/reqex:0
```

### systemd
friendly reminder not to run hot garbage foreign scripts without looking through 'em.

1. Install rust, openssl dev libraries and the usual build tools.
    - In ubuntu 20.04 lxc container, something along the lines of
      ```
      apt install build-essential pkg-config curl && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
      worked for me.
1. [`install.sh`](install.sh) builds the bot and installs a global systemd service.
