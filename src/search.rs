use crate::models::{SearchSystem, Product, GraphNode, EdgeType};
use std::collections::HashSet;
use petgraph::visit::Bfs;

/// Encontra produtos relevantes com base em um termo de busca, usando BFS no grafo.
pub fn search_by_term(system: &SearchSystem, query: &str) -> Vec<Product> {
    let graph = &system.graph;
    let query = query.to_lowercase();
    let mut results_indices = HashSet::new();

    println!("\n--- Executando Busca por Termo: '{}' ---", query);

    // 1. Encontrar o(s) nó(s) de Termo de Busca relevantes
    let start_indices: Vec<_> = graph.node_indices()
        .filter(|index| {
            if let Some(GraphNode::Term(term)) = graph.node_weight(*index) {
                query.contains(&term.term) || term.term.contains(&query)
            } else {
                false
            }
        })
        .collect();

    if start_indices.is_empty() {
        println!("Nenhum termo de busca encontrado no grafo para '{}'.", query);
        return Vec::new();
    }
    
    // 2. Executar BFS (Busca em Largura) a partir de cada nó de termo
    for start_node in start_indices {
        let mut bfs = Bfs::new(graph, start_node);

        while let Some(visited_node) = bfs.next(graph) {
            // Se o nó visitado for um Produto...
            if let Some(GraphNode::Product(product)) = graph.node_weight(visited_node) {
                // Checamos se a aresta é de indexação (Termo -> Produto)
                let is_index_edge = graph.edges_connecting(start_node, visited_node)
                    .any(|edge| edge.weight() == &EdgeType::CONTAINS_TERM);

                if is_index_edge {
                    results_indices.insert(product.clone());
                }
            }
        }
    }

    // 3. Conversão para Vec<Product> e ordenação simples
    let mut final_results = results_indices.into_iter().collect::<Vec<_>>();
    final_results.sort_by_key(|p| p.id);
    
    println!("Busca finalizada. Encontrados {} resultados.", final_results.len());
    
    final_results
}