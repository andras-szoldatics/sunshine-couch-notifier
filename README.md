# sunshine-couch-notifier

A small daemon that watches logs and triggers desktop notifications when relevant events are detected.

This project is currently a proof of concept. It can only handle a session-based `journalctl` dependent workflow, such as sunshine running as a `systemd --user` service.

## install

From the project root:

```bash
cargo build --release
install -Dm755 target/release/sunshine-couch-notifier ~/.local/bin/sunshine-couch-notifier
```

You can test it manually:

```bash
~/.local/bin/sunshine-couch-notifier journalctl
```

## daemon setup (`systemd --user`)

1. copy the service file:

```bash
install -Dm644 service/sunshine-couch-notifier.service ~/.config/systemd/user/sunshine-couch-notifier.service
```

2. reload user units and enable the daemon:

```bash
systemctl --user daemon-reload
systemctl --user enable --now sunshine-couch-notifier.service
```

3. check status and logs:

```bash
systemctl --user status sunshine-couch-notifier.service
journalctl --user -u sunshine-couch-notifier.service -f
```

## license

This project is licensed under the MIT license. you are free to use, modify, and distribute it as you like, as long as you include the original license file.

For full details, see [[LICENSE](./LICENSE)].
