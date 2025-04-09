# Emdb_lib: Memory-efficient English Language Tokenizer

## Overview

Emdb_lib is a Rust library that implements Emma Dearborn's Speedwriting system for orthographic token compression of English text. This system reduces the number of tokens required to encode meaning by applying a set of phonetic and orthographic transformations, making English easier for machines to understand.

The library provides a clean, efficient API for converting standard English text to Speedwriting notation and back.

## Features

- **Text Compression**: Losslessly compress English text by applying Dearborn's 108 principles
- **Efficient Implementation**: Optimized data structures and algorithms for fast text processing
- **Flexible API**: Configure which rules to apply and in what order
- **Comprehensive Documentation**: Detailed explanations of the principles and their implementation
- **Benchmarking**: Performance measurements for optimizing critical operations

## Usage

Add `emdb_lib` to your `Cargo.toml`:

```toml
[dependencies]
emdb_lib = "0.1.3"
```

Basic usage example:

```rust
use emdb_lib::Speedwriter;

fn main() {
    // Create a new Speedwriter instance
    let speedwriter = Speedwriter::new();
    
    // Convert text to speedwriting notation
    let result = speedwriter.to_speedwriting("This is a test message").unwrap();
    println!("Speedwriting: {}", result);
}
```

Advanced usage with custom rule selection:

```rust
use emdb_lib::{Speedwriter, PrefixTransform, SuffixTransform};

fn main() {
    // Create a customized Speedwriter with selected rules
    let speedwriter = Speedwriter::builder()
        .with_rule(PrefixTransform::new())
        .with_rule(SuffixTransform::new())
        .build();
    
    // Apply the transformations
    let result = speedwriter.to_speedwriting("Customize which rules to apply").unwrap();
    println!("Custom Speedwriting: {}", result);
}
```

## Architecture

The library is organized around the following core components:

1. **Speedwriter**: Main entry point for the API
2. **Transform**: Trait implemented by all transformation rules
3. **MorphemeLookup**: Efficient prefix and suffix lookup
4. **PrefixTrie/SuffixTrie**: Optimized data structures for pattern matching

Transformation rules are grouped into categories:

- **Words/Beginnings**: Handling word prefixes (Rule 40, 46, etc.)
- **Words/Endings**: Handling word suffixes (Rule 9, 14, etc.)
- **Phoneme Clusters**: Special sound groups (Rule 24, 38, etc.)
- **Punctuation**: Handling punctuation marks (Rule 5, 10, etc.)
- **Phrasing**: Combining words and phrases (Rule 4, 12, etc.)

## Performance Optimizations

The library includes several optimizations for performance:

1. **Efficient Data Structures**:
   - Tries for prefix and suffix matching
   - HashSets and HashMaps for quick lookups

2. **Minimized Allocations**:
   - Use of Cow<str> to avoid unnecessary string allocations
   - String reuse for repeated operations

3. **Algorithmic Improvements**:
   - Fast pattern matching instead of regex for common cases
   - Optimized prefix/suffix detection

4. **Memory Efficiency**:
   - Static data for common patterns
   - Compact representation of rules

## Contributing

Contributions are welcome! Here are ways you can contribute:

1. Implementing additional Speedwriting principles
2. Improving performance of existing implementations
3. Adding tests and documentation
4. Reporting bugs or suggesting features

## License

This project is licensed under the MIT License - see the LICENSE file for details.
