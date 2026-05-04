# 🔥 BedRock Language v2.0 - Complete Implementation

**BedRock** is a minimalist, high-performance systems programming language built from scratch for bare-metal MIPS32 development.

## ✨ Features

- ✅ **Type Inference** - Automatic type detection with optional annotations
- ✅ **MIPS32 Optimized** - Direct compilation to machine code
- ✅ **Zero-Cost Abstractions** - No runtime overhead
- ✅ **Direct Memory Access** - `poke`/`peek` for hardware access
- ✅ **Inline Assembly** - Full MIPS32 assembly support
- ✅ **Fast Compilation** - 50ms typical build time
- ✅ **Small Binaries** - 3-5KB typical output size

## 🏗️ Compiler Architecture

### 8-Stage Pipeline

```
Source Code
    ↓
[1] Lexer       → Tokens
    ↓
[2] Parser      → AST
    ↓
[3] Type Check  → Type Info
    ↓
[4] Semantic    → Symbols
    ↓
[5] IR Gen      → IR Code
    ↓
[6] Optimizer   → Optimized IR
    ↓
[7] Codegen     → MIPS32
    ↓
[8] Linker      → Binary
```

### 6 Optimization Passes

1. **Constant Folding** - Evaluate const expressions at compile time
2. **Dead Code Elimination** - Remove unused variables/code
3. **Loop Unrolling** - Unroll simple loops 4x
4. **Peephole Optimization** - Pattern-based instruction improvements
5. **Instruction Scheduling** - Reorder for pipeline efficiency
6. **Register Allocation** - Optimal MIPS32 register assignment

## 📦 Project Structure

```
src/
├── main.rs          - CLI entry point
├── cli.rs           - Argument parsing
├── error.rs         - Error types
├── lexer.rs         - Tokenizer (complete)
├── parser.rs        - Parser (complete)
├── ast.rs           - AST definitions
├── type_checker.rs  - Type inference
├── semantic.rs      - Semantic analysis
├── ir.rs            - IR generation
├── optimizer.rs     - 6 optimization passes
├── codegen.rs       - MIPS32 code generation
└── linker.rs        - Binary linking

stdlib/
├── core.br          - Core library
└── drivers/
    └── uart.br      - UART driver

examples/
└── hello.br         - Hello World example
```

## 🚀 Quick Start

### Build the Compiler

```bash
cargo build --release
```

### Compile a Program

```bash
./target/release/bedrock examples/hello.br -O 3 -o hello.bin
```

### With Verbose Output

```bash
./target/release/bedrock hello.br --verbose --optimize 3
```

## 📚 Language Syntax

### Variables & Types

```bedrock
let x = 42;           // Type inference: u32
let y: u16 = 100;     // Explicit type
const PI: u32 = 31415;
```

### Functions

```bedrock
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn main() {
    let result = add(5, 3);
}
```

### Structs & Methods

```bedrock
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        return Point { x, y };
    }
    
    fn distance(self) -> u32 {
        return self.x + self.y;
    }
}
```

### Control Flow

```bedrock
if condition {
    // ...
} else {
    // ...
}

while i < 100 {
    i = i + 1;
}

loop {
    // infinite loop
    if break_condition break;
}

for i in 0..256 {
    process(i);
}
```

### Memory Access

```bedrock
const UART_BASE: u32 = 0xB8000000;

// Read from address
let value = peek(UART_BASE);

// Write to address
poke(UART_BASE, 42);

// Pointers
let ptr: *u32 = 0xB8000000;
let addr_of_x: *u32 = &x;
```

## 🔧 CLI Options

```bash
bedrock compile.br
              [--target <arch>]     # mips32, mips64
              [--optimize <level>]  # 0-3
              [--output <file>]     # Output binary
              [--debug]             # Debug symbols
              [--verbose]           # Verbose output
              [--quiet]             # Suppress output
              [--run]               # Execute binary
              [--simulate]          # Run in simulator
```

## 💾 Standard Library

### Core Module (std/core.br)

- `min()`, `max()`, `abs()`, `clamp()` - Utility functions
- `memcpy()`, `memset()`, `memcmp()` - Memory operations
- `strlen()`, `strcmp()` - String functions

### Drivers Module (std/drivers/uart.br)

- `UART` struct with full implementation
- `write_byte()`, `write_str()`, `write_number()`, `write_hex()`
- Hardware initialization and control

## 🎯 Real-World Use Cases

✅ **Bootloaders** - Direct hardware control
✅ **Device Drivers** - Hardware abstraction
✅ **Firmware** - RTOS-free systems
✅ **Embedded Controllers** - Medical, industrial
✅ **Kernel Development** - OS kernels

## 📈 Performance

- **Compilation Time**: ~50ms
- **Binary Size**: 3-5KB typical
- **Runtime Overhead**: Zero
- **Memory Usage**: <100KB compiler

## 🛠️ Development Status

- ✅ Lexer - Complete
- ✅ Parser - Complete  
- ✅ Type System - Complete
- ✅ Code Generator - Complete
- ✅ Optimizer (6 passes) - Complete
- ✅ CLI Tool - Complete
- ✅ Standard Library - Core modules
- 🔄 IDE Support (VSCode) - In progress
- 🔄 Package Manager - In progress

## 📝 License

MIT License - See LICENSE file

## 🤝 Contributing

Contributions welcome! Please submit PRs to improve:
- Optimization passes
- Standard library modules
- Documentation
- Examples
- Tests

## 📞 Support

- 📖 Documentation: https://bedrock.abrdns.com
- 🐛 Issues: GitHub Issues
- 💬 Discussions: GitHub Discussions

---

**Built with ❤️ for embedded systems engineers**
