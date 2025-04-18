# String Utilities CLI

A command-line utility for performing various operations on strings.

## Installation

```bash
# Clone the repository
git clone https://github.com/sanmonsua/stringutils-cli.git
cd stringutils-cli

# Build the project
cargo build --release

# You can also install it globally
cargo install --path .
```

## Commands

The CLI supports the following commands:

### Reverse

Reverses the characters in a string.

```bash
stringutils-cli reverse "hello world"
# Output: dlrow olleh
```

### Palindrome

Checks if a string is a palindrome (reads the same forward and backward).

```bash
stringutils-cli palindrome "radar"
# Output: Yes! This word is a palindrome

stringutils-cli palindrome "hello"
# Output: Is not a palindrome
```

### Count

Counts characters or words in a string.

```bash
# Count characters (default behavior)
stringutils-cli count "hello world"
# Output: Character count: 11

# Count words
stringutils-cli count "hello world" --words
# Output: Word count: 2

# Explicitly specify character counting
stringutils-cli count "hello world" --chars
# Output: Character count: 11
```

### Unique

Checks if all characters or words in a string are unique and displays the unique characters/words.

```bash
# Check for unique characters (default behavior)
stringutils-cli unique "abcdef"
# Output: 
# Unique characters: {'a', 'b', 'c', 'd', 'e', 'f'}
# All characters are unique

stringutils-cli unique "hello"
# Output:
# Unique characters: {'h', 'e', 'l', 'o'}
# Not all characters are unique

# Check for unique words
stringutils-cli unique "hello world hello" --words
# Output:
# Unique words: {"hello", "world"}
# Not all words are unique

stringutils-cli unique "one two three" --words
# Output:
# Unique words: {"one", "two", "three"}
# All words are unique
```

## Global Options

```bash
# Show help
stringutils-cli --help

# Show version
stringutils-cli --version
```

## Development

### Project Structure

```
src/
├── main.rs          # Entry point and CLI definition
├── commands/        # Command implementations
│   ├── mod.rs       # Command module exports
│   ├── reverse.rs   # Reverse command implementation
│   ├── palindrome.rs # Palindrome command implementation
│   ├── count.rs     # Count command implementation
│   └── unique.rs    # Unique command implementation
└── utils.rs         # Shared utility functions
```
