# ascii-to-dur
A short script to convert an ascii art to a .dur file

Ly Display Manager is still on version 1.4.1-1 in some package managers' repositories.
This old version does not read the new version of .dur files exported by durdraw.
This script converts an ascii txt (an ascii art) into a .dur file version 7.
The output durfile is a single frame, with characters completely white.

## Usage:

```
cargo build --release
./target/release/ascii_to_dur <ascii_filename>
```

This should result in the creation of a `<ascii_filename>.dur` file, or an error.

## TEST IT BEFORE REBOOTING
in version 1.4.1-1 Ly assumes the .dur file is well formed.
If it is not, you might be soft-locked out of tty2 (and for some reason even other ttys)

To test it simply edit the config.ini and run `ly-dm` as you would normally.
