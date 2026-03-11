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

### Slices

The entire system will be built using slices (fat pointers). No null-terminated
strings (hooray!). All pointers will have an associated length. This should help
simplify interfaces and reduce the likelihood of bugs. This is the main design
decision that will differentiate this OS from the rest.

## Development Log

I have a bad habit of hopping from project to project and never really making
progress on any one of them. To try and combat that, I am going to document
my progress in the `DEVELOPMENT_LOG.md` document. These are really more personal notes that
I am making public, rather than an organized and polished document.

## Resources

| Name                                             | URL                       |
|--------------------------------------------------|---------------------------|
| Writing a Simple Operating System - From Scratch | https://github.com/tpn/pdfs/blob/04d7ca63261822510fc5aa282a2079db99b8cf6e/Writing%20a%20Simple%20Operating%20System%20from%20Scratch%20-%20Nick%20Blundell%20-%20Dec%202010.pdf |
