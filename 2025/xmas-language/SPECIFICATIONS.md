# xmas Language Specification

This directory contains versioned specifications for the xmas programming language.

## Current Version

**v0.3** - See [SPECIFICATIONS_v0.3.md](./SPECIFICATIONS_v0.3.md)

## Version History

- **[v0.3](./SPECIFICATIONS_v0.3.md)** (Current) - Added `.rows()` method, enhanced string concatenation docs, comprehensive for loop examples, extended `~` operator for boolean-to-number conversion
- **[v0.2](./SPECIFICATIONS_v0.2.md)** - Added booleans, logical operators, range literals; changed numbers to integers only
- **[v0.1](./SPECIFICATIONS_v0.1.md)** (Initial) - Initial language specification

## Quick Reference

The xmas language is a terse, list-based language designed for solving Advent of Code challenges.

**Key Features:**

- All data structures are lists
- Numbers are integers (floats coming soon)
- Booleans: `true` and `false`
- Range literals: `[0..5]` creates `[0, 1, 2, 3, 4, 5]`
- Logical operators: `&&`, `||`, `!`
- Integer division: `10 / 3 == 3`

For full details, see the [latest specification](./SPECIFICATIONS_v0.3.md).
