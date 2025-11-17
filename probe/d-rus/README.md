# d-rus

This probe is an experimental Rust implementation of a simple D-Bus
introspection XML parser. It's a part of my ongoing effort to learn parsing,
tokenization, and how structured APIs can be modeled.

## Motivation

Recently I've been been curious about [D-Bus][1], not just the technology
itself, but how widely it’s used across the Linux ecosystem.

What particularly caught my interest is the introspection XML mechanism.
This feature allows services to declare their entire API surface once, using a
language-neutral XML format.

That single definition can then be used to generate strongly-typed client
bindings across various languages, which I find to be a cool concept for
Inter-Process Communication (IPC). Even kind of reminded me of gRPC and
protobufs.

That curiosity made me want to dive deeper into:

- how D-Bus introspection works,
- how those XML definitions are structured,
- code generation capabilities.

That's where I thought to myself: parsing this well-defined, relatively small
domain by hand could be a cool exercise in *learning how to designing a
tokenizer/lexer*.

*For the full specification, check the [D-Bus Introspection Format][2].*

## Purpose

This is **not** a production parser. Instead, the goals are to:

- **Learn and practice** parsing and tokenizing concepts in Rust
- Work with XML structures in a **low-level, manual** way (instead of relying
  on XML libraries)
- Explore how D-Bus introspection data is structured
- Potentially evolve this experiment into a **code generator** for D-Bus client
  service stubs in the future

## Scope (Current State)

The implementation currently focuses on:

- Parsing and Tokenizing DBus-relevant elements from XML.

Would love to dabble in the idea of even doing some code generation though.

[1]: https://www.freedesktop.org/wiki/Software/dbus/
[2]: https://dbus.freedesktop.org/doc/dbus-specification.html#introspection-format
