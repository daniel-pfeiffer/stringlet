# 🧵 Stringlet

A fast, cheap, compile-time constructible, `Copy`-able, kinda primitive inline string type. When storing these on the
stack, you probably want to use smaller sizes, hence the name. No dependencies are planned, except for optional SerDe
support, etc. The intention is to be no-std and no-alloc – which still requires feature-gating `String` interop.

<div class="warning">
This is an <b>alpha release</b>. Using it as <code>str</code> should mostly work. The design is prepared for
<code>String</code> functionality, but that needs to be implemented.
</div>

In my casual benchmarking it beats all other string kinds and crates nicely to spectacularly on some tests. There are
four flavors of mostly the same code. They differ in length handling, which shows only in some operations, like
`len()`, `as_ref()`, and `as_str()`:

- **[`Stringlet`](https://docs.rs/stringlet/latest/stringlet/type.Stringlet.html),
  [`stringlet!(…)`](https://docs.rs/stringlet/latest/stringlet/macro.stringlet.html)**: This is fixed size, i.e. bounds
  for array access are compiled in, hence fast.

- **[`VarStringlet`](https://docs.rs/stringlet/latest/stringlet/type.VarStringlet.html), `stringlet!(var …)`,
  `stringlet!(v …)`**: This adds one byte for the length – still pretty fast. Speed differs for some content processing,
  where SIMD gives an advantage for multiples of some power of 2, e.g.  `VarStringlet<32>`. While for copying the
  advantage can be at one less, e.g. `VarStringlet<31>`. Length must be `0..=255`.

- **[`TrimStringlet`](https://docs.rs/stringlet/latest/stringlet/type.TrimStringlet.html), `stringlet!(trim …)`,
  `stringlet!(t …)`**: This can optionally trim one last byte, useful for codes with minimal length variation like
  [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging an unused last byte with a UTF-8
  niche. The length gets calculated branchlessly with very few ops.

- **[`SlimStringlet`](https://docs.rs/stringlet/latest/stringlet/type.SlimStringlet.html), `stringlet!(slim …)`,
  `stringlet!(s …)`**: This uses the same UTF-8 niche, but fully: It projects the length into 6 bits of the last byte,
  when content is less than full size. Length must be `0..=64`. Though it is done branchlessly, there are a few more ops
  for length calculation. Hence this is the slowest, albeit by a small margin. Any bit hackers, who know how to do with
  less ops, welcome on board!

N.B.: Variable size `VarStringlet` seems a competitor to [`fixedstr::str`](https://crates.io/crates/fixedstr),
[`arrayvec::ArrayString`](https://crates.io/crates/arrayvec), and the semi-official
[`heapless::String`](https://docs.rs/heapless/latest/heapless/string/type.String.html). They lack a `heapless::Str`, to
match the faster fixed size `Stringlet`. That would be given by
[`fixedstr::zstr`](https://docs.rs/fixedstr/latest/fixedstr/struct.zstr.html) but their equality checks are not
optimized. I hope it can be independently confirmed (or debunked, if I mismeasured) that for tasks like `== Self` or `==
&str` all variants in this crate seem by a factor faster than competitors.

```rust
# extern crate stringlet;
use stringlet::{Stringlet, VarStringlet, TrimStringlet, SlimStringlet, stringlet};

let a: VarStringlet<10> = "shorter".into(); // override default stringlet size of 16 and don’t use all of it
let b = a;
println!("{a} == {b}? {}", a == b);      // No “value borrowed here after move” error 😇

let nothing = Stringlet::<0>::new();     // Empty and zero size
let nil = VarStringlet::<5>::new();      // Empty and size 5 – would be impossible for fixed size Stringlet
let nada = TrimStringlet::<1>::new();    // Empty and size 1 – biggest an empty TrimStringlet can be

let x = stringlet!("Hello Rust!");       // Stringlet<11>
let y = stringlet!(v 14: "Hello Rust!"); // abbreviated VarStringlet<14>, more than length
let z = stringlet!(slim: "Hello Rust!"); // SlimStringlet<11>
let Ψ = stringlet!(v: ["abcd", "abc", "ab"]); // VarStringlet<4> for each
let ω = stringlet!(["abc", "def", "ghj"]); // Stringlet<3> for each

const HELLO: Stringlet<11> = stringlet!("Hello Rust!"); // Input length must match type
const PET: [Stringlet<3>; 4] = stringlet!(["cat", "dog", "ham", "pig"]); // size of 1st element
const PETS: [VarStringlet<8>; 4] = stringlet!(_: ["cat", "dog", "hamster", "piglet"]); // _: derive type
```

But

```text
error[E0599]: `SlimStringlet<99>` has excessive SIZE
  --> src/main.rs:99:16
   |
99 | let balloons = stringlet!(s 99: "Luftballons, auf ihrem…");
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ SIZE must be `0..=64`
   …
   = note: the following trait bounds were not satisfied:
           `stringlet::StringletBase<stringlet::Slim, 99>: stringlet::SlimConfig<99>`
           which is required by `stringlet::StringletBase<stringlet::Slim, 99>: ConfigBase<stringlet::Slim, 99>`
   = note: `SlimStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!
```

This is **not** your classical short string optimization (SSO) in so far as there is no overflow into an alternate
bigger storage. This is by design, as addressing two different data routes requires branching. On your hot path,
branch misprediction can be costly. Crate stringlet tries hard to be maximally branchless. The few `if`s and `||`s
refer to constants, so should be optimized away.

There is no `Option` or `Result` niche optimization yet. That should likewise be feasible for all stringlets with a
stored length. I only need to understand how to tell the compiler?

`VarStringlet` and `SlimStringlet` are configured so they can only be instantiated with valid sizes. For normal use
that’s all there is to it. However when forwarding generic arguments to them you too have to bound by
`VarConfig<SIZE>` or `SlimConfig<SIZE>`. I wish I could just hide it all behind `<const SIZE: usize<0..=64>>`!

## Todo

- [ ] `StringletError` & `stringlet::Result`

- [ ] Run Miri on various architectures. Who’s willing to support with exotic stuff?

- [ ] Run `cargo mutants`

- [ ] Implement mutability, `+=`, `write!()`.

- [ ] Document!

- [ ] How to implement `Cow` / `Borrow` with `String` as owned type?

- [ ] Or rather a `Cow`-like storage-constrained/limitless pair that will transparently switch on overflow.

- [ ] Implement more traits.

- [ ] `format!()` equivalent `stringlet!(format …)` or `format_stringlet!()`

- [ ] Integrate into [string-rosetta-rs](https://github.com/rosetta-rs/string-rosetta-rs)

- [ ] Implement for popular 3rd party crates.

- [ ] Why does this not pick up the default SIZE of 16: `let fail = Stringlet::new();`

- [ ] Is there a downside to `Copy` by default?

- [ ] What’s our minimal rust-version?

<!-- do not format the table body! -->

*Platt* (Low German)	|Semi-literal Translation
:---|:---
*Op de Straat löppt’n Jung mit’n **Tüddelband**<br>in’ne anner Hand’n Bodderbrood mit Kees,<br>wenn he blots ni mit de Been in’n Tüddel keem<br>un dor liggt he ok all lang op de Nees<br>un he rasselt mit’n Dassel op’n Kantsteen<br>un he bitt sick ganz geheurig op de Tung,<br>as he opsteiht, seggt he: Hett ni weeh doon,<br>dat’s’n Klacks för so’n Kieler Jung*|Upon the street runs a boy with a **Twiddle-String**<br>in another hand a buttered bread with cheese,<br>if he only not with the legs into the twiddle came<br>and there lies he already long upon the nose<br>and he rattles with the noggin upon a kerbstone<br>and he bites himself greatly upon the tongue,<br>as he up stands, says he: Has not hurt,<br>that’s a trifle for such a boy from Kiel

Dedicated to my father, who taught me this iconic Northgerman [▶ String song](https://youtu.be/ByYTEReqf4Q?list=RDByYTEReqf4Q&t=38). The many similarities nicely illustrate how English (Anglo-Saxon) comes from Northgermany. There are even more cognates that have somewhat shifted in usage: *löppt:* elopes (runs) – *Jung:* young (boy) – *Been:* bones (legs) – *weeh doon:* woe done (has hurt) – and maybe *Kant:* cant (tilt on edge)
