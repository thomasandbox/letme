use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("Let me > ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Something wrong😢\nCould you restart?");

        let mut commands = input.split(" | ").peekable();
        let mut command_status = false;
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    match env::set_current_dir(&root) {
                        Ok(_) => {
                            previous_command = None;
                            command_status = true
                        }
                        Err(_) => {
                            eprintln!(
                                "\x1b[31mOops, the directory you tried to move doesn't exist😢\x1b[m"
                            );
                            command_status = false;
                        }
                    };
                }
                "exit" => {
                    println!("\x1b[36mSee you, Have a good time😊\x1b[m");
                    return;
                }
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    if let Ok(mut child) = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn()
                    {
                        child.wait().expect("Command wasn't running😢");
                        previous_command = Some(child);
                        command_status = true
                    } else {
                        previous_command = None;
                        eprintln!("\x1b[31mThe command you typed is invalid😢\x1b[m");
                        command_status = false
                    }
                }
            }
        }
        match command_status {
            true => {
                println!("\n😄✅\n");
            }
            false => {
                println!("\n😢❌\n");
            }
        }
    }
}
