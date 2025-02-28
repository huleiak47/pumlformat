# PlantUML Formatter

A command-line tool for formatting PlantUML diagrams with consistent indentation and spacing.

## Features

- Automatically indents nested blocks
- Normalizes spacing around symbols and arrows
- Handles common PlantUML syntax elements
- Configurable indentation size

## Installation

### From Source

1. Ensure Rust 1.85+ is installed
2. Build release binary:
   ```bash
   cd pumlformat
   cargo build --release
   ```
3. The binary will be in `target/release/pumlformat`

## Usage

Basic formatting:

```bash
pumlformatter input.puml -o formatted.puml
```

With custom indentation:

```bash
pumlformatter --indent 2 input.puml
```

Pipe from stdin/stdout:

```bash
cat input.puml | pumlformatter > formatted.puml
```

## Command Line Options

```
USAGE:
    pumlformatter [INPUT] [OPTIONS]

OPTIONS:
    -o, --output <OUTPUT>    Output file (default: stdout)
        --indent <INDENT>    Number of spaces for indentation [default: 4]
    -h, --help               Print help information
    -V, --version            Print version information
```

## Examples

Before:

```
@startuml
actor User

alt A
User -> Server : Login
else B
Server -> Database : Query
else C
Server -> Mailer : SendEmail
end
@enduml
```

After formatting:

```
@startuml
actor User

alt A
    User -> Server : Login
else B
    Server -> Database : Query
else C
    Server -> Mailer : SendEmail
end
@enduml
```

## License

MIT Licensed
