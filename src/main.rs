use std::io::{self};


fn trim_input(s: &mut String){
    s.pop();
}

fn main(){
    let mut tarefas = Vec::new();
    let mut input = String::new();
    println!("Insira a tarefa");
    io::stdin()
        .read_line(&mut input)
        .expect("erro ao ler linha");

    trim_input(&mut input);

    tarefas.push(input);
    println!("{:#?}", tarefas);
}