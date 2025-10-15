# 🛒 MegaStore Busca Otimizada e Sistema de Recomendação

Este projeto implementa um sistema de busca por termo e um módulo de recomendação de produtos utilizando **Grafos Direcionados** em Rust. O objetivo é demonstrar a eficiência de estruturas de dados baseadas em grafos para resolver problemas complexos de indexação e relacionamento de dados em um ambiente de e-commerce.

---

### 🚀 Tecnologias e Ferramentas

| Ferramenta | Descrição |
| :--- | :--- |
| **Rust** | Linguagem de programação robusta, utilizada para garantir segurança e alta performance. |
| **Cargo** | O gerenciador de pacotes e sistema de build do Rust, essencial para compilação e execução de testes. |
| **Petgraph** | Crate (biblioteca) especializada em manipulação de grafos e aplicação de algoritmos de teoria dos grafos. |

---

### 🛠️ Como Clonar e Executar o Projeto

Siga os passos abaixo para compilar e rodar o projeto em sua máquina.

#### Pré-requisitos

* **Rust e Cargo:** Instale o Rust através do [rustup](https://rustup.rs/).
* **Ferramentas de Build C++:** Essenciais para a compilação do Rust no Windows (Geralmente instaladas via Visual Studio Build Tools).

#### Instruções

1.  **Clone o Repositório** (Substitua `[SEU_LINK_DO_REPOSITORIO_AQUI]` pelo link do seu repositório público do GitHub):
    ```bash
    git clone [SEU_LINK_DO_REPOSITORIO_AQUI]
    cd megastore_busca_otimizada
    ```

2.  **Compile o Projeto:**
    ```bash
    cargo build
    ```

3.  **Execute o Programa:**
    O programa `main.rs` executa demonstrações de Busca por Termo (`TV`, `roupa`) e Recomendações para os usuários (Alice e Charlie).
    ```bash
    cargo run
    ```
    *(A saída mostrará os resultados do grafo, da busca BFS e das recomendações de filtro colaborativo.)*

4.  **Execute os Testes de Integração:**
    Os testes confirmam que a construção do grafo, o algoritmo de busca BFS e o algoritmo de recomendação estão funcionando conforme o esperado.
    ```bash
    cargo test
    ```

---

### 📂 Estrutura de Arquivos

A organização do código em módulos facilita a manutenção e a separação de responsabilidades:

| Arquivo/Módulo | Descrição |
| :--- | :--- |
| `src/main.rs` | Ponto de entrada (CLI) e demonstração do sistema. |
| `src/lib.rs` | Biblioteca principal que expõe os módulos para que o `main.rs` e os testes (`tests/`) possam utilizá-los. |
| `src/models.rs` | Define as estruturas de dados: `Product`, `User`, `EdgeType`, `NodeType` e `SearchSystem`. |
| `src/graph_builder.rs` | Contém a lógica de inicialização, criação dos nós e arestas, e indexação dos dados iniciais. |
| `src/search.rs` | Implementa o algoritmo **BFS** para busca por termo. |
| `src/recommender.rs` | Implementa o algoritmo de **Filtro Colaborativo** (Graph Traversal) para recomendação. |
| `tests/integration_tests.rs`| Contém os testes de validação para a busca e a recomendação. |

---

### 🧠 Algoritmos e Estruturas de Dados Utilizados

A arquitetura do sistema é fundamentada em estruturas de dados otimizadas, garantindo eficiência, escalabilidade e precisão.

#### Estruturas de Dados

* **Estrutura Principal:** **Grafo Direcionado Ponderado** (`petgraph::DiGraph<GraphNode, EdgeType>`).
    * **Nós (Vértices):** Representam três entidades: **`Product`**, **`User`** e **`Term`** (Palavra-chave/Tag).
    * **Arestas (Relacionamentos):** Definem a rede de interações, como `BOUGHT` (compra), `VIEWED` (visualização) e similaridades (`RelatedByCategory`, `ContainsTerm`).

* **Tabelas Hash para Acesso Rápido:** **`std::collections::HashMap<u32, NodeIndex>`**
    * **Finalidade:** Utilizada na estrutura `SearchSystem` para mapear os **IDs externos** de Produto e Usuário para seus respectivos **índices internos** no grafo.
    * **Eficiência:** Garante tempo de acesso constante (**O(1)**) para iniciar qualquer busca ou recomendação a partir de um ID conhecido.

#### Algoritmos de Busca e Recomendação

* **Algoritmo de Busca por Termo:** **Breadth-First Search (BFS)**
    * **Processo:** O algoritmo inicia a busca a partir dos nós **`Term`** (palavras-chave da consulta), navegando pelas arestas de indexação (`ContainsTerm`) até os nós **`Product`**.
    * **Vantagem:** Simula um **índice reverso eficiente** e garante que todos os produtos conectados ao termo sejam encontrados em tempo hábil.

* **Algoritmo de Recomendação:** **Filtro Colaborativo Baseado em Vizinhos (Graph Traversal)**
    * **Finalidade:** Aumentar a **relevância** sugerindo produtos que o cliente **A** ainda não interagiu.
    * **Processo:**
        1.  Navega a partir do `User A` para identificar seu histórico (`BOUGHT`/`VIEWED`).
        2.  Identifica **clientes vizinhos** que interagiram com itens semelhantes no histórico de A.
        3.  Coleta produtos comprados/vistos por esses vizinhos.
        4.  Filtra para remover itens já vistos por A e ordena por **frequência de interação dos vizinhos** (relevância).