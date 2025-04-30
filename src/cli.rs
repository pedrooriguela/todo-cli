use clap::{self, ArgMatches, Command};

pub fn cmd() -> ArgMatches{

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
        
    cmd.get_matches()

}