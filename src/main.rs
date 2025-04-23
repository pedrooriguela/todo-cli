use clap::{self, ArgMatches, Command};
use std::fs::File;
use std::path::Path;

struct Tarefa{
    id: i32,
    tarefa: String,
    descricao: String
}

fn cmd() -> ArgMatches{

    let cmd = clap::Command::new("base")
        .subcommand(Command::new("add")
            .about("Adiciona item a lista")
            .arg(clap::arg!(<path> "Path do arquivo da lista"))
            .arg(clap::arg!(<item> "Item que será adicionado na lista"))
            .arg(clap::arg!(-d --descricao <descricao> "Descricao para a tarefa").required(false))
        )
        .subcommand(Command::new("list")
            .about("Lista todos os itens")
            .arg(clap::arg!(<path> "Path do arquivo da lista"))
        )
        .subcommand(Command::new("done")
            .about("Coloca como concluido algum item da lista")
            .arg(clap::arg!(<path> "Path do arquivo da lista"))
            .arg(clap::arg!(<index> "Index do item que foi concluido"))
        )
        .subcommand(Command::new("remove")
            .about("Remove algum item da lista")
            .arg(clap::arg!(<path> "Path do arquivo da lista"))
            .arg(clap::arg!(<index> "Index do item que será excluido"))
        )
        .subcommand(Command::new("clear")
            .about("Limpa a lista")
            .arg(clap::arg!(<path> "Path do arquivo da lista"))
        )
        .subcommand(Command::new("create")
            .about("cria uma nova lista")
            .arg(clap::arg!(<path> "Path do arquivo da lista"))
        )
        .arg_required_else_help(true);
        
    let ret = cmd.get_matches();
    return ret;

}

fn create(path: &String)  {
    if !Path::new(path).exists() {
        let _f = File::create_new(path);
    }
    else {
        println!("Arquivo já existe");
    }
}



fn execute(mtch: ArgMatches) {

    match mtch.subcommand(){
        Some(("add", sub_matches)) => {
            let item = sub_matches.get_one::<String>("item").expect("Item obrigatorio");
            println!("Adicionando {:?}", item);
        },
        Some(("list", _)) => {
            println!("Listando")
        },
        Some(("remove", sub_matches)) => {
            let index = sub_matches.get_one::<String>("index").expect("insira indice valido");
            println!("removendo indice {}", index);
        },
        Some(("done", sub_matches)) => {
            let index = sub_matches.get_one::<String>("index").expect("insira indice valido");
            println!("concluindo indice {}", index);
        }
        Some(("clear", _)) => {
            println!("Limpando")
        },
        Some(("create", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("Path invalido");
            create(path);
        }
        _ => {
            println!("forneça comando valido");
        }
    }

}

fn main(){
    let matches = cmd();
        
    execute(matches);
}
