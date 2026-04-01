<div id="top">
    <div align="center">
        <h1>Sho</h1>
        <p>blazingly fast TUI anime browser, written in rust.</p>
        <p>inspired by ani-cli from <a href="https://github.com/pystardust/ani-cli">pystardust</a></p>
        <p>
            <img src="https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white" alt="Rust" />
            <a href="https://www.blazingly.fast">
                <img src="https://www.blazingly.fast/api/badge.svg?repo=Scapy47%2FShio" alt="blazingly fast" />
            </a>
            <a href="https://github.com/Scapy47/Sho/actions/workflows/release.yaml">
                <img src="https://github.com/Scapy47/Sho/actions/workflows/release.yaml/badge.svg" alt="Build and Release" />
            </a>
        </p>
    </div>
</div>

## Screenshots
![looksmaxxing](https://raw.githubusercontent.com/scapy47/Sho/refs/heads/main/assets/edited-6469.jpg)
![looksmaxxing](https://raw.githubusercontent.com/scapy47/Sho/refs/heads/main/assets/edited-6468.jpg)
![looksmaxxing](https://raw.githubusercontent.com/scapy47/Sho/refs/heads/main/assets/edited-6467.jpg)
![looksmaxxing](https://raw.githubusercontent.com/scapy47/Sho/refs/heads/main/assets/edited-6466.jpg)

## Quick Links

- [Features](#Features)
- [Getting started](#Getting-started)

## Features
- Search and Browse through anime
- Vim and Emacs keybindings
- Multi audio support
- Bring Your Own [player](###Setup-Player) (Not Hard-coded)
- Zero Dependency other then libc (and curl maybe)
- Image preview
- Cross Platform
- Blazingly Fast

## Getting Started

### Setup Player

Sho uses the `SHO_PLAYER_CMD` environment variable to launch your media player. Set it to your player of choice — **mpv** and **VLC** are recommended.

**mpv**
```sh
export SHO_PLAYER_CMD="mpv --user-agent='{user_agent}' --http-header-fields='Referer: {referer}' '{url}'"
```

**VLC**
```sh
export SHO_PLAYER_CMD="vlc --http-user-agent='{user_agent}' --http-referrer='{referer}' '{url}'"
```

> [!NOTE]
> `{url}`, `{user_agent}` and `{referer}` are placeholder for values populated by sho.
> 
> `{url}` is url of video, while `{user_agent}` and `{referer}` are headers required for some sources/providers to work.


### Installation

```sh
cargo install sho
```
