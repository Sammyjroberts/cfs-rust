# RustFS OSAL

This directory contains the Rust implementation of NASA's Operating System
Abstraction Layer (OSAL), part of the Core Flight System (cFS).

## What is OSAL?

The Operating System Abstraction Layer provides a consistent API that allows
flight software to run on different operating systems without modification. It
handles tasks such as:

- Task management and scheduling
- Message queues and inter-task communication
- File handling
- Time services
- Hardware interfaces

## Implementation Status

🚧 **Very Early Development** 🚧

This is currently a work in progress with limited functionality. The
implementation priorities are:

1. Establish the basic API structure that mirrors the C OSAL
2. Implement core functionality first (tasks, queues)
3. Add platform-specific implementations
4. Complete coverage of the OSAL API

## Using This Code

This code is not ready for actual use yet! It's being developed as a learning
exercise and is not intended for flight or production environments.

## Design Philosophy

While porting the OSAL to Rust, I'm following these principles:

- Maintain API compatibility with the C version where it makes sense
- Leverage Rust's safety features (ownership, type system, error handling)
- Use idiomatic Rust patterns rather than directly translating C code
- Keep the codebase well-documented to aid learning

## Contributing

Contributions are welcome! Some ways you can help:

- Implementing missing functionality
- Writing tests
- Improving documentation
- Providing feedback on the API design

Don't be afraid to experiment or make suggestions - this is a learning project
for everyone involved.

## License

This project is licensed under the [MIT License](../LICENSE) - see the LICENSE
file for details.
