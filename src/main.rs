// src/main.rs
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Produto {
    nome: String,
    marca: String,
    categoria: String,
    preco: f32,
}

fn carregar_produtos() -> Vec<Produto> {
    let path = "produtos.json";
    if Path::new(path).exists() {
        let file = File::open(path).expect("Erro ao abrir produtos.json");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn salvar_produtos(produtos: &Vec<Produto>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("produtos.json")
        .expect("Erro ao abrir produtos.json para escrita");
    serde_json::to_writer_pretty(file, produtos).expect("Erro ao salvar produtos");
}

fn construir_indices(produtos: &[Produto]) -> (HashMap<String, Vec<usize>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<usize>>) {
    let mut por_nome = HashMap::new();
    let mut por_marca = HashMap::new();
    let mut por_categoria = HashMap::new();

    for (i, produto) in produtos.iter().enumerate() {
        por_nome.entry(produto.nome.to_lowercase()).or_insert(vec![]).push(i);
        por_marca.entry(produto.marca.to_lowercase()).or_insert(vec![]).push(i);
        por_categoria.entry(produto.categoria.to_lowercase()).or_insert(vec![]).push(i);
    }

    (por_nome, por_marca, por_categoria)
}

fn buscar_por_chave(indice: &HashMap<String, Vec<usize>>, chave: &str, produtos: &[Produto]) {
    let chave = chave.to_lowercase();
    if let Some(indices) = indice.get(&chave) {
        for &i in indices {
            println!("{:?}", produtos[i]);
        }
    } else {
        println!("Nenhum produto encontrado com chave '{}'.", chave);
    }
}

fn main() {
    let mut produtos = carregar_produtos();

    loop {
        println!("\n===== MENU MEGASTORE =====");
        println!("1. Adicionar produto");
        println!("2. Listar produtos");
        println!("3. Buscar por nome");
        println!("4. Buscar por marca");
        println!("5. Buscar por categoria");
        println!("6. Remover produto por nome");
        println!("0. Sair");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler opção");

        let (por_nome, por_marca, por_categoria) = construir_indices(&produtos);

        match opcao.trim() {
            "1" => {
                let mut nome = String::new();
                let mut marca = String::new();
                let mut categoria = String::new();
                let mut preco = String::new();

                println!("Nome:");
                io::stdin().read_line(&mut nome).unwrap();
                println!("Marca:");
                io::stdin().read_line(&mut marca).unwrap();
                println!("Categoria:");
                io::stdin().read_line(&mut categoria).unwrap();
                println!("Preço:");
                io::stdin().read_line(&mut preco).unwrap();

                let nome = nome.trim().to_string();
                let marca = marca.trim().to_string();
                let categoria = categoria.trim().to_string();
                let preco: f32 = preco.trim().parse().unwrap_or(0.0);

                if nome.is_empty() || marca.is_empty() || categoria.is_empty() {
                    println!("Todos os campos devem ser preenchidos corretamente.");
                } else {
                    let novo = Produto { nome, marca, categoria, preco };
                    produtos.push(novo);
                    println!("Produto adicionado.");
                }
            }
            "2" => {
                for produto in &produtos {
                    println!("{:?}", produto);
                }
            }
            "3" => {
                let mut chave = String::new();
                println!("Nome:");
                io::stdin().read_line(&mut chave).unwrap();
                buscar_por_chave(&por_nome, chave.trim(), &produtos);
            }
            "4" => {
                let mut chave = String::new();
                println!("Marca:");
                io::stdin().read_line(&mut chave).unwrap();
                buscar_por_chave(&por_marca, chave.trim(), &produtos);
            }
            "5" => {
                let mut chave = String::new();
                println!("Categoria:");
                io::stdin().read_line(&mut chave).unwrap();
                buscar_por_chave(&por_categoria, chave.trim(), &produtos);
            }
            "6" => {
                let mut nome = String::new();
                println!("Nome do produto para remover:");
                io::stdin().read_line(&mut nome).unwrap();
                let nome = nome.trim().to_lowercase();
                produtos.retain(|p| p.nome.to_lowercase() != nome);
                println!("Se existia, produto removido.");
            }
            "0" => {
                salvar_produtos(&produtos);
                println!("Encerrando e salvando...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}
