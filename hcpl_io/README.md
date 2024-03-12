This module contains various tools for standard I/O operations,
specifically used for competitive programming.

# The `Cinable` derive macro
You can `derive(Cinable)` for all accepted derive inputs:
struct, enum, and union.

Please note that this derive macro is available on **crate
feature** `derive` only.

## Using `derive(Cinable)` on a struct
Here is an example of `derive(Cinable)` usage on all of three kinds
of struct expressions: struct, tuple, and unit.

```rust
use hcpl_proc_macro::Cinable;

#[derive(Cinable)]
struct StructStruct {
    x: char,
    y: i32,
}

#[derive(Cinable)]
struct TupleStruct(bool, u64);

#[derive(Cinable)]
struct UnitStruct;
```

The macro for the code above will expand to something
like the following.
```rust
# use hcpl_io;
# #[allow(dead_code)]
# struct StructStruct {
#    x: char,
#    y: i32,
# }
#[automatically_derived]
impl hcpl_io::Cinable for StructStruct {
    fn read_from(cin: &mut hcpl_io::Cin) -> Self {
        StructStruct {
            x: cin.get(),
            y: cin.get(),
        }
    }
}

# struct TupleStruct(bool, u64);
#[automatically_derived]
impl hcpl_io::Cinable for TupleStruct {
    fn read_from(cin: &mut hcpl_io::Cin) -> Self {
        TupleStruct(cin.get(), cin.get())
    }
}

# struct UnitStruct;
#[automatically_derived]
impl hcpl_io::Cinable for UnitStruct {
    fn read_from(cin: &mut hcpl_io::Cin) -> Self {
        UnitStruct
    }
}
```

## Using `derive(Cinable)` on an enum
To inform the program about which enum variant needs to be
constructed, you need to add tags to each variant. This is
achieved by using the `tag` attribute. Consequently, you need
to explicitly specify the type of tags before the enum's
definition using the `tag_type` attribute. For more information
about the tag_type and tag attributes, please refer to the
[The tag_type and tag attributes](#the-tag_type-and-tag-attributes)
section.

Here is an example of `derive(Cinable)` usage on an enum.
```rust
use hcpl_proc_macro::Cinable;

#[derive(Cinable)]
#[allow(dead_code)]
#[tag_type(char)]
enum UfdsOperations {
    #[tag = 'u']
    Unite { x: usize, y: usize },
    #[tag = 'f']
    Find(usize),
    #[tag = 's']
    Size,
}
```

The macro for the code above will expand to something
like the following.
```rust
# use hcpl_io;
# #[allow(dead_code)]
# enum UfdsOperations {
#     Unite { x: usize, y: usize },
#     Find(usize),
#     Size,
# }
#[automatically_derived]
impl hcpl_io::Cinable for UfdsOperations {
    fn read_from(cin: &mut hcpl_io::Cin) -> Self {
        let tag = cin.get::<char>();
        match tag {
            'u' => {
                Self::Unite {
                    x: cin.get(),
                    y: cin.get(),
                }
            }
            'f' => Self::Find(cin.get()),
            's' => Self::Size,
            _ => panic!("unexpected value: {0:?}", tag),
        }
    }
}
```

## Deriving `Cinable` on a union
Although using an enum is sufficient for most use cases, you can
also `derive(Cinable)` for unions. Like enums, you must provide
`tag_type` and `tag` attributes.

Here is an example of `derive(Cinable)` usage on an union.
```rust
use hcpl_proc_macro::Cinable;

#[derive(Cinable)]
#[tag_type(u16)]
union Ascii {
    #[tag = 2]
    ch: char,
    #[tag = 0]
    byte: u8,
    #[tag = 1]
    another_byte: u8,
}
```

The macro for the code above will expand to something
like the following.
```rust
# use hcpl_io;
# union Ascii {
#     ch: char,
#     byte: u8,
#     another_byte: u8,
# }
#[automatically_derived]
impl hcpl_io::Cinable for Ascii {
    fn read_from(cin: &mut hcpl_io::Cin) -> Self {
        let tag = cin.get::<u16>();
        match tag {
            2 => Self { ch: cin.get() },
            0 => Self { byte: cin.get() },
            1 => Self { another_byte: cin.get() },
            _ => panic!("unexpected value: {0:?}", tag),
        }
    }
}
```

## The `tag_type` and `tag` attributes
- The `tag_type` attribute defines the tag type for tagged
unions. It has the following syntax:
  ```text
  #[tag_type(T)]
  ```
  Where `T` is a type and must implement [`Cinable`]. Each enum
  and union that uses `derive(Cinable)` must have **exactly one**
  `tag_type` attribute.

- The `tag` attribute defines the tag for enum variants and union
fields. It has the following syntax for name-value pairs:
  ```text
  #[tag = lit]
  ```
  or the following syntax for list-style attributes:
  ```text
  #[tag(expr_lit, ...)]
  ```
  Where `lit` is a literal and `expr_lit` is a literal expression.
Each enum variant and union field must have **at least one**
tag` attribute.

  Please note that the value in name-value syntax is a literal. That
means you can't use `-1` as value because it consists of a minus
sign (`-`) and the literal `1`. Instead, use the list-style syntax
for that.

Here is an example to demonstrate how to use the `tag_type` and
`tag` attributes.
```rust
use hcpl_proc_macro::Cinable;

#[derive(Cinable)]
#[tag_type(i32)]
enum NumberType {
    #[tag = 1]
    One,
    #[tag(2, 3, 5)]
    Prime,
    #[tag(-2, -5)]
    #[tag(-3)]
    NegativePrime,
    #[tag(100,)]
    #[tag = 727]
    #[tag = 421]
    ThreeDigits
}
```

An enum variant or union field that has multiple tags
will expand to a sequence of literal patterns separated
by vertical bars (`|`). The macro for the code above
will expand to something like the following. 
```rust
# use hcpl_io;
# use hcpl_proc_macro::Cinable;
# enum NumberType {
#     One,
#     Prime,
#     NegativePrime,
#     ThreeDigits,
# }
#[automatically_derived]
impl hcpl_io::Cinable for NumberType {
    fn read_from(cin: &mut hcpl_io::Cin) -> Self {
        let tag = cin.get::<i32>();
        match tag {
            1 => Self::One,
            2 | 3 | 5 => Self::Prime,
            -2 | -5 | -3 => Self::NegativePrime,
            100 | 727 | 421 => Self::ThreeDigits,
            _ => panic!("unexpected value: {0:?}", tag),
        }
    }
}
```