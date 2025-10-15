use crate::models::{SearchSystem, Product, GraphNode, EdgeType};
use petgraph::Direction;
use std::collections::{HashSet, HashMap};
use petgraph::visit::EdgeRef;

/// Encontra recomendações de produtos para um usuário específico.
/// 
/// Algoritmo Simplificado: Recomendação Baseada em Clientes Vizinhos (Collaborative Filtering).
pub fn recommend_for_user(system: &SearchSystem, user_id: u32) -> Vec<Product> {
    let graph = &system.graph;
    
    // 1. Encontrar o índice do usuário de partida (Usando o HashMap O(1))
    let user_index = match system.user_id_to_index.get(&user_id) {
        Some(index) => *index,
        None => {
            println!("\nUsuário ID {} não encontrado no sistema.", user_id);
            return Vec::new();
        }
    };
    println!("\n--- Gerando Recomendações para Usuário ID: {} ---", user_id);

    // Conjunto para armazenar produtos que o usuário JÁ viu/comprou
    let mut user_history = HashSet::new();
    // Mapa para armazenar produtos que os vizinhos interagiram, e a contagem de vizinhos
    let mut neighbor_product_counts: HashMap<u32, i32> = HashMap::new();
    
    // 2. Encontrar o histórico do usuário (Produtos que ele interagiu)
    for edge in graph.edges_directed(user_index, Direction::Outgoing) {
        let neighbor_index = edge.target();
        // Arestas de interesse: BOUGHT ou VIEWED
        if edge.weight() == &EdgeType::BOUGHT || edge.weight() == &EdgeType::VIEWED {
             if let Some(GraphNode::Product(product)) = graph.node_weight(neighbor_index) {
                user_history.insert(product.id);
            }
        }
    }
    
    // 3. Encontrar Clientes Vizinhos (Vizinhos de Produtos comprados/vistos pelo Cliente A)
    // Para simplificar, vamos encontrar vizinhos de produtos no histórico
    for &product_id in &user_history {
        let product_index = *system.product_id_to_index.get(&product_id).unwrap();

        // Encontrar vizinhos do PRODUTO (outros usuários que o compraram)
        for edge in graph.edges_directed(product_index, Direction::Incoming) {
            let neighbor_index = edge.source();

            // Se o vizinho é um Usuário (e não o próprio User A)
            if let Some(GraphNode::User(neighbor_user)) = graph.node_weight(neighbor_index) {
                if neighbor_user.id != user_id {
                    
                    // 4. Encontrar os produtos que esses vizinhos compraram/viram
                    for neighbor_edge in graph.edges_directed(neighbor_index, Direction::Outgoing) {
                        let recommended_product_index = neighbor_edge.target();
                        
                        // Garante que é uma interação BOUGHT/VIEWED e que o nó é um Produto
                        if (neighbor_edge.weight() == &EdgeType::BOUGHT || neighbor_edge.weight() == &EdgeType::VIEWED) 
                           && graph.node_weight(recommended_product_index).map(|n| n.get_type()) == Some(crate::models::NodeType::Product)
                        {
                            if let Some(GraphNode::Product(rec_product)) = graph.node_weight(recommended_product_index) {
                                // 5. Filtrar: Só recomenda se o User A não tiver visto/comprado
                                if !user_history.contains(&rec_product.id) {
                                    // Aumenta a contagem de relevância
                                    *neighbor_product_counts.entry(rec_product.id).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

   // 6. Converter resultados SEM USAR into_iter() para preservar o HashMap
    let mut recommendations_temp: Vec<(Product, i32)> = neighbor_product_counts.iter()
        .map(|(id, count)| {
            // Buscamos o objeto Product real
            let index = system.product_id_to_index.get(id).unwrap();
            if let Some(GraphNode::Product(p)) = graph.node_weight(*index) {
                println!("  [Relevância: {}] -> {} (ID: {})", count, p.name, p.id);
                (p.clone(), *count) // Retorna o Produto e a contagem de relevância
            } else {
                unreachable!() // Não deve acontecer
            }
        })
        .collect();
        
    // 7. Ordenar de forma decrescente pela contagem (o elemento `count` está na tupla)
    // Usamos `sort_by` porque é mais fácil para ordenar tuplas.
    recommendations_temp.sort_by(|a, b| b.1.cmp(&a.1)); // Compara o count (o segundo elemento da tupla)

    recommendations_temp.truncate(5); // Limita as 5 melhores recomendações
    
    // Converte de volta para Vec<Product>
    let recommendations: Vec<Product> = recommendations_temp.into_iter().map(|(p, _)| p).collect();

    if recommendations.is_empty() {
        println!("  Nenhuma recomendação nova baseada em vizinhos encontrada.");
    }
    
    recommendations
}