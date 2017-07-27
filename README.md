# Panic! At The Disco

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
panic_at_the_disco = "*"
```

Add this to your lib ...

```rust
extern crate panic_at_the_disco;
```

Then initialise it by calling:

```rust
panic_at_the_disco::init();
```

Then panic in style:

```rust
panic!("at the disco");
```

## License

Creative Chorus 0
