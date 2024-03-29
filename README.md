<p align="center">
  <img src="./assets/logo-72-ppi.png" width="500">
</p>

<p align="center">
  <a href="https://docs.rs/agera/latest/agera/">
    <img src="https://img.shields.io/badge/Rust%20API%20Documentation-gray?link=https%3A%2F%2Fdocs.rs%2Fagera%2Flatest%2Fagera%2F">
  </a>
</p>

# Agera SDK

> **NOTE**&nbsp;&nbsp;&nbsp; Unavailable

Develop rich internet applications robustly through Agera SDK, built in the Rust programming language.

<p align="center">
  <img src="./assets/intro-code.png">
</p>

## Progress

Agera applications use mainly a Node graph, featuring:

- [ ] Graphical experience
  - [ ] Display objects
  - [ ] Controls
  - [ ] Themes
  - [ ] Example theme constants for use when prototyping applications
- [ ] Sound APIs
- [ ] Media APIs
- [ ] Touch APIs
- [ ] Input APIs
- [ ] Networking APIs
  - [ ] HTTP
  - [ ] WebSocket
- [ ] Application
  - `Window`
    - [ ] Border and size configuration
    - [ ] Instantiating a window, which is faillible. It must always fail in web browsers.
- [x] File APIs
- [x] Timer APIs

Other features may be unlisted.

Additional notes:

- Rust: Developers must use the nightly Rust compiler to leverage language features, such as use of declarative macros and `try` blocks.

When getting started with the CLI, creating a fresh project will look like follows:

```sh
agera new com.q.a
```

A fresh application will not require manually adding common minor dependencies such as, say, regular expressions, collection literals and serialization, as Agera re-exports them as APIs.

## License

MIT or Apache 2.0

- [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)