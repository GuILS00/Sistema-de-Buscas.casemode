# Sistema de Busca MegaStore

Este é um sistema de busca otimizado para o catálogo de produtos da **MegaStore**, desenvolvido em Rust como parte de um projeto acadêmico. Ele foi projetado para atender aos seguintes critérios:

## ✔️ Requisitos atendidos

### ✅ Busca otimizada
- **Busca por nome, marca e categoria**, com estrutura de dados eficiente baseada em índices (HashMaps).
- Resposta imediata mesmo com grande volume de dados.

### ✅ Indexação eficiente
- Produtos são automaticamente indexados por nome, marca e categoria no momento da adição.
- Remoções também atualizam os índices dinamicamente.

### ✅ Escalabilidade
- Código modular e eficiente.
- Preparado para expansão (ex: mais campos ou filtros).
- Suporte à persistência em arquivo JSON (`produtos.json`).

### ✅ Segurança dos dados
- Armazenamento local dos produtos com leitura e escrita automática no início e fim do programa.
- Evita perda de dados com persistência em disco.

---

## 🚀 Funcionalidades

- Adicionar produto
- Remover produto por nome
- Buscar produto por:
  - Nome
  - Marca
  - Categoria
- Listar todos os produtos cadastrados
- Salvar e carregar produtos automaticamente

---

## 🧠 Tecnologias utilizadas

- Linguagem: **Rust**
- Persistência: `serde` e `serde_json`
- Estruturas: `HashMap` para indexação
- Interface: Terminal interativo (via `stdin`/`stdout`)

---

## 🗃️ Execução local

1. Clone o repositório:

```bash
git clone https://github.com/GuILS00/Sistema-de-Buscas.casemode.git

cd Sistema-de-Buscas.casemode

cargo run

sistema_busca_megastore/
├── src/
│   └── main.rs
├── produtos.json       # Base de dados persistente
├── Cargo.toml          # Dependências
├── Cargo.lock
└── README.md           # Este arquivo


Os dados são automaticamente carregados do arquivo produtos.json ao iniciar e salvos ao sair.

O sistema trata remoções e buscas com eficiência mesmo com grande volume de produtos.

GuILS00
GitHub: GuILS00

## 🖥️ Demonstração do Sistema

Abaixo estão capturas de tela reais da execução do sistema, demonstrando suas principais funcionalidades:

### 📌 Execução Inicial (cargo run) e Interface
Mostra a interface do sistema ao iniciar pelo terminal.

![Interface inicial](imgs/cargo%20run.interface.PNG)

---

### ➕ Adição de Produtos e Listagem
Demonstra a adição de produtos e a listagem completa atualizada.

![Adição e listagem de produtos](imgs/adição%20de%20produtos.lista%20de%20produtos.PNG)

---

### 🔍 Busca Múltipla (nome, marca e categoria)
Demonstra o funcionamento das buscas com resultados imediatos.

![Busca por nome, marca e categoria](imgs/busca%20multipla.PNG)

---

### ❌ Remoção e Checagem
Exemplo de remoção de produto e confirmação da operação.

![Remoção de produtos](imgs/remoção%20e%20checagem.PNG)

---

### 💾 Persistência (Memória mantida após reinício)
Prova que os dados inseridos são mantidos após reinicialização do sistema.

![Persistência dos dados](imgs/memória%20mantida.PNG)
