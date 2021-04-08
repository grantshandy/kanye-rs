# kanye-rs
KAAS (Kanye As A Service)

Prints out a random Kanye quote from the [kanye.rest](https://kanye.rest/) API.

```rust
fn main() {
    let quote = kanye::quote().unwrap();

    println!("Kanye says \"{}\"", quote);
}
```

Output:
```
Kanye says "We used to diss Michael Jackson the media made us call him crazy ... then they killed him"
```