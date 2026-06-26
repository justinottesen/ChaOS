# ChaOS

My handmade, from scratch Operating System.

## Building

This repository uses an `xtask` for its build. The config is specified in `chaos-config.toml` by
default, or another path can be provided. Run `cargo xtask help` for more details.

## Documentation

I am going to do my best to document as much of this process as I can alongside the code, maybe it
can serve as a reference to anyone trying to do the same as I am. However, the comments will likely
end up documenting the resulting code, rather than my process to get there. This is because I am
embedding it in the code itself rather than as a separate linear set of markdown documents.

If you are interested in tracing through my process, you can use the git history. The place I am
starting is the [x86_64 BIOS bootloader](./boot/x86_64/bios/stage1.asm).

## Notes

- For now, this is fully intended to be a solo project. I don't plan on accepting contributions.
- I am currently only targeting x86_64.
