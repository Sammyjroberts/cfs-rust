# cfs-rust: A Rust Port of NASA's Core Flight System

## What is this?

This is a hobby project to port NASA's Core Flight System (cFS) to the Rust
programming language. I'm doing this as a learning exercise to:

1. Understand flight software architecture and principles
2. Learn Rust in the context of a real-world, complex system
3. Explore how Rust's safety guarantees can benefit space systems

## Current Status

🚧 **Early Development** 🚧

This project is in its very early stages. Don't expect anything flight-ready or
production-quality! I'm starting with the Operating System Abstraction Layer
(OSAL) and will gradually work through other components.

## Project Structure

This is organized as a monorepo with the following components:

- `osal/` - Rust implementation of the Operating System Abstraction Layer
- `cfe/` - (Planned) Rust implementation of the Core Flight Executive
- `psp/` - (Planned) Platform Support Package implementations

## Contributing

Contributions are absolutely welcome! This is a hobby project, but I'd love to
collaborate with others interested in flight software, Rust, or both.

Some ways you can help:

- Code contributions
- Documentation improvements
- Testing on different platforms
- Suggestions and ideas
- Bug reports

## Learning Resources

If you're new to cFS or Rust, here are some helpful resources:

**cFS Resources:**

- [NASA cFS GitHub Repository](https://github.com/nasa/cFS)
- [cFS Documentation](https://github.com/nasa/cFS/tree/main/docs)

**Rust Resources:**

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## Important Disclaimer

This is **not** an official NASA project. It is not yet intended for actual
spacecraft or any critical systems. For now it is a learning project and should
be treated as such.

## License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file
for details.

## Acknowledgments

- NASA for creating and open-sourcing the Core Flight System
- The Rust community for building such an amazing language
- Everyone who contributes to this project in any way
