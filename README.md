# Unconverter

A BL2 modding utility. Converts from a patch containing hotfixes to individual hotfixes for use in the converter utility.

Note that it is rough around the edges and built to exactly what I needed at the time, it might not fulfill everyone's needs, but it should allow people to get started working on hotfixes. You should definitely not use this as an example of good coding practice in Rust, I spent very little time architecting this and just built what worked.

## Compilation

    cargo build

This project was built for Rust version 1.17 Beta in April 2017. While this is the version used to build the project, earlier and later versions will likely work fine. (No promises)

To install Rust, go to https://www.rust-lang.org/en-US/

## Usage

    cargo run -- <options>

Run through cargo or the executable itself.

Command line options:

-f=\<file1\> : patch file

The hotfixes are output to stdout, to output to a file use shell stdout redirection.

Example:

    cargo run -- -f=./hotfixes_set_notation.txt > hotfixes.txt

## Examples

Given are the original Gearbox hotfixes as of April 7th 2017, in both the original set format and the patch format output from this project.

## Support

This project is unsupported, use at your own risk.

## License

This project is licensed under the Apache V2, included in LICENSE-2.0.txt.
