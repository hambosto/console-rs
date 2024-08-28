# Custom Shell Implementation in Rust

This project implements a basic custom shell in Rust that supports command execution and piping. It provides a simple command-line interface where users can enter commands, change directories, and pipe output between commands.

## Features

- Basic command execution
- Support for command piping (e.g., `command1 | command2`)
- Built-in `cd` command for changing directories
- Built-in `exit` command to terminate the shell

## Getting Started

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Building the Project

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/custom-shell-rust.git
   cd custom-shell-rust
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the shell:
   ```
   cargo run --release
   ```

## Usage

Once the shell is running, you'll see a prompt (`>`). You can enter commands just like in a regular shell:

```
> ls -l
> cd /some/directory
> echo "Hello" | grep "H"
> exit
```

## Implementation Details

The shell is implemented with the following key components:

- `main()`: The main loop that continuously prompts for user input and executes commands.
- `parse_command_chain()`: Splits the input into individual commands for piping.
- `execute_command_chain()`: Executes a chain of commands, handling piping between them.
- `handle_cd_command()`: Implements the built-in `cd` command.
- `execute_single_command()`: Executes a single command with given arguments and I/O configurations.

## Limitations

- This is a basic implementation and does not support all features of a full-fledged shell.
- Error handling is minimal and can be improved.
- It doesn't support advanced features like input/output redirection, background processes, or shell scripting.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
