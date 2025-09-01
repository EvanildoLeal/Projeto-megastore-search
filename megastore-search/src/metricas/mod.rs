use std::time::Duration;

pub struct MetricasBusca {
    pub tempo_resposta: Duration,
    pub resultados_encontrados: usize,
    pub consultas_realizadas: u32,
}

impl MetricasBusca {
    pub fn new() -> Self {
        Self {
            tempo_resposta: Duration::default(),
            resultados_encontrados: 0,
            consultas_realizadas: 0,
        }
    }

    pub fn registrar_consulta(&mut self, resultados: usize, tempo: Duration) {
        self.consultas_realizadas += 1;
        self.resultados_encontrados = resultados;
        self.tempo_resposta = tempo;
    }

    pub fn exibir_metricas(&self) {
        println!("Consultas realizadas: {}", self.consultas_realizadas);
        println!("Última consulta - Resultados: {}, Tempo: {:?}", 
                 self.resultados_encontrados, self.tempo_resposta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_registro_metricas() {
        let mut metricas = MetricasBusca::new();
        
        metricas.registrar_consulta(5, Duration::from_micros(100));
        assert_eq!(metricas.consultas_realizadas, 1);
        assert_eq!(metricas.resultados_encontrados, 5);
        assert_eq!(metricas.tempo_resposta.as_micros(), 100);
    }

    #[test]
    fn test_multiplas_metricas() {
        let mut metricas = MetricasBusca::new();
        
        metricas.registrar_consulta(1, Duration::from_micros(50));
        metricas.registrar_consulta(2, Duration::from_micros(75));
        
        assert_eq!(metricas.consultas_realizadas, 2);
        assert_eq!(metricas.resultados_encontrados, 2); // Último registro
    }
}
