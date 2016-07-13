# rust-cfitsio

[![Join the chat at https://gitter.im/mindriot101/rust-cfitsio](https://badges.gitter.im/mindriot101/rust-cfitsio.svg)](https://gitter.im/mindriot101/rust-cfitsio?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Build Status](https://travis-ci.org/mindriot101/rust-cfitsio.svg?branch=master)](https://travis-ci.org/mindriot101/rust-cfitsio)

FFI wrapper around cfitsio in Rust



## Api roadmap

```
FitsFile
- fn hdu -> returns FitsHdu
- fn next -> impl Iterator around FitsHdu objects

FitsHdu
- fn read_key -> returns header value
- if image:
    - fn image_dimensions -> Vec<usize>
    - fn image_type -> DataType
    - fn read_section -> reads image section into either Vec<_> or ndarray
- if table:
    - fn num_rows -> usize
    - fn rows -> impl Iterator over rows
    - fn row -> get single row by index
    - fn columns -> impl Iterator over columns
    - fn column -> get single column by name or index
```

### Images

* Change HDU
* Read image data
* Get image metadata

### Tables

## Examples

Open a fits file

```rust
let f = fitsio::FitsFile::open("test.fits");
```
