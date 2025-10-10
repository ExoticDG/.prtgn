# .prtgn

A Protogen file extension written in Rust. 

![.prtgn logo](https://github.com/ExoticDG/.prtgn/blob/71e6699c3ce09ec64a5feb2ce82113c5c2a69477/prtgn_logo.jpeg)

> [!CAUTION]
> 
>.prtgn, a protogen inspired file extention writen in Rust.
>Copyright (C) 2025  ExoticDG
>
>This program is free software: you can redistribute it and/or modify
>it under the terms of the GNU General Public License as published by
>the Free Software Foundation, either version 3 of the License, or
>(at your option) any later version.
>
>This program is distributed in the hope that it will be useful,
>but WITHOUT ANY WARRANTY; without even the implied warranty of
>MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
>GNU General Public License for more details.
>
>You should have received a copy of the GNU General Public License
>along with this program.  If not, see <https://www.gnu.org/licenses/>.

## A few other protogen repositories 

[Proto-OS](https://github.com/dimitrivlachos/Proto-OS), "An operating system for a Protogen head which utilises computer vision and machine learning to replicate the user's facial expressions and eye position onto the exterior displays of the character head. Made for an RPI4." -- [dimitrivlachos](https://github.com/dimitrivlachos)

[Proto-Ear-Twitch](https://github.com/stef1949/Proto-Ear-Twitch), "Code for controlling protogen ears" -- [stef1949](https://github.com/stef1949)

[ProtogenHelmet-ESP32](https://github.com/NCPlyn/ProtogenHelmet-ESP32), "Controller & Remote & Animator for Furry Protogen helmet using ESP32-S3 & MAX7219/WS2812" -- [NCPlyn](https://github.com/NCPlyn)

## Plans

- [x] Rataui for a CLI UI for file editing and all sorts of stuff -- https://ratatui.rs/tutorials/json-editor/ \\ https://ratatui.rs \\ https://github.com/rhysd/tui-textarea
- [x] CLI command (prtgn) for doing things. Example, `prtgn new <filename>` or something like that would create a new file and open the file editing UI -- https://rust.code-maven.com/clap-subcommand \\ https://medium.com/coderhack-com/writing-a-cli-tool-in-rust-237d7e6417f6 \\ https://rust-cli.github.io/book/tutorial/index.html
- [ ] Custom 'encryption', similar to that of Hexidecimal or Binary or something but not.
- [ ] Instiation file that just uses cargo to install everyting to PATH and whatnot. Cargo works on Linux and Windows. That might meed to install Rust as well though. so maybe just give in and try to find something that will add the thing to PATH.


## For Thoust dev's

Debian (deb) install package : [Cargo-deb](https://crates.io/crates/cargo-deb)

Fedora (rpm) install package : ~~[Cargo-rmp](https://crates.io/crates/cargo-rpm)~~

Microsoft Windows install package : [Cargo-wix](https://crates.io/crates/cargo-wix)
