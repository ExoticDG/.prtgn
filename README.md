# .prtgn

A Protogen file extension written in Rust. 

![.prtgn logo](https://github.com/ExoticDG/.prtgn/blob/71e6699c3ce09ec64a5feb2ce82113c5c2a69477/prtgn_logo.jpeg)

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

Fedora (rpm) install package : [Cargo-rmp](https://crates.io/crates/cargo-rpm)

Microsoft Windows install package : [Cargo-wix](https://crates.io/crates/cargo-wix)



