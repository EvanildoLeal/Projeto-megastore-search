use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::time::{Instant, Duration};

pub struct CacheBusca {
    cache: Arc<RwLock<LruCache<String, (Vec<u32>, Instant)>>>,
    tempo_vida: Duration,
}

impl CacheBusca {
    pub fn new(capacidade: usize, tempo_vida: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(capacidade).unwrap()
            ))),
            tempo_vida,
        }
    }

    pub fn obter(&self, chave: &str) -> Option<Vec<u32>> {
        let mut cache = self.cache.write().unwrap();
        if let Some((resultados, timestamp)) = cache.get(chave) {
            if timestamp.elapsed() < self.tempo_vida {
                return Some(resultados.clone());
            }
            cache.pop(chave); // Remove se expirado
        }
        None
    }

    pub fn inserir(&self, chave: String, resultados: Vec<u32>) {
        let mut cache = self.cache.write().unwrap();
        cache.put(chave, (resultados, Instant::now()));
    }

    pub fn limpar(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }

    pub fn estatisticas(&self) -> usize {
        let cache = self.cache.read().unwrap();
        cache.len()
    }
}
