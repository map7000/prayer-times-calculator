# PrayTimes.js v2.3 reference fixture

This directory backs the parity expectations used in `../calculator.rs`.

- `PrayTimes.js` — the **original PrayTimes.js v2.3** library from
  [PrayTimes.org](http://praytimes.org) (Copyright (C) 2007-2011 Hamid
  Zarrabi-Zadeh, LGPL 3.0). It is the algorithm this crate is based on.
- `reference_harness.js` — a Node script that drives the library above with
  the same settings the Rust port uses and prints `Float` times for each
  parity fixture case. Run it with:

  ```sh
  node tests/fixtures/praytimes/reference_harness.js
  ```

  regenerating `reference.json`.
- `reference.json` — the generated reference values. The numbers quoted in
  `../calculator.rs` are these values rounded to three decimals (minutes).

Note: this crate runs the solver with 2 iterations where the upstream JS uses
1, so values can differ by a fraction of a minute; the test tolerances
account for that.
