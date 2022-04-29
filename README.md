sndfile.rs
==============

A safe rust wrapper of [libsndfile](http://www.mega-nerd.com/libsndfile/).  
With this crate, you can read or save audio files.

[![Travis-CI Status](https://travis-ci.org/Xeiron/sndfile.rs.svg?branch=master)](https://travis-ci.org/Xeiron/sndfile.rs)
[![Latest version](https://img.shields.io/crates/v/sndfile.svg)](https://crates.io/crates/sndfile)
[![Documentation](https://docs.rs/sndfile/badge.svg)](https://docs.rs/sndfile)
[![License](https://img.shields.io/crates/l/sndfile.svg)](https://github.com/Xeiron/sndfile.rs#license)

# Getting Started

[sndfile.rs is available on crates.io](https://crates.io/crates/sndfile).

With minimal features:
````toml
[dependencies]
sndfile = "0.1"
````

With ndarray supports:
````toml
[dependencies.sndfile]
version = "0.1"
features = ["ndarray_features"]
````

...and see the [docs](https://docs.rs/sndfile) for how to use it.

# Example

```rust
extern crate sndfile;
extern crate ndarray;

fn main() {
  use sndfile::*;
  let mut snd = sndfile::OpenOptions::ReadOnly(ReadOptions::Auto).from_path(
    "/mnt/st4t_0/tuxzz/muz/muz0/Call My Name/13.Loow.flac"
  ).unwrap();
  let data: ndarray::Array2<f32> = snd.read_all_to_ndarray().unwrap();

  let samplerate = snd.get_samplerate();
  let n_frame = snd.len().unwrap();
  let n_channels = snd.get_channels();
  let title = snd.get_tag(TagType::Title).unwrap();
  println!("Loaded song `{}`:", title);
  println!("  Length: {:.2} seconds", n_frame as f64 / samplerate as f64);
  println!("  Sample rate: {} Hz", samplerate);
  println!("  Channel count: {}", n_channels);
  println!("  DC offset = {}", data.mean().unwrap());
}

/*
== Expected output ==
Loaded song `Loow`:
  Length: 277.06 seconds
  Sample rate: 44100 Hz
  Channel count: 2
  DC offset = 0.00018921464
*/
```

## License

Licensed under of
 * MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
