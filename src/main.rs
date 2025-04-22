use clap::{self, Arg, Command};


fn main(){
    let cmd = clap::Command::new("base")
        .subcommand(Command::new("add").about("Adiciona item a lista").arg(clap::arg!(<item> "Item que será adicionado na lista")))
        .subcommand(Command::new("list").about("Lista todos os itens"))
        .subcommand(Command::new("done").about("Coloca como concluido algum item da lista").arg(clap::arg!(<index> "Index do item que foi concluido")))
        .subcommand(Command::new("remove").about("Remove algum item da lista").arg(clap::arg!(<index> "Index do item que será excluido")))
        .subcommand(Command::new("clear").about("Limpa a lista"))
        .arg_required_else_help(true);
    let matches = cmd.get_matches();
        
    match matches.subcommand(){
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
        _ => {
            println!("forneça algo valido");
        }
    }
}
