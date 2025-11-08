use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// VALIDAÇÃO DE CPF

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ValidarCpfRequest {
    pub cpf: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ValidarCpfResponse {
    pub valido: bool,
    pub cpf_formatado: Option<String>,
    pub mensagem: String,
}

// TERMO

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CriarTermoRequest {
    pub cpf: String,
    pub telefone: String, // formato: 11984353470
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CriarTermoResponse {
    pub termo_id: String,
    pub status: String,
    pub mensagem: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AutorizarTermoRequest {
    pub termo_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AutorizarTermoResponse {
    pub consult_id: String,
    pub nome: String,
    pub margem_disponivel: String,
    pub parcelas_min: i32,
    pub parcelas_max: i32,
    pub status: String,
    pub mensagem: String,
}

// SIMULAÇÃO

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GerarSimulacoesRequest {
    pub consult_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SimulacaoResumo {
    pub parcelas: i32,
    pub valor_parcela: f64,
    pub valor_total: f64,
    pub valor_liberado: f64,
    pub taxa_juros_mensal: f64,
    pub primeira_parcela: String,
    pub simulation_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GerarSimulacoesResponse {
    pub simulacoes: Vec<SimulacaoResumo>,
    pub status: String,
    pub mensagem: String,
}

// PROPOSTA

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CriarPropostaRequest {
    pub simulation_id: String,
    pub chave_pix: String,
    pub tipo_chave_pix: String, // "cpf", "phone", "email", "random"
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CriarPropostaResponse {
    pub operation_id: String,
    pub formalization_url: String,
    pub status: String,
    pub mensagem: String,
}

// CONSULTA DE OPERAÇÃO

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ConsultarOperacaoResponse {
    pub operation_id: String,
    pub status: String,
    pub provider: String,
    pub mensagem: String,
}

// LEGADOS (já existentes)

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ConsultaCpfRequest {
    pub cpf: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ConsultaCpfResponse {
    pub cpf: String,
    pub nome: String,
    pub status: String,
}

// PROPOSTA - VERSÃO COMPLETA

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CriarPropostaRequestCompleta {
    pub cpf: String,
    pub nome: String,
    pub email: String,
    pub telefone: String,
    pub data_nascimento: String, // YYYY-MM-DD
    pub genero: String,           // "male" ou "female"
    pub mae: String,
    pub simulation_id: String,
    pub chave_pix: String,
    pub tipo_chave_pix: String, // "cpf", "phone", "email", "random"
}

// VALIDAÇÃO DE PIX

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ValidarPixRequest {
    pub cpf: String,
    pub chave_pix: String,
    pub tipo_chave: String, // "cpf", "phone", "email", "random"
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ValidarPixResponse {
    pub valida: bool,
    pub tipo_chave: String,
    pub chave_formatada: Option<String>,
    pub mensagem: String,
}
