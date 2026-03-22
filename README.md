# ChaOS

I am building an operating system. I have no idea what I am doing.

I highly doubt anyone would find this useful either as a guide or to use as an
OS, but I am making it public so I am held accountable to actually work on it.

## Design Principles

### Learning Experience

The goal of this project is for me to learn and experiment. Not to make the next
Linux. This means that for the scope of this project, I will always value
something that I have written myself over copy-pasting from an existing source.
This will also motivate some of my design decisions, which I expect to cause me
to run into some unique issues.

### Rust

As a challenge, I will also be writing this OS entirely in Rust. As of starting
this project, I have never used Rust, but I figure it will mean I am forging my
own path, and hopefully should make this unique.

### Slices 

The entire system will be built using slices (fat pointers). No null-terminated
strings (hooray!). All pointers will have an associated length. This should help
simplify interfaces and reduce the likelihood of bugs. This is the main design
decision that will differentiate this OS from the rest.
