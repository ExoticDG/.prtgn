# .prtgn

A Protogen file extension written in Rust. 

![.prtgn logo](https://github.com/ExoticDG/.prtgn/blob/71e6699c3ce09ec64a5feb2ce82113c5c2a69477/prtgn_logo.jpeg)

> [!CAUTION]
> 
>.prtgn, a protogen inspired file extension written in Rust.
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

## What we have to offer

Welcome to .prtgn! The all new CLI file format for Protogens. Powered by Rust and Ratatui!

We offer 'secure' files only select programs have the ability to understand, so your RAM cash and USB serial is safe with us!

> [!CAUTION]
> **PRTGN IS A SECURITY THROUGH OBSCURITY APPLICATION**
>
> The ONLY security on PRTGN is obscuring the data in files. **IT IS NOT ENCRYPTED**
>
> PRTGN is NOT meant as a file type for critical information and therefor the security is treated as such.
> We will take security issues and treat them as problems, though ultimately PRTGN shouldn't be relied on for security. 
>
> PRTGN is NOT at fault for any information that gets seen through the use of our files

Along with that we have a wonderful Protogen friendly CLI interface, and a built in translation for Human or other species's to understand.  

Have a look at our wonderful command structure! Start out with 'prtgn' and then apply any of the following sub commands!

>[!TIP]
>Check out out [wiki](https://github.com/ExoticDG/.prtgn/wiki) for the different commands!

## A few other protogen repositories 

[Proto-OS](https://github.com/dimitrivlachos/Proto-OS), "An operating system for a Protogen head which utilizes computer vision and machine learning to replicate the user's facial expressions and eye position onto the exterior displays of the character head. Made for an RPI4." -- [dimitrivlachos](https://github.com/dimitrivlachos)

[Proto-Ear-Twitch](https://github.com/stef1949/Proto-Ear-Twitch), "Code for controlling protogen ears" -- [stef1949](https://github.com/stef1949)

[ProtogenHelmet-ESP32](https://github.com/NCPlyn/ProtogenHelmet-ESP32), "Controller & Remote & Animator for Furry Protogen helmet using ESP32-S3 & MAX7219/WS2812" -- [NCPlyn](https://github.com/NCPlyn)

## Plans

- [x] Rataui for a CLI UI for file editing and all sorts of stuff -- https://ratatui.rs/tutorials/json-editor/ \\ https://ratatui.rs \\ https://github.com/rhysd/tui-textarea
- [x] CLI command (prtgn) for doing things. Example, `prtgn new <filename>` or something like that would create a new file and open the file editing UI -- https://rust.code-maven.com/clap-subcommand \\ https://medium.com/coderhack-com/writing-a-cli-tool-in-rust-237d7e6417f6 \\ https://rust-cli.github.io/book/tutorial/index.html
- [x] 'Security' Through Obscurity
- [x] Fedora / Rocky / RHL Support
- [ ] ARM support
- [ ] More than text in the files. I.E. Making it able to do more stuff. Maybe images or a wrapper for Rust or something.
- [x] Automatically adding .prtgn to a filename in the init command
- [ ] Benchmark/test sub command \\ usage stats and whatnot \\ https://github.com/sharkdp/hyperfine ?
- [ ] *File format converter*
- [ ] **MIDI / other musics** | Inspired by [Ivycomb](https://youtube.com/@ivycomb?si=hL9f19mSvyffFUk1) - [YTShort](https://youtube.com/shorts/dQyZ-WTuBwQ?si=PoWy2zuMMxrF3mQX) / [Ivycomb Music](https://youtube.com/@ivycombmusic?si=K92ak8535oQ7ik8r) - [YTMusic](https://music.youtube.com/watch?v=J620cBDOrj4&si=S0GaU3D3IH-71s0k)
- [ ] Website

## For Thoust dev's

Debian (deb) install package : [Cargo-deb](https://crates.io/crates/cargo-deb)

Fedora (rpm) install package : ~~[Cargo-rmp](https://crates.io/crates/cargo-rpm)~~ ~~[Cargo-generate-rpm](https://crates.io/crates/cargo-generate-rpm)~~ | I was unable to get either of these to work.

Microsoft Windows install package : [Inno Setup](https://jrsoftware.org/isinfo.php) | ~~win_install.bat~~ **NOT SAFE** | [Cargo-wix](https://crates.io/crates/cargo-wix) is hard :(

INNO Registry PATH : `Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"`
