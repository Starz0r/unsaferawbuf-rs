# UnsafeRawBuf

[![GitHub](https://img.shields.io/github/license/Starz0r/unsaferawbuf-rs?style=flat-square)](https://github.com/Starz0r/unsaferawbuf-rs) [![crates.io badge](https://shields.io/crates/v/unsaferawbuf?style=flat-square)](https://crates.io/unsaferawbuf) [![Docs.rs](https://img.shields.io/docsrs/unsaferawbuf/latest?style=flat-square)](https://docs.rs/unsaferawbuf/latest) ![rustc requirements](https://img.shields.io/badge/rust-1.49+-brightgreen.svg?logo=rust&style=flat-square)

`unsaferawbuf` provides a interface to a raw chunk of allocated memory. Intended for usage with libraries or applications that can typically only communicate over via a shared memory region. Thus, making it very unsafe, and the crate it's self, provides very little safety guarantees. This simply acts as a container around a raw memory address, and does pointer arithmetic around it when reading and writing data to it. Allowing for some additional convince when transacting from an arbitrary memory address is unavoidable.

# Usage

Right now, you can only initialize a `UnsafeRawBuf` from a raw memory address. In the future, you'll be able to create a freshly allocated memory buffer along with it's associated interface. As of now, that is currently impossible, so for now we have to manually allocate memory, then assign it to the container.

```rust
use std::ffi::c_void;
use unsaferawbuf::UnsafeRawBuf;

pub unsafe extern "C" fn dmp(addr: *mut c_void) {
    let buf = UnsafeRawBuf::from_address(addr as _);
    buf.write(42);
}
```