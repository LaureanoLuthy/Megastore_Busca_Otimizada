// src/lib.rs

// As funções e structs públicas que queremos expor ao binário (main.rs) e aos testes
pub mod models; 
pub mod graph_builder; 
pub mod search; 
pub mod recommender;

// Re-exporta as funções principais no nível da crate para fácil acesso nos testes
pub use graph_builder::build_system_from_data;
pub use search::search_by_term;
pub use recommender::recommend_for_user;