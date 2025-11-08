use crate::error::{AppError, AppResult};

/// Remove caracteres não numéricos do CPF
pub fn clean_cpf(cpf: &str) -> String {
    cpf.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

/// Valida se o CPF é válido
pub fn validate_cpf(cpf: &str) -> AppResult<String> {
    let cpf_clean = clean_cpf(cpf);

    // 1. Verificar se tem 11 dígitos
    if cpf_clean.len() != 11 {
        tracing::warn!("CPF inválido: deve ter 11 dígitos. Recebido: {}", cpf);
        return Err(AppError::ValidationError(
            "CPF deve conter exatamente 11 dígitos".to_string(),
        ));
    }

    // 2. Verificar se todos os dígitos são iguais (ex: 111.111.111-11)
    if cpf_clean.chars().all(|c| c == cpf_clean.chars().next().unwrap()) {
        tracing::warn!("CPF inválido: todos os dígitos são iguais. CPF: {}", cpf);
        return Err(AppError::ValidationError(
            "CPF inválido: todos os dígitos são iguais".to_string(),
        ));
    }

    // 3. Validar dígitos verificadores
    if !validate_check_digits(&cpf_clean) {
        tracing::warn!("CPF inválido: dígitos verificadores incorretos. CPF: {}", cpf);
        return Err(AppError::ValidationError(
            "CPF inválido: dígitos verificadores incorretos".to_string(),
        ));
    }

    tracing::debug!("CPF válido: {}", cpf_clean);
    Ok(cpf_clean)
}

/// Valida os dígitos verificadores do CPF
fn validate_check_digits(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Calcular primeiro dígito verificador
    let mut sum = 0;
    for i in 0..9 {
        sum += digits[i] * (10 - i as u32);
    }
    let remainder = sum % 11;
    let first_check_digit = if remainder < 2 { 0 } else { 11 - remainder };

    if digits[9] != first_check_digit {
        return false;
    }

    // Calcular segundo dígito verificador
    let mut sum = 0;
    for i in 0..10 {
        sum += digits[i] * (11 - i as u32);
    }
    let remainder = sum % 11;
    let second_check_digit = if remainder < 2 { 0 } else { 11 - remainder };

    digits[10] == second_check_digit
}

/// Formata CPF para exibição (xxx.xxx.xxx-xx)
pub fn format_cpf(cpf: &str) -> String {
    let cpf_clean = clean_cpf(cpf);
    if cpf_clean.len() != 11 {
        return cpf.to_string();
    }

    format!(
        "{}.{}.{}-{}",
        &cpf_clean[0..3],
        &cpf_clean[3..6],
        &cpf_clean[6..9],
        &cpf_clean[9..11]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_cpf() {
        assert_eq!(clean_cpf("123.456.789-10"), "12345678910");
        assert_eq!(clean_cpf("123 456 789 10"), "12345678910");
        assert_eq!(clean_cpf("12345678910"), "12345678910");
    }

    #[test]
    fn test_valid_cpf() {
        // CPF válido de teste
        assert!(validate_cpf("11144477735").is_ok());
        assert!(validate_cpf("111.444.777-35").is_ok());
    }

    #[test]
    fn test_invalid_cpf_length() {
        assert!(validate_cpf("123").is_err());
        assert!(validate_cpf("123456789101112").is_err());
    }

    #[test]
    fn test_invalid_cpf_all_same_digits() {
        assert!(validate_cpf("11111111111").is_err());
        assert!(validate_cpf("00000000000").is_err());
        assert!(validate_cpf("99999999999").is_err());
    }

    #[test]
    fn test_invalid_cpf_check_digits() {
        assert!(validate_cpf("12345678901").is_err());
        assert!(validate_cpf("11144477736").is_err());
    }

    #[test]
    fn test_format_cpf() {
        assert_eq!(format_cpf("11144477735"), "111.444.777-35");
        assert_eq!(format_cpf("111.444.777-35"), "111.444.777-35");
    }
}
