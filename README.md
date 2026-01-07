![mulstant](https://capsule-render.vercel.app/api?type=waving&height=300&color=gradient&text=Mulstant)

# <center>Mulstant</center>

[![CI](https://github.com/mincomk/mulstant/actions/workflows/ci.yml/badge.svg)](https://github.com/mincomk/mulstant/actions/workflows/ci.yml)

The meaning of the word "mulstant" is "multiple" + "instant".

A simple multi-timer utility for recording multiple durations between events in Rust.

## Usage
```bash
cargo add mulstant
```

### Example

```rust
use mulstant::Mulstant;
use std::{thread, time::Duration};

fn main() {
    let mut mu = Mulstant::new();

    // Record events
    mu.record("initialization");
    thread::sleep(Duration::from_millis(500));

    mu.record("process_data");
    thread::sleep(Duration::from_millis(300));

    mu.record("cleanup");

    // Finalize and get results
    let result = mu.finalize();

    // Print summary
    println!("{}", result.summary());
}
```

This will output something like:

```
initialization: 207.00ns
process_data: 500.06ms
cleanup: 300.10ms
Total Duration: 800.16ms
```

## Features

- **serde** Enables Serde support for `Record` and `MulstantResult`. Disabled by default.

## License

MIT
