# 🧵 Stringlet

A fast, cheap, compile-time constructible, `Copy`-able, kinda primitive inline string type. Stringlet length is limited
to 64. *Though the longer your stringlets, the less you should be moving and copying them!*  No dependencies are
planned, except for optional SerDe support, etc. The intention is to be no-std and no-alloc. This might yet require
feature-gating `String` interop?

<div class="warning">
This is an <b>alpha release</b>. Using it as <code>str</code> should mostly work. The design is prepared for
<code>String</code> functionality, but that needs to be implemented.
</div>

In my casual benchmarking it beats all other string kinds and crates nicely on some tests. The fixed sized variant is
otherwise on par with the competition. However for the variable sized variant, `as_str()` is noticeably slower than
others. The necessary length calculation is branchless, but I’m still racking my brain how to do it with less ops. Any
bit hackers, welcome on board!


```rust
# extern crate stringlet;
use stringlet::{FixedStringlet, Stringlet, stringlet};

let a: Stringlet<10> = "shorter".into(); // override default Stringlet size of 16 and don’t use all of it
let b = a;
println!("{a} == {b}? {}", a == b);      // No “value borrowed here after move” error 😇

let nothing = Stringlet::<0>::new();     // Empty and zero size
let nil = Stringlet::<5>::from_str("");  // Empty and size 5

let x = stringlet!("Hello Rust!");       // Stringlet<11>
let y = stringlet!(14: "Hello Rust!");   // Stringlet<14>, more than length
let z = stringlet!(="Hello Rust!");      // FixedStringlet<11>, colon optional here
let Ψ = stringlet!(["abcd", "abc", "ab"]); // Stringlet<4> for each, colon optional here
let ω = stringlet!(=["abc", "def", "ghj"]); // FixedStringlet<3> for each, colon optional here

const HELLO: Stringlet = stringlet!(_: "Hello Rust!"); // derived default size of Stringlet<16>
const PETS: [Stringlet<8>; 4] = stringlet!(_: ["cat", "dog", "hamster", "piglet"]); // derive size for all
const PE: [FixedStringlet<2>; 4] = stringlet!(_: ["ca", "do", "ha", "pi"]); // derive size and fixed for all
```

But

```text
error[E0277]: `Stringlet<99>` has excessive SIZE
  --> src/main.rs:99:16
   |
99 | let balloons = stringlet!(99: "Luftballons, auf ihrem…");
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ SIZE must be `0..=64`
   |
   = help: the trait `Config<99>` is not implemented for `Stringlet<99>`
   = note: `Stringlet` cannot be longer than 64 bytes. Consider using `String`!
   = note: Also ALIGN is 1. This must be one of 1, 2, 4, 8, 16, 32, or 64!
```

This is **not** your classical short string optimization (SSO) in so far as there is no overflow into an alternate
bigger storage. This is by design, as addressing two different data routes requires branching. On your hot path, branch
misprediction can be costly. Stringlet tries hard to be maximally branchless. The few `if`s and `||`s refer to
constants, so should be optimized away.

The length is tucked in through a no-extra-space branchless UTF-8 hack, when shorter than size. There is no `Option` or
`Result` niche optimization yet. But, that should likewise be feasible for all stringlets shorter than physical size. I
only need to understand how to tell the compiler?

`Stringlet` is configured so can only be instantiated with valid size. For normal use that’s all there is to it. However
when forwarding generic arguments to `Stringlet` you too have to specify `Config`. I wish I could just hide it all
behind `<const SIZE: usize<0..=64>>`! Since we have this anyway, it can also apply alignments up to 64 to each instance
where you may need this. This is aliased to names like `Stringlet4` or `FixedStringlet8`.

## Todo

- [ ] `StringletError` & `stringlet::Result`

- [ ] Run Miri on various architectures. Who’s willing to support with exotic stuff?

- [ ] Run `cargo mutants`

- [ ] Implement mutability, `+=`, `write!()`.

- [ ] Document!

- [ ] How to implement `Cow` / `Borrow` with `String` as owned type?

- [ ] Or rather a `Cow`-like storage-constrained/limitless pair that will transparently switch on overflow.

- [ ] Implement more traits.

- [ ] Add a macro syntax for align.

- [ ] `format!()` equivalent `format_stringlet!()`

- [ ] Integrate into [string-rosetta-rs](rosetta-rs/string-rosetta-rs)

- [ ] Implement for popular 3rd party crates.

- [ ] Why does this not pick up the default SIZE of 16: `let fail = Stringlet::new();`

- [ ] What’s our minimal rust-version?
