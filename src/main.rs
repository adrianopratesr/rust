use std::io::{Stdin, Stdout, Write};

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Todo {
        return Todo { message };
    }
}

fn main() {
    println!("Seja bem vindo ao gerador de TODO 🚀 ");
    loop {
        let mut start_terminal = Terminal::new();
        if !start_terminal.ask_to_create_todo() {
            break;
        }
    }
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn ask_to_create_todo(&mut self) -> bool {
        println!("Você deseja criar um novo TODO? s/n");

        let answer = self.input();

        if answer == "s" {
            let todo = self.ask_for_new_todo();
            self.show_todo(&todo);
            true
        } else if answer == "n" {
            println!("Fechando gerador de TODO 😀");
            false
        } else {
            println!("Favor inserir apenas s/n");
            true
        }
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        println!("Qual o nome do seu novo TODO?");
        let name_todo = self.input();
        Todo::new(name_todo)
    }

    fn show_todo(&mut self, todo: &Todo) {
        println!("Seu TODO foi criado 😎");
        writeln!(self.stdout, "Seu TODO é: {}", todo.message).unwrap();
    }

    fn input(&mut self) -> String {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}
