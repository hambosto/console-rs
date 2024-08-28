use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Custom shell implementation thats support basic command execution and piping.
fn main() -> io::Result<()> {
    loop {
        print_prompt()?;
        let user_input = read_user_input()?;

        if user_input.trim().is_empty() {
            continue;
        }

        let command_chain = parse_command_chain(&user_input);
        execute_command_chain(command_chain)?;
    }
}

/// Prints the shell prompt.
fn print_prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()
}

/// Reads a line of input from the user.
fn read_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

/// Splits the input string into individual commands for piping.
fn parse_command_chain(input: &str) -> Vec<&str> {
    input.trim().split(" | ").collect()
}

/// Executes a chain of commands, handling piping beetween them.
fn execute_command_chain(command_chain: Vec<&str>) -> io::Result<()> {
    let mut previous_command: Option<Child> = None;

    for (command_index, command) in command_chain.iter().enumerate() {
        let mut command_parts = command.trim().split_whitespace();
        let command_name = command_parts.next().unwrap_or("");
        let command_args: Vec<&str> = command_parts.collect();

        match command_name {
            "cd" => handle_cd_command(&command_args),
            "exit" => std::process::exit(0),
            _ => {
                let stdin = get_command_stdin(&mut previous_command);
                let stdout = get_command_stdout(command_index, &command_chain);

                match execute_single_command(command_name, &command_args, stdin, stdout) {
                    Ok(child_process) => previous_command = Some(child_process),
                    Err(error) => {
                        eprintln!("Error: {}", error);
                        previous_command = None;
                    }
                }
            }
        }
    }
    // Wait for the last command in the chain to finish
    if let Some(mut final_command) = previous_command {
        final_command.wait()?;
    }

    Ok(())
}

// Handles the `cd` command to change the current directory.
fn handle_cd_command(args: &[&str]) {
    let new_dir = args.first().map_or("/", |&x| x);
    let new_dir_path = Path::new(new_dir);
    if let Err(error) = env::set_current_dir(&new_dir_path) {
        eprintln!("cd error: {}", error);
    }
}

/// Determines the stdin for the current command based on the previous commands output.
fn get_command_stdin(previous_command: &mut Option<Child>) -> Stdio {
    match previous_command.take() {
        Some(mut child) => child.stdout.take().map_or(Stdio::inherit(), Stdio::from),
        None => Stdio::inherit(),
    }
}

/// Determines the stdout for the current command based on its position in the command chain.
fn get_command_stdout(current_index: usize, command_chain: &[&str]) -> Stdio {
    if current_index < command_chain.len() - 1 {
        // If there a next command, we need to pipe the output
        Stdio::piped()
    } else {
        // If it's the last command, output to terminal
        Stdio::inherit()
    }
}

/// Executes a single command with the given arguments and I/O configurations.
fn execute_single_command(
    command: &str,
    args: &[&str],
    stdin: Stdio,
    stdout: Stdio,
) -> io::Result<Child> {
    Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
}
