# 🧵 Stringlet

A fast, cheap, compile-time constructible, `Copy`-able, kinda primitive inline string type. When storing these on the
stack, you probably want to use smaller sizes, hence the name. No dependencies are planned, except for optional SerDe
support, etc. The intention is to be no-std and no-alloc – which still requires feature-gating `String` interop.

<div class="warning">
This is an <b>alpha release</b>, but well tested by <code>mutants</code> and <code>miri</code>. Using it as an inline
<code>str</code> works. The design is prepared for mutating functionality like <code>String</code>, but that needs to be
implemented.<br/><br/>

As it came first, the fixed kind is simply called <code>Stringlet</code>, whereas all other kinds have it in their
name, e.g. <code>VarStringlet</code>. This asymetry can be confusing, both when reading code and when talking about
various kinds. Therefore I’m considering renaming it to <code>FixedStringlet</code>. This would free the old name to
become an enum unifying all kinds into one type. That would make it easy to mix and pass around all kinds. Happy to hear
your better name suggestions!
</div>

In my casual benchmarking it beats all other string kinds and crates nicely, or even spectacularly on some tests. There
are four kinds sharing mostly the same code. They differ in length handling, which gives different trade-offs in only
some operations, like `len()`, `as_ref()`, and `as_str()`:

- **[`Stringlet`](https://docs.rs/stringlet/latest/stringlet/type.Stringlet.html),
  [`stringlet!(…)`](https://docs.rs/stringlet/latest/stringlet/macro.stringlet.html)**: This is fixed size, i.e. bounds
  for array access are compiled in, hence fast.

- **[`VarStringlet`](https://docs.rs/stringlet/latest/stringlet/type.VarStringlet.html), `stringlet!(var …)`,
  `stringlet!(v …)`**: This adds one byte for the length – still pretty fast.  Speed differs for some content processing,
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

> *Equality is fast, as it prefers testing the full arrays, as far as possible. Hard coding that beats determining the
> length and checking the bounds at run time. This can mean checking too much, but that’s ok, as per the name this crate
> is for shorter strings. It works because, because shorter stringlets are padded in such a way that they can only match
> the same padding.*

> *Sadly this shortcut isn’t possible for comparison of non-fixed stringlets: a size 2* `"a"`*, even if NUL padded,
> would not be less than valid string* `"a\0"`*, without also checking the length. And that can’t be done branchlessly.
> So in many cases we must compare dynamic slices.*

```rust
# extern crate stringlet;
use stringlet::prelude::*;

let a: VarStringlet<10> = "shorter".try_into()?; // override default stringlet size of 16 and don’t use all of it
let b = a;
println!("{a} == {b}? {}", a == b);      // No “value borrowed here after move” error 😇

let nothing = Stringlet::<0>::new();     // Empty and zero size
let nil = VarStringlet::<5>::new();      // Empty and size 5 – would be impossible for fixed size Stringlet
let nada = TrimStringlet::<1>::new();    // Empty and size 1 – biggest an empty TrimStringlet can be

let x = stringlet!("Hello Rust!");       // Stringlet<11> (len of parameter)
let y = stringlet!(v 14: "Hello Rust!"); // abbreviated VarStringlet<14>, more than length
let z = stringlet!(slim: "Hello Rust!"); // SlimStringlet<11> (len of parameter)
let Ψ = stringlet!(v: ["abcd", "abc", "ab"]); // VarStringlet<4> (len of 1st parameter) for each
let ω = stringlet!(["abc", "def", "ghj"]); // Stringlet<3> (len of 1st parameter) for each

const HELLO: Stringlet<11> = stringlet!("Hello Rust!"); // Input length must match type
const PET: [Stringlet<3>; 4] = stringlet!(["cat", "dog", "ham", "pig"]); // size of 1st element
const PETS: [VarStringlet<8>; 4] = stringlet!(_: ["cat", "dog", "hamster", "piglet"]); // _: derive type
# stringlet::Result::Ok(())
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

`VarStringlet` and `SlimStringlet` are configured so they can only be instantiated with valid sizes. For normal use
that’s all there is to it. However when forwarding generic arguments to them you too have to bound by
`stringlet::VarConfig<SIZE>` or `stringlet::SlimConfig<SIZE>`. I wish I could just use `<const SIZE: usize<0..=64>>`!

Summarized nicely on [DeepWiki](https://deepwiki.com/daniel-pfeiffer/stringlet).

## Tradeoffs

On a 64 bit host `Box<str>` has an in-place 8 byte overhead, `&str` costs 16 extra bytes and `String` even 24
bytes. *(IMHO the latter should store its capacity on the heap, reducing the move cost by 33%. Make it a fat pointer
transmutable to `&str`, with capacity preplaced at `pointer - 8`.)* Compared to this, only `VarStringlet` has any
overhead, and only 1 byte. For accessing those standard string types, you always have an indirection (pointer
dereference behind the scenes,) whereas stringlets are immediately there.

The downside is that, while the standard strings copy or move only the above mentioned overhead, stringlets must copy or
move the whole content. What is an advantage for smaller stringlets, becomes more expensive the bigger they get. If you
return them up the stack, and until `super let` lands, each stack frame needs to have space for the full size. Each
stringlet always occupies its full size, no matter how much actual length content you store in it. To alleviate that,
all kinds and sizes of stringlet are maximally interoperable. So do choose the optimal kind and smallest possible for
each use case!

This is **not** your classical short string optimization (SSO) in so far as there is no overflow into an alternate
bigger storage. This is by design, as addressing two different data routes requires branching. On your hot path,
branch misprediction can be costly. Crate stringlet tries hard to be maximally branchless. The few `if`s and `||`s
refer to constants, and are thus optimized away.

`SlimStringlet` and `TrimStringlet` use an invalid last byte UTF-8 hack, for coding the length. The same trick could be
applied to the first byte for an `Option` and possibly `Result` niche optimization. According to the UTF-8 standard no
byte can be `0b1111_1xxx`. That gives eight possible niche values. But the compiler can’t know this. And it doesn’t yet
seem to offer a way of expressing such a niche explicitly.

## Input Validation

There are two aspects to this, checking for well-formed UTF-8, and that the string fits into the receiving stringlet.
While the Rust compiler knows both things about a `"literal str"`, alas it chooses to not tell the type-system the size.
Whereas, if you give a `b"byte slice"` instead, the size is available, but UTF-8 is not checked. You can’t get both.
Therefore normal Rust code can’t validate both at compile time. You can choose either constructor
[`from_str`](https://docs.rs/stringlet/latest/stringlet/struct.StringletBase.html#method.from_str) or
[`from_utf8_slice`](https://docs.rs/stringlet/0.8.0/stringlet/struct.StringletBase.html#method.from_utf8_slice).  Each
will check the other property only once it runs.

*Dreaming:* Even if the compiler were to mark this literal with a size hint like `&str<11>`, that would still not be
good enough. Because most stringlet kinds also accept shorter strings, we would need a way to denote this flexibility in
the type system. The constructor signature in e.g. `TrimStringlet<SIZE>`, which can be one byte shorter, would need to
be `from_str(str: &str<SIZE-1..=SIZE>)`. One possibility for this would require two separate entry points to the
function. Glue code that checks this contract at run time, when called with a dynamic string. Whereas for constant
string expressions the compiler would perform that check and skip over the glue code.

Macros have more possibilities. While they can’t generally know which parameter is const, a literal certainly is. So,
every literal str passed to [`stringlet!(…)`](https://docs.rs/stringlet/latest/stringlet/macro.stringlet.html) is
validated at compile time. Additionally, if size is neither given explicitly, nor requested to be derived with `_`, it
is taken from the first parameter. For that it must be const, even if it is not a literal str. So that too is validated.

## Todo

- [x] `stringlet::error::Error` & `stringlet::Result`

- [x] Run `cargo llvm-cov` & `cargo crap` to eliminate untested code

- [ ] Run `cargo fuzz` to stress it

- [x] Run `cargo mutants` to find missing test constellations

- [x] Run Miri on both `{x86_64,s390x}-unknown-linux-gnu` to find unsound code

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

## *Tüddelband* Digression

<!-- DO NOT FORMAT the table body! -->

*Platt* (Low German)	|Semi-literal Translation
:---|:---
*Op de Straat löppt’n Jung mit’n **Tüddelband**<br>in’ne anner Hand’n Bodderbrood mit Kees,<br>wenn he blots ni mit de Been in’n Tüddel keem<br>un dor liggt he ok all lang op de Nees<br>un he rasselt mit’n Dassel op’n Kantsteen<br>un he bitt sick ganz geheurig op de Tung,<br>as he opsteiht, seggt he: Hett ni weeh doon,<br>dat’s’n Klacks för so’n Kieler Jung*|Upon the street runs a boy with a **Twiddle-String**<br>in another hand a buttered bread with cheese,<br>if he only not with the legs into the twiddle came<br>and there lies he already long upon the nose<br>and he rattles with the noggin upon a kerbstone<br>and he bites himself greatly upon the tongue,<br>as he up stands, says he: Has not hurt,<br>that’s a trifle for such a boy from Kiel

Dedicated to my father, who taught me this iconic Northgerman [▶ String
song](https://youtu.be/ByYTEReqf4Q?list=RDByYTEReqf4Q&t=38) (in a past long gone by, actually a metal hoop toy.)  The
many similar words nicely illustrate how Anglo-Saxon English comes from Northgermany. There are even more cognates that
have shifted in usage: *löppt:* elopes (runs) – *Jung:* young (boy) – *Been:* bones (legs) – *weeh doon:* woe done (has
hurt) – and maybe *Kant:* cant (edge)
