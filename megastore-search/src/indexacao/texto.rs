pub struct ProcessadorTexto;

impl ProcessadorTexto {
    pub fn new() -> Self {
        Self
    }

    pub fn processar(&self, texto: &str) -> Vec<String> {
        texto.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn processar_termo(&self, termo: &str) -> String {
        termo.to_lowercase().trim().to_string()
    }
}
