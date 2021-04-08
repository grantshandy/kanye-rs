# kanye-rs
KAAS (Kanye As A Service)

Prints out a random Kanye quote from the [kanye.rest](https://kanye.rest/) API.

## As A Command Line Tool
Install it:
```
$ cargo install kanye
```

Use it:
```
$ kanye 
Kanye says "One of my favorite of many things about what the Trump hat represents to me is that people can't tell me what to do because I'm black"
```

## As A Library
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