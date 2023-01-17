fn input() -> String {
   let mut buf = String::new();
   std::io::stdin().read_line(&mut buf).unwrap();
   buf.trim().to_string()
}

fn main() {
    println!("Seja bem vindo ao gerador de TODO 🚀 ");
    loop {        
        if todo_start() {            
            break;
        }
    }
}

fn todo_start() -> bool {
    println!("Deseja adicionar um novo TODO? s/n");

    let question = input();
    if question == "s" {
        println!("Qual o nome do seu novo TODO");
        let ntodo: String = input();
        println!("Seu TODO foi criado 😎");
        println!("{}", ntodo);
        false
    } else if question == "n" {
        println!("Fechando gerador de TODO 😀");
        true
    } else {
        println!("Só pode ser inserida a letra 's' para gerar novo TODO ou 'n' para finalizar o gerador 🤔");
        false
    }
}
