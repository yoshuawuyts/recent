<h1 align="center">recent</h1>
<div align="center">
  <strong>
    Get the recent issues and pull-requests on github
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/recent">
    <img src="https://img.shields.io/crates/v/recent.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/recent">
    <img src="https://img.shields.io/crates/d/recent.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/recent">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/recent">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/recent/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/recent/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Installation
```sh
$ cargo add recent
```

## Deployment

Instructions to deploy to the remote server.
```sh
cargo build --release
scp -i ~/.ssh/config target/release/recent recent:~/

# on the server
nohup recent &
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/recent/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/recent/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/recent/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
