use crate::error::{AppError, AppResult};
use crate::utils::cpf_validator;
use regex::Regex;

/// Valida formato de chave PIX conforme tipo
pub fn validate_pix_key(chave: &str, tipo: &str) -> AppResult<String> {
    let chave_limpa = chave.trim().to_string();

    match tipo.to_lowercase().as_str() {
        "cpf" => validate_cpf_key(&chave_limpa),
        "phone" | "telefone" => validate_phone_key(&chave_limpa),
        "email" => validate_email_key(&chave_limpa),
        "random" | "aleatoria" => validate_random_key(&chave_limpa),
        _ => Err(AppError::ValidationError(
            format!("Tipo de chave PIX inválido: {}", tipo)
        )),
    }
}

/// Valida chave PIX tipo CPF
fn validate_cpf_key(chave: &str) -> AppResult<String> {
    cpf_validator::validate_cpf(chave)
}

/// Valida chave PIX tipo telefone
fn validate_phone_key(chave: &str) -> AppResult<String> {
    let phone_clean: String = chave.chars().filter(|c| c.is_ascii_digit()).collect();

    // Formato: +5511984353470 ou 5511984353470 ou 11984353470
    if phone_clean.len() == 13 && phone_clean.starts_with("55") {
        // +55 11 98435-3470
        return Ok(format!(
            "+55 {} {}-{}",
            &phone_clean[2..4],
            &phone_clean[4..9],
            &phone_clean[9..13]
        ));
    } else if phone_clean.len() == 11 {
        // 11 98435-3470
        return Ok(format!(
            "+55 {} {}-{}",
            &phone_clean[0..2],
            &phone_clean[2..7],
            &phone_clean[7..11]
        ));
    }

    Err(AppError::ValidationError(
        "Telefone deve ter 11 dígitos (DDD + número)".to_string(),
    ))
}

/// Valida chave PIX tipo email
fn validate_email_key(chave: &str) -> AppResult<String> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|_| AppError::InternalError("Erro ao compilar regex".to_string()))?;

    if email_regex.is_match(chave) {
        Ok(chave.to_lowercase())
    } else {
        Err(AppError::ValidationError(
            "Email inválido para chave PIX".to_string(),
        ))
    }
}

/// Valida chave PIX aleatória (EVP)
fn validate_random_key(chave: &str) -> AppResult<String> {
    let key_clean = chave.replace("-", "").to_lowercase();

    // Formato UUID: 8-4-4-4-12 caracteres hex
    let uuid_regex = Regex::new(r"^[a-f0-9]{32}$")
        .map_err(|_| AppError::InternalError("Erro ao compilar regex".to_string()))?;

    if uuid_regex.is_match(&key_clean) && key_clean.len() == 32 {
        // Formatar como UUID
        Ok(format!(
            "{}-{}-{}-{}-{}",
            &key_clean[0..8],
            &key_clean[8..12],
            &key_clean[12..16],
            &key_clean[16..20],
            &key_clean[20..32]
        ))
    } else {
        Err(AppError::ValidationError(
            "Chave aleatória deve ser um UUID válido (32 caracteres hex)".to_string(),
        ))
    }
}

/// Determina tipo de chave automaticamente
pub fn detect_pix_key_type(chave: &str) -> String {
    let clean = chave.chars().filter(|c| c.is_ascii_digit()).collect::<String>();

    // CPF: 11 dígitos
    if clean.len() == 11 && cpf_validator::validate_cpf(chave).is_ok() {
        return "cpf".to_string();
    }

    // Telefone: 11 ou 13 dígitos
    if clean.len() == 11 || clean.len() == 13 {
        return "phone".to_string();
    }

    // Email
    if chave.contains('@') {
        return "email".to_string();
    }

    // UUID/Random
    if chave.len() >= 32 {
        return "random".to_string();
    }

    "unknown".to_string()
}
