use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::error::Error;
use crate::model::Tarefa;


pub fn criar_arquivo_lista(path: &String) -> Result<(), Box<dyn Error>>{

    if !Path::new(path).exists() {
        let file = File::create(path)?;
        let tarefas: Vec<Tarefa> = Vec::new();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &tarefas)?;
        Ok(())
    }
    else {
        println!("Arquivo já existe");
        Ok(())
    }
}

pub fn salvar_tarefas(path: &String, tarefas: &Vec<Tarefa>) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tarefas)?;
    Ok(())
}

pub fn ler_tarefas(path: &String) -> Result<Vec<Tarefa>, Box<dyn Error>> {
    
    if !Path::new(path).exists() {
        return Ok(Vec::new()); 
    }
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let tarefas: Vec<Tarefa> = serde_json::from_reader(reader)?;
    
    Ok(tarefas)
}