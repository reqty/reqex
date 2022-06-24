# reqex
## Basic usage
Reply to message `bob and foo went to the forest` with `s/fo/bar/g` → bot replies with `bob and baro went to the barrest`.

## Privacy
Incoming requests (`s/fo/bar/g`) are logged to stdout with the username and id of whom sent them. Malicious use of the Telegram bot's token, or modifications in the codebase, naturally enable full access to chats the bot is in.

[Privacy mode](https://core.telegram.org/bots#privacy-mode) is **not** supported, as it'd require all requests to start with `/`. Assuming it isn't the only bot with privacy mode in the chat, a command prefix would be desired as well. As `s/fo/bar/g` is simpler than `/reqex s/fo/bar/g`, the hoster is trusted to known users, and time plus brains is scarce, implementation of the feature is not planned.

## Running
### Hosted
[@reqexbot](https://t.me/reqexbot)

### Docker
```sh
docker build -t reqex https://github.com/reqty/reqex.git
docker run -e TELEGRAM_BOT_TOKEN=xyz reqex
```

### systemd
friendly reminder not to run hot garbage foreign scripts without looking through 'em.

1. Install rust, openssl dev libraries and the usual build tools.
    - In ubuntu 20.04 lxc container, the following worked for me:
      ```
      apt install build-essential pkg-config curl && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
1. [`install.sh`](install.sh) builds the bot and installs a global systemd service.
