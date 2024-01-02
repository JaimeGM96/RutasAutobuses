#[cfg(test)]
mod tests {
    use buscador_rutas::*;
    use chrono::NaiveTime;
    use std::collections::HashMap as Map; 
    use std::fs;

    fn create_linea(paradas: Vec<i64>) -> Linea {
        let file: String = fs::read_to_string("data/data.json").expect("Failed to read file");
        let mut json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not well-formatted");

        let id_linea = json["lineas"][0]["id"].take().as_i64().unwrap();
        let binding = json["lineas"][0]["paradas"].take();
        let paradas_json = binding.as_array().unwrap();
        let primera_parada = paradas_json[0].as_i64().unwrap();
        let segunda_parada = paradas_json[1].as_i64().unwrap();
        let tercera_parada = paradas_json[2].as_i64().unwrap();

        let mut horarios = Map::new();

        for parada in &paradas {
            let hora = json["lineas"][0]["horarios"][parada.to_string()]["horas"].take().as_i64().unwrap() as u32;
            let minuto = json["lineas"][0]["horarios"][parada.to_string()]["minutos"].take().as_i64().unwrap() as u32;
            let segundo = json["lineas"][0]["horarios"][parada.to_string()]["segundos"].take().as_i64().unwrap() as u32;

            horarios.insert(*parada, vec![NaiveTime::from_hms(
                hora,
                minuto,
                segundo
            )]);
        }

        let linea = Linea {
            id: id_linea,
            paradas: vec![
                primera_parada,
                segunda_parada,
                tercera_parada,
            ],
            horarios: horarios,
        };

        linea
    }

    #[test]
    fn test_a_linea_can_be_created() {
        let linea = create_linea(vec![102]);

        assert_eq!(linea.id, 1);
        assert_eq!(linea.paradas.len(), 3);
        assert_eq!(linea.horarios.len(), 1);
    }

    #[test]
    fn test_a_transbordo_can_be_created() {
        let file: String = fs::read_to_string("data/data.json").expect("Failed to read file");
        let mut json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not well-formatted");

        let id_linea = json["lineas"][0]["id"].take().as_i64().unwrap();
        let parada = json["lineas"][0]["paradas"].take().as_array().unwrap()[0].as_i64().unwrap();
        let horas = json["lineas"][0]["horarios"]["101"]["horas"].take().as_i64().unwrap() as u32;
        let minutos = json["lineas"][0]["horarios"]["101"]["minutos"].take().as_i64().unwrap() as u32;
        let segundos = json["lineas"][0]["horarios"]["101"]["segundos"].take().as_i64().unwrap() as u32;

        let transbordo = Transbordo {
            linea: id_linea,
            parada: parada,
            hora: NaiveTime::from_hms(
                horas,
                minutos,
                segundos
            ),
        };

        assert_eq!(transbordo.linea, id_linea);
        assert_eq!(transbordo.parada, parada);
        assert_eq!(transbordo.hora, NaiveTime::from_hms(
            horas,
            minutos,
            segundos
        ));
    }

    #[test]
    fn test_a_buscador_rutas_can_be_created() {
        let linea = create_linea(vec![102]);

        let mut paradas = Map::new();
        paradas.insert(101, vec![1]);
        paradas.insert(102, vec![1, 2]);

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas).unwrap();

        assert_eq!(buscador_rutas.lineas.len(), 1);
        assert_eq!(buscador_rutas.paradas.len(), 2);
    }

    #[test]
    fn test_a_buscador_rutas_cannot_be_created_without_paradas() {
        let linea = create_linea(vec![]);
        let paradas = Map::new();

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas);

        assert_eq!(buscador_rutas.err(), Some("No se puede crear un buscador de rutas sin paradas"));
    }

    #[test]
    fn test_a_buscador_rutas_cannot_be_created_without_lineas() {
        let mut paradas = Map::new();
        paradas.insert(101, vec![1]);
        paradas.insert(102, vec![1, 2]);

        let buscador_rutas = BuscadorRutas::new(vec![], paradas);

        assert_eq!(buscador_rutas.err(), Some("No se puede crear un buscador de rutas sin lineas"));
    }

    #[test]
    pub fn test_given_a_buscador_rutas_can_get_rutas() {
        let linea = create_linea(vec![101, 102, 103]);

        let mut paradas = Map::new();
        paradas.insert(101, vec![1]);
        paradas.insert(102, vec![1, 2]);

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas);

        let rutas = buscador_rutas.unwrap().encuentra(NaiveTime::from_hms(7, 0, 0), 101, 103);

        assert_eq!(rutas.unwrap().len(), 1);
    }
}