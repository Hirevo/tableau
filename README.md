<div align=center><h1>Tableau</h1></div>
<div align=center><strong>A library for building pretty tables in the terminal</strong></div>

<br />

<div align="center">
  <!-- crate version -->
  <a href="https://crates.io/crates/tableau">
    <img src="https://img.shields.io/crates/v/tableau" alt="crates.io version" />
  </a>
  <!-- crate downloads -->
  <a href="https://crates.io/crates/tableau">
    <img src="https://img.shields.io/crates/d/tableau" alt="crates.io download count" />
  </a>
  <!-- crate docs -->
  <a href="https://docs.rs/tableau">
    <img src="https://img.shields.io/docsrs/tableau" alt="docs.rs docs" />
  </a>
  <!-- crate license -->
  <a href="https://github.com/Hirevo/tableau#license">
    <img src="https://img.shields.io/crates/l/tableau" alt="crate license" />
  </a>
</div>

About
-----

This is a library for building pretty tables in the terminal.  

This library is heavily inspired from the [**term-table**] library.  

My goals with this library are:

- to serve as a common and stable terminal table library in some of my projects, like [**persist**] or [**rebench-tabler**].  
- to experiment and personally learn more about layout algorithms.  
- to potentially extend it with features that I find useful, or that I think would be interesting to implement.  

[**term-table**]: https://github.com/RyanBluth/term-table-rs
[**persist**]: https://github.com/Hirevo/persist
[**rebench-tabler**]: https://github.com/Hirevo/rebench-tabler

Features
--------

> All features marked as not yet implemented are ideas of things that could be done, and not necessarily an indication of it currently being worked on.

> These are also non-exhaustive, so if there is a feature that you'd like to see done, feel free to open an issue to show your interest in it, regardless of whether it is mentionned in this list or not.

> If you wish to contribute to this project, feel free to use this list to see if there is something you would be interested to either work on, or simply sharing some of your knowledge on how it could be achieved.

- [x] Multi-line cell contents
- [x] Handling of ANSI colors and attributes
- [x] Per-cell text alignment
- [x] Cells spanning multiple columns
- [ ] Cells spanning multiple rows
- [x] Per-column maximum width
- [x] Toggleable row separators

Examples
--------

Here is an example output from the `cities` example:

```text
╭─────────────────────────────────────────────────────╮
│               CITIES AROUND THE WORLD               │
├─────────────┬────────────────┬───────────┬──────────┤
│    CITY     │    COUNTRY     │ LONGITUDE │ LATITUDE │
├─────────────┼────────────────┼───────────┼──────────┤
│ Tokyo       │     Japan      │  139.6917 │  35.6895 │
│ Jakarta     │   Indonesia    │  106.8650 │  -6.1751 │
│ Delhi       │     India      │   77.1025 │  28.7041 │
│ Seoul       │  South Korea   │  126.9780 │  37.5665 │
│ Paris       │     France     │    2.3522 │  48.8566 │
│ London      │ United Kingdom │   -0.1276 │  51.5072 │
│ Beijing     │     China      │  116.4074 │  39.9042 │
│ New York    │ United States  │  -74.0060 │  40.7128 │
│ Sao Paulo   │     Brazil     │  -46.6333 │ -23.5505 │
│ Mexico City │     Mexico     │  -99.1332 │  19.4326 │
│ Mumbai      │     India      │   72.8777 │  19.0760 │
│ Los Angeles │ United States  │ -118.2437 │  34.0522 │
│ Istanbul    │     Turkey     │   28.9784 │  41.0082 │
╰─────────────┴────────────────┴───────────┴──────────╯
```

You can see more examples of how to use this library by looking at [**the different examples available**](https://github.com/Hirevo/tableau/tree/main/examples).

License
-------

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
