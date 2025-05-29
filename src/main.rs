use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
    marca: String,
}

struct Catalogo {
    produtos: Vec<Produto>,
    index_nome: HashMap<String, Vec<usize>>,
    index_marca: HashMap<String, Vec<usize>>,
    index_categoria: HashMap<String, Vec<usize>>,
    proximo_id: u32,
}

impl Catalogo {
    fn new() -> Self {
        Catalogo {
            produtos: Vec::new(),
            index_nome: HashMap::new(),
            index_marca: HashMap::new(),
            index_categoria: HashMap::new(),
            proximo_id: 1,
        }
    }

    // Rebuild all indices from scratch (call após carregar ou alterar lista)
    fn reconstruir_indices(&mut self) {
        self.index_nome.clear();
        self.index_marca.clear();
        self.index_categoria.clear();

        for (pos, produto) in self.produtos.iter().enumerate() {
            self.index_nome.entry(produto.nome.to_lowercase()).or_default().push(pos);
            self.index_marca.entry(produto.marca.to_lowercase()).or_default().push(pos);
            self.index_categoria.entry(produto.categoria.to_lowercase()).or_default().push(pos);
        }
    }

    fn adicionar_produto(&mut self, nome: String, categoria: String, marca: String) {
        let produto = Produto {
            id: self.proximo_id,
            nome: nome.trim().to_string(),
            categoria: categoria.trim().to_string(),
            marca: marca.trim().to_string(),
        };
        self.proximo_id += 1;

        self.produtos.push(produto);
        self.reconstruir_indices();
    }

    fn remover_por_nome(&mut self, nome: &str) -> bool {
        let nome = nome.to_lowercase();
        if let Some(posicoes) = self.index_nome.get(&nome) {
            if !posicoes.is_empty() {
                let idx = posicoes[0];
                self.produtos.remove(idx);
                self.reconstruir_indices();
                return true;
            }
        }
        false
    }

    fn buscar_por_nome(&self, nome: &str) -> Vec<&Produto> {
        let chave = nome.to_lowercase();
        if let Some(posicoes) = self.index_nome.get(&chave) {
            posicoes.iter().map(|&i| &self.produtos[i]).collect()
        } else {
            Vec::new()
        }
    }

    fn buscar_por_marca(&self, marca: &str) -> Vec<&Produto> {
        let chave = marca.to_lowercase();
        if let Some(posicoes) = self.index_marca.get(&chave) {
            posicoes.iter().map(|&i| &self.produtos[i]).collect()
        } else {
            Vec::new()
        }
    }

    fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        let chave = categoria.to_lowercase();
        if let Some(posicoes) = self.index_categoria.get(&chave) {
            posicoes.iter().map(|&i| &self.produtos[i]).collect()
        } else {
            Vec::new()
        }
    }

    fn listar_todos(&self) -> &Vec<Produto> {
        &self.produtos
    }

    fn salvar_em_arquivo(&self, caminho: &str) -> io::Result<()> {
        let arquivo = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(caminho)?;

        let escritor = BufWriter::new(arquivo);
        serde_json::to_writer_pretty(escritor, &self.produtos)?;
        Ok(())
    }

    fn carregar_de_arquivo(&mut self, caminho: &str) -> io::Result<()> {
        let arquivo = File::open(caminho)?;
        let leitor = BufReader::new(arquivo);
        let dados: Vec<Produto> = serde_json::from_reader(leitor)?;
        self.produtos = dados;
        self.proximo_id = self.produtos.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        self.reconstruir_indices();
        Ok(())
    }
}

fn ler_entrada(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().expect("Falha ao limpar buffer de saída");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler entrada");
    entrada.trim().to_string()
}

fn main() {
    let mut catalogo = Catalogo::new();
    let caminho_arquivo = "produtos.json";

    // Tenta carregar dados salvos ao iniciar
    if let Err(_) = catalogo.carregar_de_arquivo(caminho_arquivo) {
        println!("Nenhum arquivo de dados encontrado, iniciando catálogo vazio.");
    }

    loop {
        println!("\n=== Sistema de Busca MegaStore ===");
        println!("1. Adicionar produto");
        println!("2. Remover produto por nome");
        println!("3. Buscar por nome");
        println!("4. Buscar por marca");
        println!("5. Buscar por categoria");
        println!("6. Listar todos os produtos");
        println!("0. Sair");

        let escolha = ler_entrada("Escolha uma opção: ");
        match escolha.as_str() {
            "1" => {
                let nome = ler_entrada("Nome do produto: ");
                let categoria = ler_entrada("Categoria: ");
                let marca = ler_entrada("Marca: ");

                if nome.is_empty() || categoria.is_empty() || marca.is_empty() {
                    println!("Erro: Nenhum campo pode ficar vazio.");
                } else {
                    catalogo.adicionar_produto(nome, categoria, marca);
                    catalogo.salvar_em_arquivo(caminho_arquivo).unwrap_or_else(|e| println!("Erro ao salvar arquivo: {}", e));
                    println!("Produto adicionado.");
                }
            }
            "2" => {
                let nome = ler_entrada("Nome do produto a remover: ");
                if catalogo.remover_por_nome(&nome) {
                    catalogo.salvar_em_arquivo(caminho_arquivo).unwrap_or_else(|e| println!("Erro ao salvar arquivo: {}", e));
                    println!("Produto removido.");
                } else {
                    println!("Produto não encontrado.");
                }
            }
            "3" => {
                let nome = ler_entrada("Busca por nome: ");
                let resultados = catalogo.buscar_por_nome(&nome);
                if resultados.is_empty() {
                    println!("Nenhum produto encontrado.");
                } else {
                    for produto in resultados {
                        println!("- {:?}", produto);
                    }
                }
            }
            "4" => {
                let marca = ler_entrada("Busca por marca: ");
                let resultados = catalogo.buscar_por_marca(&marca);
                if resultados.is_empty() {
                    println!("Nenhum produto encontrado.");
                } else {
                    for produto in resultados {
                        println!("- {:?}", produto);
                    }
                }
            }
            "5" => {
                let categoria = ler_entrada("Busca por categoria: ");
                let resultados = catalogo.buscar_por_categoria(&categoria);
                if resultados.is_empty() {
                    println!("Nenhum produto encontrado.");
                } else {
                    for produto in resultados {
                        println!("- {:?}", produto);
                    }
                }
            }
            "6" => {
                let todos = catalogo.listar_todos();
                if todos.is_empty() {
                    println!("Nenhum produto cadastrado.");
                } else {
                    for produto in todos {
                        println!("- {:?}", produto);
                    }
                }
            }
            "0" => {
                println!("Encerrando sistema...");
                break;
            }
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
