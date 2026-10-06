# stringsolver
a CLI string characters converter written in Rust

Copyright (C) 2026 ghlilith1k, distributed under the GNU/GPLv2 license

# Prelude
**please do not take this program as a serious password/file crypter/decrypter,**
**this program is still very experimental and if you need a real file crypter to**
**protect your data, please don't consider using this as is not very secure for now.**
**this program just show the logic behind string parsing and translation and it's not**
**meant for commercial use, the creator doesn't guarantee anything and absolutely invitates**
**you to use a more serious file crypter to protect your personal data, you have been warned.**

# How to install:
    sudo make install

# About
- current version: v0.0.1 ALPHA

- current stable: none

- AI code: only the Some match arm, line 34

- for future releases: add string scrambling

# Usage
- --help:               prints help message.
- --file=FILE:          file to translate (path).
- --output=term/FILE:   where to paste output (term is to print it in the terminal,
-                       FILE is to paste translated file contents in a newly created file, path).
- --debug:              prints debug info.

# Changelog
(NFY means not finished yet)

## (NFY) release 0.0.1 ALPHA:
- added --help, --file, --output and --debug options.