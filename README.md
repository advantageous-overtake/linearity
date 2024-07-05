# Introduction
`linearity` is a constant-accelerated crate for various operations commonly found in branchless programming.

The primary trait is [`Linearity`], it provides facilities for clear and concise branchless programming.
 
See the totality of the [crate documentation](https://docs.rs/linearity) for more information.

**NOTE:** This library is currently nightly-only, due to the usage of const-traits and some currently unstable features.
**WARNING:** The current CI failure is due to a toolchain bug.

## Usage
To use `linearity` in your project, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
linearity = "1"
```

Then, in your Rust code, you can import and use the crate as follows:

```rust
use linearity::Linearity;
```

For detailed usage examples and API documentation, please refer to the [crate documentation](https://docs.rs/linearity).

## Contributing
Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/advantageous-overtake/linearity).

## License
This crate is distributed under the terms of the AGPLV3 license. See `LICENSE` for more information.

## Acknowledgements
We would like to thank all the contributors to the `linearity` crate for their valuable contributions and feedback.
