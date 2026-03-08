use std::process::Command;
use std::io;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            Command::new("ls").arg(&path).status().expect("Failed to execute ls");
        }
        FileOperation::Display(path) => {
            Command::new("cat").arg(&path).status().expect("Failed to execute cat");
        }
        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);
            Command::new("sh").arg("-c").arg(&command).status().expect("Failed to create file");
        }
        FileOperation::Remove(path) => {
            Command::new("rm").arg(&path).status().expect("Failed to remove file");
        }
        FileOperation::Pwd => {
            Command::new("pwd").status().expect("Failed to execute pwd");
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("Enter your choice (0-5): ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "0" => {
                println!("\nGoodbye!");
                break;
            }
            "1" => {
                println!("Enter directory path: ");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                perform_operation(FileOperation::List(path.trim().to_string()));
            }
            "2" => {
                println!("Enter file path: ");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                perform_operation(FileOperation::Display(path.trim().to_string()));
            }
            "3" => {
                println!("Enter file path: ");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                
                println!("Enter content: ");
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Failed to read line");
                
                perform_operation(FileOperation::Create(path.trim().to_string(), content.trim().to_string()));
                println!("File created successfully");
            }
            "4" => {
                println!("Enter file path: ");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                perform_operation(FileOperation::Remove(path.trim().to_string()));
                println!("File removed successfully");
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            _ => {
                println!("Invalid menu option");
            }
        }
    }
}