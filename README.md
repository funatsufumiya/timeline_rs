# timeline_rs

[![Crates.io](https://img.shields.io/crates/v/timeline_rs)](https://crates.io/crates/timeline_rs)
[![Docs.rs](https://docs.rs/timeline_rs/badge.svg)](https://docs.rs/timeline_rs)
[![License](https://img.shields.io/crates/l/timeline_rs)](LICENSE)

This is a simple timeline library for Rust. It is designed to be used in a game engine, but can be used for any purpose.

This work is heavily inspired by [ofxTimeline](https://github.com/YCAMInterlab/ofxTimeline) of [YCAMInterlab](https://github.com/YCAMInterlab), and intended to use data created by ofxTimeline and [loaf_timeline](https://github.com/funatsufumiya/loaf_timeline/) (lua/oF scripting environment using ofxTimeline).

## Examples

### Easing Tester

![screenshot_easing_tester](screenshot_easing_tester.png)

```bash
$ cargo run --example easing_tester --features bevy_example
```

### Timeline Simple

![screenshot_timeline_simple](screenshot_timeline_simple.png)

```bash
$ cargo run --example timeline_simple --features bevy_example
```

### Timeline From XML

![screenshot_timeline_simple](screenshot_timeline_simple.png)

```bash
$ cargo run --example timeline_from_xml --features bevy_example
```



## License Acknowledgements

My code-base is published under WTFPL and/or 0BSD (see [LICENSE_WTFPL](./LICENSE_WTFPL) and/or [LICENSE_0BSD](./LICENSE_0BSD)). However, the dependencies of this project have different licenses.

- `easing.rs` is ported from [ofxEasing.h](https://github.com/arturoc/ofxEasing/blob/master/src/ofxEasing.h) used in [ofxEasing](https://github.com/arturoc/ofxEasing), based on [terms of use](https://github.com/arturoc/ofxEasing/blob/3a15beffb9cfdce26ffafb1f78e06e730b26c239/src/easing_terms_of_use.html) (BSD License).

### Referenced projects

This lib is created to use data of [ofxTimeline](https://github.com/YCAMInterlab/ofxTimeline) of [YCAMInterlab](https://github.com/YCAMInterlab) and [loaf_timeline](https://github.com/funatsufumiya/loaf_timeline/) (lua/oF scripting environment using ofxTimeline).

Some code-bases are referenced from ofxTimeline, and some dependencies are also referenced like ofxEasing and ofxTween.

- [ofxEasing](https://github.com/arturoc/ofxEasing), is licensed under the MIT license. see [ofxEasing's LICENSE](https://github.com/arturoc/ofxEasing/blob/master/LICENSE)
- [ofxTween](https://github.com/arturoc/ofxTween), is licensed under the MIT license. see [ofxTween's LICENSE](https://github.com/arturoc/ofxTween/blob/master/LICENSE)
- [ofxTimeline](https://github.com/YCAMInterlab/ofxTimeline), is licensed under the Apache license. see [ofxTimeline's README](https://github.com/YCAMInterlab/ofxTimeline/blob/master/README.md)
