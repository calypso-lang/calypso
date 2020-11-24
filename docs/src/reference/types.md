# Types

This is a reference of all of the primitive types in Calypso. This is just a
reference on what types exist and a rough reference on how they work. Rigorous
definitions of how types are represented, operate, and interact will be present
in the VM guide.

## Table

| Name       | Rust Equivalent                                | Quick Description                                                                                 | 
|:----------:|:-----------------------------------------------|:--------------------------------------------------------------------------------------------------|
| [`undef`]  | no Rust equivalent                             | Variables that were never defined (or never bound, in non-strict mode).                           |
| [`null`]   | `Option::None`                                 | A value that was explicitly set to be nothing.                                                    |
| [`bool`]   | `bool`                                         | True or false values.                                                                             |
| [`sint`]   | `i64`                                          | Signed integers. Affixed by `s`, optionally, to disambiguate.                                     |
| [`uint`]   | `u64`                                          | Unsigned integers. Affixed by `u`, optionally, to disambiguate.                                   |
| [`float`]  | `f64`                                          | Floating point values (`binary64` in IEEE 754-2008). Affixed by `f`, optionally, to disambiguate. |
| [`string`] | `&str` or `String`                             | Mutable and immutable (stored in read-only section) strings of text as UTF-8.                     |
| [`char`]   | `char`                                         | A single Unicode codepoint stored in UTF-32.                                                      |
| [`tuple`]  | `(A, B, ...)`                                  | A fixed-length finite possibly heterogenous sequence.                                             |
| [`array`]  | `[A; N]` or `Vec<T>` (able to be heterogenous) | A dynamic-length possibly heterogenous sequence.                                                  |

[`undef`]:  #undef
[`null`]:   #null
[`bool`]:   #bool
[`sint`]:   #sint
[`uint`]:   #uint
[`float`]:  #float
[`string`]: #string
[`char`]:   #char
[`tuple`]:  #tuple
[`array`]:  #array

## `undef`

Variables that were never defined (or never bound, in non-strict mode).

All `undef` values are aliased (in the same memory location) in the VM.

### Examples

```calypso
let foo;
println(typeof(foo)); // undef
```

In non-strict mode, this is also valid:
```calypso
println(typeof(foo)); // undef
```

However, in strict mode, it would error because `foo` was never bound with a
`let` binding, as seen in the first example.

## `null`

A value that was explicitly set to be nothing.


All `null` values are aliased (in the same memory location) in the VM.

### Examples

```calypso
let foo = null;
if foo is null {
    println("Oh no! foo is null!");
}
```

## `bool`

`true` or `false` values.

### Examples

```calypso
let foo = true;
let bar = false;
if foo || bar {
    println("Either foo or bar was true");
}
```

## `sint`

Signed integers. Affixed by `s`, optionally, to disambiguate.

Integers in Calypso are assumed to be unsigned, unless a minus sign (`-`) is
present at the beginning. However, you can also affix a number with `s` like
this: `162s` to make sure that it is signed.

### Examples

```calypso
println(1 + 2s); // 3
```

```calypso
println(-1 + 6); // 5
```

## `uint`

Unsigned integers. Affixed by `u`, optionally, to disambiguate.

Integers in Calypso are assumed to be unsigned, unless a minus sign (`-`) is
present at the beginning. However, you can also affix a number with `u` like
this: `162u` to make sure that it is unsigned. This is not allowed for numbers
that have a minus sign present at the beginning.

### Examples

```calypso
println(5 + 3u); // 8
```

## `float`

Floating point values (`binary64` in IEEE 754-2008). Affixed by `f`, optionally,
to disambiguate.

Numbers in Calypso are assumed to be integers unless a decimal portion of the
number is present. However, you can also affix a number with `f` like this: `5f`
to make sure that it is a float. Integers are coerced into floats when necessary,
but floats are never coerced into integers as that would require truncation,
which loses data.

### Examples

```calypso
println(5.2 + 2.8); // 8.0
```

```calypso
println(7f * 4.0); // 28
```

## `string`

Mutable and immutable (stored in read-only section) strings of text as UTF-8.

Strings in Calypso are clone-on-write. They start off immutable (as they are
just references to a section of the bytecode file), but when a user wishes to
write to or change a string, it is cloned into the VM's stack or heap, then
modified there.

They are represented as UTF-8.

### Examples

```calypso
println("This is a string!");
```

```calypso
let mut str = "This is a string!"; // It's immutable right now. 
str = str <> " Thanks for using it!"; // Now it's mutable.
println(str); // This is a string! Thanks for using it!
```

## `char`

A single Unicode codepoint stored in UTF-32.

When concatenated to another character, it forms a string two characters long.
Characters can also be concatenated to strings, which just adds the character to
the beginning or end of the string.

### Examples

```calypso
let rainbow = '\u{1f308}';
let zwj = '\u{200d}';
let white_flag = '\u{1f3f3}';
println(rainbow <> zwj <> white_flag); // 🌈‍🏳 (rainbow flag, if your browser doesn't support emoji)
```

## `tuple`

A fixed-length, finite, possibly heterogenous sequence.

A tuple is stored contiguously in memory, but it's fixed-length. Elements are
accessed using `.E` where `E` is the zero-index element number.

### Examples

```calypso
let tup = (1, "hi", 3);
println(tup.1); // hi
```

## `array`

A dynamic-length possibly heterogenous sequence.

An array is stored contiguously in memory, but it's growable. In Rust, this is
implemented using a `Vec<T>`. Elements are accessed using `[E]` where `E` is the
zero-indexed element number.

### Examples

```calypso
let arr = [1, 2, "hello"];
println(arr[2]); // hello
arr = arr <> [5, 2.6, 'a'];
arr = arr <> null;
println(inspect(arr)); // [1, 2, "hello", 5, 2.6, 'a', null]
```
