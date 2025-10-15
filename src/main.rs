// src/main.rs

// 1. Importa as funções diretamente da nossa crate (expostas via src/lib.rs)
use megastore_busca_otimizada::{
    build_system_from_data, 
    search_by_term, 
    recommend_for_user
};

fn main() {
    println!("Inicializando o Sistema de Busca da MegaStore...");
    
    // 1. Constrói o sistema de busca completo (Grafo + HashMaps)
    // Chamada simplificada, sem o prefixo 'graph_builder::'
    let search_system = build_system_from_data(); 
    
    println!("\nSistema de Busca Carregado com Sucesso!");
    println!("Total de nós no Grafo: {}", search_system.graph.node_count());

    // =================================================================
    // 2. EXEMPLOS DE USO DE BUSCA POR TERMO
    // =================================================================

    // Exemplo A: Busca por termo 'TV'
    let query_a = "TV";
    // Chamada simplificada, sem o prefixo 'search::'
    let results_a = search_by_term(&search_system, query_a); 
    
    println!("\nResultados de BUSCA para '{}':", query_a);
    for product in results_a {
        println!("  -> {} (ID: {})", product.name, product.id);
    }
    
    // Exemplo B: Busca por termo 'roupa' (tag)
    let query_b = "roupa";
    let results_b = search_by_term(&search_system, query_b);

    println!("\nResultados de BUSCA para '{}':", query_b);
    for product in results_b {
        println!("  -> {} (ID: {})", product.name, product.id);
    }
    
    // =================================================================
    // 3. EXEMPLOS DE RECOMENDAÇÃO
    // =================================================================

    // Cliente 1 (Alice)
    let user_id_a = 1;
    // Chamada simplificada, sem o prefixo 'recommender::'
    let recommendations_a = recommend_for_user(&search_system, user_id_a);
    
    println!("\nRECOMENDAÇÕES FINAIS para Alice (ID {}):", user_id_a);
    for product in recommendations_a {
        println!("  -> {} (ID: {})", product.name, product.id);
    }
    
    // Cliente 3 (Charlie)
    let user_id_c = 3;
    let recommendations_c = recommend_for_user(&search_system, user_id_c);

    println!("\nRECOMENDAÇÕES FINAIS para Charlie (ID {}):", user_id_c);
    for product in recommendations_c {
        println!("  -> {} (ID: {})", product.name, product.id);
    }
}