// src/graph_builder.rs

use crate::models::{
    MegaStoreGraph, SearchSystem, Product, User, 
    GraphNode, EdgeType, Term, 
};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

/// Função que carrega dados simulados e constrói o sistema de busca.
// PRECISA DE 'pub'
pub fn build_system_from_data() -> SearchSystem { 
    // ... (restante da lógica do grafo)
    let mut graph = MegaStoreGraph::new();
    let mut product_id_to_index = HashMap::new();
    let mut user_id_to_index = HashMap::new();

    // --- DADOS SIMULADOS ---
    let products = create_mock_products();
    let users = create_mock_users();
    let interactions = create_mock_interactions();
    // ---------------------------------------------------

    // 2. ADICIONA NÓS DE PRODUTO e preenche o HashMap de produtos
    for p in products {
        let id = p.id;
        let index = graph.add_node(GraphNode::Product(p));
        product_id_to_index.insert(id, index);
    }

    // 3. ADICIONA NÓS DE CLIENTE e preenche o HashMap de usuários
    for u in users {
        let id = u.id;
        let index = graph.add_node(GraphNode::User(u));
        user_id_to_index.insert(id, index);
    }

    // 4. ADICIONA INTERAÇÕES (ARESTAS User -> Product)
    for (user_id, product_id, edge_type) in interactions {
        let user_index = *user_id_to_index.get(&user_id).expect("User not found");
        let product_index = *product_id_to_index.get(&product_id).expect("Product not found");

        graph.add_edge(user_index, product_index, edge_type);
    }

    // 5. CRIA NÓS DE TERMOS DE BUSCA (Indexação) e arestas de relacionamentos Item-Item
    let mut term_to_index: HashMap<String, NodeIndex> = HashMap::new();

    // Iteramos sobre todos os produtos já criados
    for (product_id, product_node_index) in product_id_to_index.clone() { // <-- CLONA O MAPA DE ÍNDICES!
        
        // 1. Extrai o dado do Produto (fazendo uma cópia ou clone do dado dentro do nó)
        let product = if let Some(GraphNode::Product(p)) = graph.node_weight(product_node_index) {
            p.clone() // <-- CLONE O DADO AQUI, para que o borrow imutável não dure
        } else {
            continue;
        };
        
        // Indexação por Termo (agora sem borrow imutável do grafo)
        let terms: Vec<String> = product.name.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
        let all_terms = [terms, product.tags.clone()].concat();

        for term_str in all_terms {
            let term_str = term_str.to_lowercase();
            
            // Pega ou cria o Nó de Termo de Busca
            let term_index = *term_to_index.entry(term_str.clone())
                .or_insert_with(|| graph.add_node(GraphNode::Term(Term { term: term_str }))); // Mutável OK

            // Cria a aresta de indexação (Term -> Produto)
            graph.add_edge(term_index, product_node_index, EdgeType::CONTAINS_TERM); // Mutável OK
        }

        // Relacionamentos Item-Item (Produto <-> Produto) - AGORA COM O CLONE DO DADO 'product'
        for (other_id, other_index) in &product_id_to_index {
            if product.id != *other_id {
                 if let Some(GraphNode::Product(other_product)) = graph.node_weight(*other_index) {
                    if product.category == other_product.category {
                        graph.add_edge(product_node_index, *other_index, EdgeType::RELATED_BY_CATEGORY);
                    }
                }
            }
        }
    }

    println!("Grafo construído com sucesso! Nós: {}, Arestas: {}", graph.node_count(), graph.edge_count());

    // 6. Retorna a estrutura completa
    SearchSystem {
        graph,
        product_id_to_index,
        user_id_to_index,
    }
}

// --- Funções Auxiliares (Privadas) ---

fn create_mock_products() -> Vec<Product> {
    vec![
        Product { id: 101, name: "Smart TV 4K LG 55".to_string(), brand: "LG".to_string(), category: "Eletrônicos".to_string(), tags: vec!["tv".to_string(), "smart".to_string()] },
        Product { id: 102, name: "Soundbar JBL 5.1".to_string(), brand: "JBL".to_string(), category: "Eletrônicos".to_string(), tags: vec!["audio".to_string(), "speaker".to_string()] },
        Product { id: 103, name: "Vestido Floral Verão".to_string(), brand: "Zara".to_string(), category: "Vestuário".to_string(), tags: vec!["roupa".to_string(), "verao".to_string()] },
        Product { id: 104, name: "Smart TV QLED Samsung 65".to_string(), brand: "Samsung".to_string(), category: "Eletrônicos".to_string(), tags: vec!["tv".to_string(), "qled".to_string()] },
        Product { id: 105, name: "Calça Jeans Slim Fit".to_string(), brand: "Levi's".to_string(), category: "Vestuário".to_string(), tags: vec!["jeans".to_string(), "calca".to_string()] },
    ]
}

fn create_mock_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
        User { id: 3, name: "Charlie".to_string() },
    ]
}

fn create_mock_interactions() -> Vec<(u32, u32, EdgeType)> {
    // (user_id, product_id, edge_type)
    vec![
        (1, 101, EdgeType::BOUGHT),  // Alice comprou TV LG
        (1, 102, EdgeType::VIEWED),  // Alice viu Soundbar
        (2, 104, EdgeType::BOUGHT),  // Bob comprou TV Samsung
        (2, 102, EdgeType::BOUGHT),  // Bob comprou Soundbar
        (3, 103, EdgeType::BOUGHT),  // Charlie comprou Vestido
        (3, 105, EdgeType::VIEWED),  // Charlie viu Calça Jeans
    ]
}