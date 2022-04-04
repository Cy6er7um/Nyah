# 📕 Designs

This document tells the designations about the runtime of Nyah.

## Object

```rust
#[repr(usize)]
pub enum ObjectTagType {
    Unknown = 0,
    Static = 1,
    RefCount = 2,
}

pub struct ObjectTag {
    pub tag_type: ObjectTagType,
    pub ref_count: usize,
}

pub struct ObjectHeader {
    pub class: ClassReference,
    pub reference: ObjectTag,
}

pub struct ObjectModel<D> {
    pub header: ObjectHeader,
    pub data: D,
}

pub struct ObjectReference<D = Untyped, const I: usize = 0> {
    pub pointer: Pointer<ObjectModel<D>>,
    pub interfaces: InterfaceTable<I>,
}
```

### Object Header

The header of an object is used to store the class reference and the object tag made up by the tag type and the
reference counter.

### Object Reference

The object reference can take an object model pointer and a table of interfaces. The size of the reference instance is
dependent on the number of interfaces. For example, in the x86-64 architecture, the size of the reference with one
interface is 16 bytes (8 + 1*8 = 16).

For more information about the interface, see the [Interface](#Interface) section.

## Interface

_Unfinished_
