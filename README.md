# Music command line utility

An installable command line application for managing the Apple Music app written in Rust.

NOTE: the current state of the project is "proof of concept". Think of it as a simplest implementation of a utility to control Apple Music. It may or may not be improved in the future. Your contribution to the project is welcome.

## Requirements

- MacOs operating system system (tested with 12.6.3 version)
- Apple Music app (tested with 1.2.5.7 version)
- Any modern command line terminal

## Installation

    $ cargo install music-cli

## Usage

Available commands:

    $ music-cli play

    $ music-cli pause

    $ music-cli next

    $ music-cli previous

## Inspired by

- [zshMusic](https://github.com/codingMustache/zshMusic)
- [DylanCa/apple-music](https://github.com/codingMustache/zshMusic)
- MacOs ability to execute scripts with [osascript](https://ss64.com/mac/osascript.html)
