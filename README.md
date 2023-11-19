# arkley

**arkley** is your multi-purpose cross-platform , lightweight, offline math library, goes beyond the conventional to offer in algebraic calculations and clear explanations of mathematical concepts,while providing a simple interface to use but provides power and versatility normally reserved for complicated math packages.

It aims to be a convenient and cost-effective solution for performing algebraic operations, with the option to toggle step-by-step explanations , designed to handle various data types (including intergating custom types into the crate) and provide accurate results for precise calculations with useful error messages, making it suitable for both learners and advanced users. 

It includes useful tools for everyday needs (such as currency conversion and unit conversion) , so explore customizable functions, unit calculations, symbolic math, and more in with an intuitive interface, available on desktop, mobile, web , command line and any platform that can run [rust](https://www.rust-lang.org/)


**Note:** This library  is currently in early development, and some features described below may not be fully implemented or may exhibit limitations. Your feedback and contributions are invaluable as we work towards refining the library. Thank you for your understanding and support during this phase.

## Table of Contents

- [Features](#features)
  - [General](#general)
  - [Mathematical](#mathematical)
- [Getting Started](#getting-started)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Features 

### General

- **Platform Compatibility**: Designed to run efficiently on various platforms, including web, desktop, mobile, and low-level machines that support Rust code, providing a seamless experience.
- **Offline Usage**: Operates entirely offline, ensuring uninterrupted access to algebraic calculations and descriptions without the need for a constant internet connection.
- **Error Handling**: Includes robust mechanisms to handle common mathematical errors, ensuring calculations and operations are performed correctly and reliably.
- **Cost-Effective Solution**: Eliminates the need for expensive subscriptions or paid plans associated with online math resources. Provides a comprehensive set of algebraic tools and offline explanations.
- **Algebraic Calculations**: Perform a wide range of operations, including addition, subtraction, multiplication, and division, on numbers, variables, and expressions. It is equipped to handle complex mathematical expressions with ease , even with *custom types* 🤯.
- **Unit Conversion**: Effortlessly convert a diverse range of everyday units and physics-centric measurements into an extensive variety of units , including unconventional cases like meters per year..
- **Step-by-Step Explanations (Optional)**: Gain a deeper understanding of mathematical concepts with the option to toggle step-by-step explanations.
- **Translations Support (Optional)**: Optional translations for descriptions and explanations to accommodate users from diverse language backgrounds.

### Mathematical

- Basic operations `+ - * / ^ ` and `nth roots`
- Expressions may contain any combination of numbers, functions, units, variables, vectors and matrices, and dates
- Factorization and simplification
- Evaluation and Variable Subsitutions
- Supports complex and infinite numbers
- Differentiation and integration
- Quadractics
- Matrices and vectors, and related operations (determinants, etc.)
- Symbolic calculations like `3a * speed_of_light + 5y` where `speed_of_light = 3*10^8` and `a,y` are w/o values
- Solve majority equations and inequalities
- Basic constants: pi, e 
- Predefined functions like `cos` , `sin` with numeric or algebric expressions
- Can plot functions or data (matrices and vectors).
- Flexible - may contain simple numbers, units, or whole expressions. Multiple `Contexts` (data sets) with objects and associated properties

For detailed documentation, please visit the [core crate's documentation](https://docs.rs/arkley_algebra)

## Getting Started 

- To use it on the web, mobile, or desktop, refer to this [link](https://github.com/Deaths-Door/arkley/tree/main/arkley_ui)
- For command-line usage, refer to this [link](https://github.com/Deaths-Door/arkley/tree/main/arkley_cli)
- To use it as a library or explore its individual components, refer to this [link](https://github.com/Deaths-Door/arkley/tree/main/arkley_algebra)

## Documentation

For detailed documentation, please visit the [crate documentation](https://docs.rs/arkley)

## Contributing

We welcome contributions from the community to help improve and enhance Arkley. Your contributions are valuable in making Arkley even better!

## License

Arkley is licensed under the terms of either the MIT License or the Apache License (Version 2.0), at your option. You may choose to use this project under either of these licenses.

Please review the terms of each license before using this software in your commerical project.