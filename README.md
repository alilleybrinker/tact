# Tact

An isometric, pixel-art tactics game, inspired by *Final Fantasy Tactics
Advance*, built with [Bevy](https://bevy.org).

See [`docs/design.md`](docs/design.md) for the game design document.

## Running

```sh
cargo run
```

On Linux, Bevy needs a few system libraries (ALSA and udev, plus X11 or
Wayland). On Debian/Ubuntu:

```sh
sudo apt install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

### Controls

| Key            | Action      |
| -------------- | ----------- |
| Arrows / WASD  | Move cursor |

## License

[MPL-2.0](LICENSE)
