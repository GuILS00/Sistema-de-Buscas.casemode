use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
    marca: String,
}

struct Sistema {
    produtos: Vec<Produto>,
    indice_nome: HashMap<String, HashSet<u32>>,     // nome -> ids de produtos
    indice_marca: HashMap<String, HashSet<u32>>,    // marca -> ids de produtos
    indice_categoria: HashMap<String, HashSet<u32>>, // categoria -> ids de produtos
    proximo_id: u32,
}

impl Sistema {
    fn new() -> Self {
        let mut sistema = Sistema {
            produtos: vec![],
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
            proximo_id: 1,
        };
        sistema.carregar();
        sistema
    }

    fn adicionar_produto(&mut self, nome: String, categoria: String, marca: String) {
        let id = self.proximo_id;
        self.proximo_id += 1;
        let produto = Produto { id, nome: nome.clone(), categoria: categoria.clone(), marca: marca.clone() };
        self.produtos.push(produto);

        // Atualiza índices
        self.indice_nome.entry(nome.to_lowercase()).or_default().insert(id);
        self.indice_marca.entry(marca.to_lowercase()).or_default().insert(id);
        self.indice_categoria.entry(categoria.to_lowercase()).or_default().insert(id);

        self.salvar();
        println!("Produto adicionado.");
    }

    fn remover_produto_por_nome(&mut self, nome: &str) {
        let nome_lc = nome.to_lowercase();
        if let Some(ids) = self.indice_nome.get(&nome_lc) {
            let ids_vec: Vec<u32> = ids.iter().copied().collect();
            for id in ids_vec {
                if let Some(pos) = self.produtos.iter().position(|p| p.id == id) {
                    let produto = self.produtos.remove(pos);

                    // Remove dos índices
                    self.indice_nome.get_mut(&produto.nome.to_lowercase()).map(|set| set.remove(&produto.id));
                    self.indice_marca.get_mut(&produto.marca.to_lowercase()).map(|set| set.remove(&produto.id));
                    self.indice_categoria.get_mut(&produto.categoria.to_lowercase()).map(|set| set.remove(&produto.id));
                }
            }
            self.indice_nome.remove(&nome_lc);
            self.salvar();
            println!("Produto(s) removido(s).");
        } else {
            println!("Produto não encontrado.");
        }
    }

    fn buscar_por_nome(&self, nome: &str) {
        let nome_lc = nome.to_lowercase();
        if let Some(ids) = self.indice_nome.get(&nome_lc) {
            for id in ids {
                if let Some(prod) = self.produtos.iter().find(|p| p.id == *id) {
                    println!("{:?}", prod);
                }
            }
        } else {
            println!("Produto não encontrado.");
        }
    }

    fn buscar_por_marca(&self, marca: &str) {
        let marca_lc = marca.to_lowercase();
        if let Some(ids) = self.indice_marca.get(&marca_lc) {
            for id in ids {
                if let Some(prod) = self.produtos.iter().find(|p| p.id == *id) {
                    println!("{:?}", prod);
                }
            }
        } else {
            println!("Nenhum produto encontrado para esta marca.");
        }
    }

    fn buscar_por_categoria(&self, categoria: &str) {
        let categoria_lc = categoria.to_lowercase();
        if let Some(ids) = self.indice_categoria.get(&categoria_lc) {
            for id in ids {
                if let Some(prod) = self.produtos.iter().find(|p| p.id == *id) {
                    println!("{:?}", prod);
                }
            }
        } else {
            println!("Nenhum produto encontrado para esta categoria.");
        }
    }

    fn listar_todos(&self) {
        if self.produtos.is_empty() {
            println!("Nenhum produto cadastrado.");
        } else {
            for produto in &self.produtos {
                println!("{:?}", produto);
            }
        }
    }

    fn salvar(&self) {
        let json = serde_json::to_string_pretty(&self.produtos).expect("Erro ao salvar JSON");
        fs::write("produtos.json", json).expect("Erro ao escrever arquivo");
    }

    fn carregar(&mut self) {
        if let Ok(data) = fs::read_to_string("produtos.json") {
            if let Ok(produtos) = serde_json::from_str::<Vec<Produto>>(&data) {
                self.produtos = produtos;
                // Reconstroi índices e ajusta próximo_id
                self.indice_nome.clear();
                self.indice_marca.clear();
                self.indice_categoria.clear();
                let mut max_id = 0;
                for p in &self.produtos {
                    self.indice_nome.entry(p.nome.to_lowercase()).or_default().insert(p.id);
                    self.indice_marca.entry(p.marca.to_lowercase()).or_default().insert(p.id);
                    self.indice_categoria.entry(p.categoria.to_lowercase()).or_default().insert(p.id);
                    if p.id > max_id {
                        max_id = p.id;
                    }
                }
                self.proximo_id = max_id + 1;
            }
        }
    }
}

fn ler_entrada(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha na leitura");
    entrada.trim().to_string()
}

fn main() {
    let mut sistema = Sistema::new();

    loop {
        println!("\n=== Sistema de Busca MegaStore ===");
        println!("1. Adicionar produto");
        println!("2. Remover produto por nome");
        println!("3. Buscar por nome");
        println!("4. Buscar por marca");
        println!("5. Buscar por categoria");
        println!("6. Listar todos os produtos");
        println!("0. Sair");
        let opcao = ler_entrada("Escolha uma opção: ");

        match opcao.as_str() {
            "1" => {
                let nome = ler_entrada("Nome do produto: ");
                let categoria = ler_entrada("Categoria: ");
                let marca = ler_entrada("Marca: ");
                sistema.adicionar_produto(nome, categoria, marca);
            }
            "2" => {
                let nome = ler_entrada("Nome do produto a remover: ");
                sistema.remover_produto_por_nome(&nome);
            }
            "3" => {
                let nome = ler_entrada("Nome: ");
                sistema.buscar_por_nome(&nome);
            }
            "4" => {
                let marca = ler_entrada("Marca: ");
                sistema.buscar_por_marca(&marca);
            }
            "5" => {
                let categoria = ler_entrada("Categoria: ");
                sistema.buscar_por_categoria(&categoria);
            }
            "6" => {
                sistema.listar_todos();
            }
            "0" => {
                println!("Encerrando sistema...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}
