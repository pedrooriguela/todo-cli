use clap::ArgMatches;
mod model;
mod cli;
mod storage;
mod operations;


fn execute(mtch: ArgMatches) {

    match mtch.subcommand(){
        Some(("add", sub_matches)) => {
            let item = sub_matches.get_one::<String>("item").expect("Item obrigatorio").to_string();
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            operations::add(path, item).expect("Erro ao adicionar a lista");
 
        },
        Some(("list", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            operations::list(path).expect("Erro ao listar");
        },
        Some(("remove", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            let index = sub_matches.get_one::<String>("index").expect("Índice inválido")
                                .parse::<usize>().expect("Índice deve ser um número");
            operations::remove(path, index).expect("Erro ao deletar tarefa");
        },
        Some(("done", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            let index = sub_matches.get_one::<String>("index").expect("Índice inválido")
                                .parse::<usize>().expect("Índice deve ser um número");
            operations::done(path, index).expect("Erro ao concluir tarefa");
        }
        Some(("clear", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            operations::clear(path).expect("Erro ao limpar");
        },
        Some(("create", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            operations::create(path).expect("Erro ao criar");
        }
        _ => {
            println!("forneça comando valido");
        }
    }

}   

fn main(){
    let matches = cli::cmd();
        
    execute(matches);
}
