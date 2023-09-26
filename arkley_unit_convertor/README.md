# arkley_unit_converter 
`arkley_unit_converter` is a Rust library that provides utilities for unit conversions in various domains, such as time and data storage. It aims to simplify the process of converting values between different units of measurement , as well addition and subtract with them.

## Features

* Unit conversion for time durations.
* Unit conversion for data storage capacities.

## Installation

To use `arkley_unit_converter` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
arkley_unit_converter = "0.1.0"
```

## Usage

### Time Conversion

```rust
use arkley_unit_converter::{Time, TimeUnits};

let time = Time::new(3600.0, TimeUnits::Seconds); // 3600 seconds
let converted_time = time.convert_to(TimeUnits::Minutes); // Convert to minutes
println!("Converted time: {} {}", converted_time.number(), converted_time.current_unit());

```

### Data Storage Conversion

```rust
use arkley_unit_converter::{DataStorage, DataStorageUnit};

let data = DataStorage::new(1024.0, DataStorageUnit::Bytes); // 1024 bytes
let converted_data = data.convert_to(DataStorageUnit::Kilobytes); // Convert to kilobytes
println!("Converted data: {} {}", converted_data.number(), converted_data.current_unit());
```

## Documentation

For detailed documentation and examples, visit the [official documentation](https://docs.rs/arkley_unit_converter).
