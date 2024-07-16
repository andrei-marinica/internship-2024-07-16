# MultiversX course for interns: Rust basics

I wanted to focus on the topics that I found most challenging while learning Rust.

For all the others (and a good recap as well), please read [The Rust Book](https://doc.rust-lang.org/book/title-page.html), it's not very long.

We talked about:

## Rust crate structure

- `src` folder
- `Cargo.toml`
- How to import libraries (`cargo add ...` is a nice trick)
- Semantic Versioning (semver). Very important topic for any programmer & any language. Read more here: https://semver.org/

## Rust modules

- `mod` keyword
- `.rs` files are always modules
- Encapsulation in modules: `pub`, `pub(crate)`, `pub(super)`.
- `use` keyword, also `super` and `crate` modules.

## Functions, basic operators and types

We wrote a factorial function on u64.

We also explored mutability, and `let` vs. `let mut`.

## Ownership

I used the `BigUint` type to demonstrate how ownership and lifetimes work in Rust, on a basic level.

We talked about owned types, immutable references, mutable references.

Contrast:
- values: `x`, `&x`, `&mut x`
- types: `X`, `&X`, `&mut X`

The main rules:
    - variable becomes unusable once ownership is lost `let y = x`
    - only allowed to coexist at the same time:
        - ONE mutable reference, OR
        - any number immutable references

To understand it better, we expanded this:
```
    print_big_uint(&x);
    inc_big_uint(&mut x);
    print_big_uint(&x);
```

... into this:
```
    {
        // 'a
        let x_a: &BigUint = &x;
        print_big_uint(x_a);
        print_big_uint(&x);
        print_big_uint(x_a);
    }

    {
        // 'b
        let x_b = &mut x;
        inc_big_uint(x_b);
    }

    {
        // 'c
        print_big_uint(&x);
    }
```

## Structs

We created struct `FactorialComputation`, a constructor, some methods, discussed the special argument `self`.

Methods (have `self`) vs. associated functions (they don't).

## Traits

We discovered:
- `Default`. Implemented manually, instead of `#[derive(Default)]`.
- `From`, got the first taste of generics.

`.into()` is always the opposite of `from(...)`, and is implemented automatically.

## Enums

Simple, familiar enum:

```
pub enum DeJucarie {
    Egld,
    Achievement,
    Usdc,
}
```

The more powerful "discriminated union". 

```
pub enum DeJucarie {
    Egld(u32),
    Achievement(String),
    Usdc,
}
```

How to `match` and enum.

The `Option` enum.

We added an optional result to `FactorialComputation`.


## Further reading

[The Rust Book](https://doc.rust-lang.org/book/title-page.html), especially chapters 3, 4, 5, 6, 7, 8, 10, 11, 12.

Good luck to everyone!
