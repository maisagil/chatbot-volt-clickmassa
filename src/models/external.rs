use serde::{Deserialize, Serialize};

// HIGHCONSULT (Dados de CPF)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HighConsultResponse {
    pub nome: String,
    pub nasc: String,      // YYYYMMDD
    pub mae: String,
    pub endereco: String,
    pub cidade: String,
    pub uf: String,
    pub email: Option<String>,
    pub cep: String,
    pub bairro: String,
}

// VIACEP (Dados de Endereço)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViaCepResponse {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
    pub ibge: Option<String>,
    pub gia: Option<String>,
    pub ddd: Option<String>,
    pub siafi: Option<String>,
}
