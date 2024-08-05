# Fields

Interface and macros for dynamically accessing struct fields at runtime.

## Usage
```rs
use fields::Fields;

#[derive(Default, Fields)]
struct MyStruct {
    valid: bool,
    id: u32,
    name: String,
}

let mut my_struct = MyStruct::default();
my_struct.set(MyStructField::Valid(true));
```

The name and derives of the generated fields enum can be customized:
```rs
#[derive(Default, Fields)]
#[fields(name = "MyField")]
#[fields(derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize))]
struct MyStruct {
    valid: bool,
    id: u32,
    name: String,
}

let mut my_struct = MyStruct::default();
my_struct.set(MyField::Valid(true));
```

Generated field variants can be filtered based on visibility:
```rs
#[derive(Fields)]
#[fields(visibility(pub, pub(crate)))]
struct Restricted {
    pub valid: bool,
    pub(crate) id: u32,
    name: String, // not in fields enum
}

#[derive(Fields)]
#[fields(visibility(priv))]
struct Private {
    pub valid: bool, // not in fields enum
    pub(crate) id: u32, // not in fields enum
    name: String,
}
```
