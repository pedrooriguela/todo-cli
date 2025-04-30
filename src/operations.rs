use std::error::Error;
use crate::model::Tarefa;
use crate::storage::{ler_tarefas, salvar_tarefas, criar_arquivo_lista};



pub fn add(path: &String, item: String) -> Result<(), Box<dyn Error>> {
    let mut tarefas = ler_tarefas(path)?;
    
    let nova_tarefa = Tarefa {
        tarefa: item,
        estado: "incompleta".to_owned(),
    };
    
    tarefas.push(nova_tarefa);
    
    salvar_tarefas(path, &tarefas).expect("Erro ao salvar as novas tarefas");
    
    Ok(())
}

pub fn clear(path: &String) -> Result<(), Box<dyn Error>> {
    let tarefas: Vec<Tarefa> = Vec::new();
    salvar_tarefas(path, &tarefas)?;
    println!("Lista limpa!");
    
    Ok(())
}

pub fn create(path: &String) -> Result<(), Box<dyn Error>> {
    criar_arquivo_lista(path)
}

pub fn list (path: &String) -> Result<(), Box<dyn Error>> {
    let tarefas = ler_tarefas(path)?;
    for (i, tarefa) in tarefas.iter().enumerate(){
        println!("{}) {} - {}", i, tarefa.tarefa, tarefa.estado);
    }
    Ok(())
}

pub fn done(path: &String, index: usize) -> Result<(), Box<dyn Error>>{
    let mut tarefas = ler_tarefas(path)?;
    if index < tarefas.len() {
        tarefas[index].estado = "concluida".to_owned();
        salvar_tarefas(path, &tarefas)?;
    }
    else{
        println!("indice invalido")
    }
    Ok(())
}

pub fn remove(path: &String, index: usize) -> Result<(), Box<dyn Error>> {
    let mut tarefas = ler_tarefas(path)?;
    if index < tarefas.len() {
        tarefas.remove(index);
        salvar_tarefas(path, &tarefas)?;
    }
    else{
        println!("indice invalido")
    }
    Ok(())
}