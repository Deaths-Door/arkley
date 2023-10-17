# arkley

**arkley** is a lightweight and offline math library that provides powerful algebraic calculations and explanations of mathematical concepts. It aims to be a convenient and cost-effective solution for performing algebraic operations, with the option to toggle step-by-step explanations. Arkley is designed to handle various data types and provide accurate results for precise calculations, making it suitable for both learners and advanced users.

**Note:** This library is currently in early development, and some features described below may not be fully implemented or may not work correctly. Please be aware that you might encounter issues or limitations while using the library until it reaches a more stable and mature state. Your feedback and contributions are valuable in helping us enhance and refine the library further. Thank you for your understanding and support during this development phase.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Algebraic Calculations** : Perform a wide range of algebraic operations, including addition, subtraction, multiplication, and division, on numbers, variables, and expressions. Arkley is equipped to handle complex mathematical expressions with ease.
- **Step-by-Step Explanations  (Optional):** Gain a deeper understanding of mathematical concepts with the option to toggle step-by-step explanations. Whether you are learning algebra or need a refresher, Arkley provides clear descriptions that guide you through each step of the process. You can choose to have explanations enabled or disabled based on your preferences.
- **Minimal Overhead for Explanations (Optional):** For those who prefer to focus solely on algebraic calculations, Arkley provides the option to toggle off step-by-step explanations. This ensures that the library runs efficiently with minimal overhead, meeting the needs of users seeking quick and accurate algebraic results.
- **Accessible Descriptions:** Arkley offers concise descriptions, explanations, and guidance for various mathematical concepts, theorems, formulas, and operations. The explanations are provided in plain language, making complex concepts more accessible to users of different mathematical proficiency levels.
- **Offline Usage:** Arkley operates entirely offline, ensuring that you have uninterrupted access to algebraic calculations and descriptions wherever you are, without the need for a constant internet connection. This makes it ideal for use in areas with limited or no internet access.
- **Unit Conversion :**  With the Arkley package, you can effortlessly convert a diverse range of everyday units and physics-centric measurements into an extensive variety of units, including unconventional cases like meters per hour.
- **Cost-Effective Solution:** Arkley eliminates the need for expensive subscriptions or paid plans that are often associated with online math resources. By providing a comprehensive set of algebraic tools and offline explanations.
- **Customization for Target Audience:** Tailor the content to match the proficiency level of your target audience. Arkley includes level filters, allowing users to select their desired level, such as beginner, intermediate, or advanced. This customization ensures that the explanations and descriptions cater to the specific needs and understanding of the users.
- **Translations Support (Optional):** Arkley offers optional translations for its descriptions and explanations, allowing users from diverse language backgrounds to benefit from the library's capabilities. Users can enable or disable translations as per their preferences, making it a flexible feature for a broader audience.
- **Error Handling:** Arkley includes robust error handling mechanisms to handle common mathematical errors, such as division by zero, invalid input, and undefined operations. This ensures that calculations and operations are performed correctly and reliably.
- **Platform Compatibility:** Arkley is designed to be compatible with various platforms, including web, desktop, mobile, and low-level machines. It can run efficiently on any machine that supports Rust code, providing a seamless experience across different devices.

## Getting Started

To get started with *arkley*, you'll need to install it and understand how to use its features.

### Prerequisites

Before you begin, ensure you have the following prerequisites:

- [Rust](https://www.rust-lang.org) (Stable version recommended for most users)

Please note that the stable version of the Rust compiler is recommended but it's important to mention that certain features within *arkley* may be more efficient or exclusively available on the nightly version of the Rust compiler.

### Installation

To use this crate, you have the flexibility to use the umbrella "arkley" with predefined features

```toml
[dependencies]
arkley = "0.0.3" # Version
```

*Alternatively*, if you prefer a more customized setup, you can install individual crates without their features. For example:

```toml
[dependencies]
arkley_algebra = { version = "0.0.3" , features = [..] } # Fill in features 
```

## Usage

Here's an example of how to rearrange an equation to isolate 'x' as the subject:

```rust
use arkley::*;

fn main() {
    // Create an equation
    let equation = Equation::try_from("3x - 5(5 - 6y) = 9").unwrap();

    // Define the subject you want to isolate (in this case, 'x')
    let subject = Term::from('x');

    // Attempt to make 'x' the subject of the equation
    match equation.try_make_subject(subject) {
        Ok(value) => println!("Rearranged equation to make 'x' the subject: {}", value),
        Err(error) => println!("Unexpected error: {}", error),
    }
}
```

## Documentation

For detailed documentation, please visit the [crate documentation](https://docs.rs/arkley)

## Contributing

We welcome contributions from the community to help improve and enhance Arkley. Your contributions are valuable in making Arkley even better!

## License

Arkley is licensed under the terms of either the MIT License or the Apache License (Version 2.0), at your option. You may choose to use this project under either of these licenses.

Please review the terms of each license before using this software in your commericalproject.
