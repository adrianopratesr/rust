#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Todo {
        return Todo { message };
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    println!("Seja bem vindo ao gerador de TODO 🚀 ");
    let start_terminal = Terminal::new();
    loop {
        if start_terminal.ask_to_create_todo() {
            start_terminal.create_todo();
        } else {
            println!("Fechando gerador de TODO 😀");
            std::process::exit(0)
        }
    }
}

struct Terminal {}

impl Terminal {
    fn new() -> Terminal {
        Terminal {}
    }

    fn create_todo(&self) {
        println!("Qual o nome do seu novo TODO?");
        let name_todo = input();
        let todo = Todo::new(name_todo);
        self.show_todo(&todo);
    }

    fn ask_to_create_todo(&self) -> bool {
        loop {
            println!("Você deseja criar um novo TODO? s/n");
            let answer = input();
            if answer == "s" {
                return true;
            } else if answer == "n" {
                return false;
            } else {
                println!("Favor inserir apenas s/n");
            }
        }
    }

    fn show_todo(&self, todo: &Todo) {
        println!("Seu TODO foi criado 😎");
        println!("Seu TODO é: {}", todo.message);
    }
}
