use serde::Deserialize;
// parametros para paginar la lista

#[derive(Deserialize)]
#[serde(default)]
pub struct ParametrosList {
    pub pagina: i64,
    pub filtro: String,
    pub longitud: String,
}
impl Default for ParametrosList {
    fn default() -> Self {
        Self {
            pagina: 1,
            filtro: String::new(),
            longitud: String::from("10"),
        }
    }
}

impl ParametrosList {
    // calcula el numero de paginas
    pub fn get_paginas(&self, nro_registros: i64) -> i64 {
        if self.longitud == "todos" {
            1
        } else {
            let l = self.longitud.parse::<i64>().unwrap();
            if nro_registros % l > 0 {
                nro_registros / l + 1
            } else {
                nro_registros / l
            }
        }
    }

    // fn calcula el numero de tomos = conjuntos de 10 paginas
    pub fn get_tomos(paginas: i64, longitud_tomo: i64) -> i64 {
        if paginas % longitud_tomo > 0 {
            paginas / longitud_tomo + 1
        } else {
            paginas / longitud_tomo
        }
    }
}
