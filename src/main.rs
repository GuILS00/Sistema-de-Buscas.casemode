// sistema_busca_megastore/src/main.rs

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Produto {
    nome: String,
    marca: String,
    categoria: String,
}

type BaseDeDados = HashMap<String, Produto>;

fn carregar_dados(caminho: &str) -> BaseDeDados {
    if Path::new(caminho).exists() {
        let arquivo = File::open(caminho).expect("Erro ao abrir o arquivo.");
        let leitor = BufReader::new(arquivo);
        serde_json::from_reader(leitor).unwrap_or_default()
    } else {
        BaseDeDados::new()
    }
}

fn salvar_dados(caminho: &str, dados: &BaseDeDados) {
    let arquivo = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(caminho)
        .expect("Erro ao salvar o arquivo.");

    serde_json::to_writer_pretty(arquivo, dados).expect("Erro ao escrever JSON.");
}

fn ler_input(msg: &str) -> String {
    println!("{}", msg);
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

fn adicionar_produto(dados: &mut BaseDeDados) {
    let nome = ler_input("Nome:");
    let marca = ler_input("Marca:");
    let categoria = ler_input("Categoria:");

    let produto = Produto { nome: nome.clone(), marca, categoria };
    dados.insert(nome, produto);
    println!("Produto adicionado com sucesso!\n");
}

fn remover_produto(dados: &mut BaseDeDados) {
    let nome = ler_input("Nome do produto a remover:");
    if dados.remove(&nome).is_some() {
        println!("Produto removido.\n");
    } else {
        println!("Produto não encontrado.\n");
    }
}

fn buscar(dados: &BaseDeDados, campo: &str) {
    let valor = ler_input(&format!("Buscar por {}:", campo));
    for produto in dados.values() {
        let campo_valor = match campo {
            "nome" => &produto.nome,
            "marca" => &produto.marca,
            "categoria" => &produto.categoria,
            _ => "",
        };

        if campo_valor.eq_ignore_ascii_case(&valor) {
            println!("{:?}", produto);
        }
    }
    println!("");
}

fn listar(dados: &BaseDeDados) {
    if dados.is_empty() {
        println!("Nenhum produto cadastrado.\n");
    } else {
        for produto in dados.values() {
            println!("{:?}", produto);
        }
        println!("");
    }
}

fn main() {
    let caminho = "produtos.json";
    let mut dados = carregar_dados(caminho);

    loop {
        println!("\nSistema de Busca - MegaStore");
        println!("1. Adicionar produto");
        println!("2. Remover produto");
        println!("3. Buscar por nome");
        println!("4. Buscar por marca");
        println!("5. Buscar por categoria");
        println!("6. Listar todos os produtos");
        println!("0. Sair");

        let escolha = ler_input("Escolha uma opção:");

        match escolha.as_str() {
            "1" => adicionar_produto(&mut dados),
            "2" => remover_produto(&mut dados),
            "3" => buscar(&dados, "nome"),
            "4" => buscar(&dados, "marca"),
            "5" => buscar(&dados, "categoria"),
            "6" => listar(&dados),
            "0" => {
                salvar_dados(caminho, &dados);
                println!("Dados salvos. Encerrando...");
                break;
            },
            _ => println!("Opção inválida.\n"),
        }
    }
}
