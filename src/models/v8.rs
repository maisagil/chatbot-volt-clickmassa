use serde::{Deserialize, Serialize};

// 1. TERMO DE AUTORIZAÇÃO

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneNumber {
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "areaCode")]
    pub area_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTermoRequest {
    #[serde(rename = "borrowerDocumentNumber")]
    pub borrower_document_number: String,
    #[serde(rename = "signerName")]
    pub signer_name: String,
    #[serde(rename = "signerEmail")]
    pub signer_email: String,
    #[serde(rename = "signerPhone")]
    pub signer_phone: PhoneNumber,
    #[serde(rename = "birthDate")]
    pub birth_date: String, // YYYY-MM-DD
    pub gender: String,     // "male" ou "female"
    pub provider: String,   // "QI"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTermoResponse {
    pub id: String,
}

// 2. CONSULTA APÓS AUTORIZAÇÃO

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationLimit {
    #[serde(rename = "monthMin")]
    pub month_min: i32,
    #[serde(rename = "monthMax")]
    pub month_max: i32,
    #[serde(rename = "installmentsMin")]
    pub installments_min: i32,
    #[serde(rename = "installmentsMax")]
    pub installments_max: i32,
    #[serde(rename = "valueMin")]
    pub value_min: f64,
    #[serde(rename = "valueMax")]
    pub value_max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsultDataResponse {
    pub id: String,
    pub status: String,
    #[serde(rename = "partnerId")]
    pub partner_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "documentNumber")]
    pub document_number: String,
    pub name: String,
    #[serde(rename = "partnerInternalId")]
    pub partner_internal_id: String,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    pub gender: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub description: Option<String>,
    #[serde(rename = "marginBaseValue")]
    pub margin_base_value: String,
    #[serde(rename = "consultEligible")]
    pub consult_eligible: bool,
    #[serde(rename = "admissionDate")]
    pub admission_date: Option<String>,
    #[serde(rename = "terminationDate")]
    pub termination_date: Option<String>,
    #[serde(rename = "employerDocumentNumber")]
    pub employer_document_number: String,
    #[serde(rename = "employerName")]
    pub employer_name: String,
    #[serde(rename = "workerCategoryCode")]
    pub worker_category_code: i32,
    #[serde(rename = "registrationNumber")]
    pub registration_number: String,
    #[serde(rename = "admissionDateMonthsDifference")]
    pub admission_date_months_difference: i32,
    #[serde(rename = "simulationLimit")]
    pub simulation_limit: SimulationLimit,
    #[serde(rename = "recommendedSimulationInstallmentValue")]
    pub recommended_simulation_installment_value: String,
}

// 3. SIMULAÇÃO

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSimulationRequest {
    pub consult_id: String,
    pub number_of_installments: i32,
    pub installment_face_value: f64,
    pub config_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisbursementOption {
    pub iof_amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationResponse {
    pub id_simulation: String,
    pub installment_value: f64,
    pub number_of_installments: i32,
    pub operation_amount: f64,
    pub issue_amount: f64,
    pub disbursement_option: DisbursementOption,
    pub iof_amount: f64,
    pub monthly_interest_rate: f64,
    pub disbursed_issue_amount: f64,
    pub disbursement_amount: f64,
    pub first_installment_date: String,
    pub is_insured: bool,
    pub insurance_amount: Option<f64>,
    pub provider: String,
    pub simulation_config_id: String,
    pub simulation_config_slug: String,
}

// 4. CRIAR OPERAÇÃO/PROPOSTA

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowerPhone {
    pub country_code: String,
    pub area_code: String,
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowerAddress {
    pub postal_code: String,
    pub city: String,
    pub state: String,
    pub number: String,
    pub street: String,
    pub complement: Option<String>,
    pub neighborhood: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowerBank {
    pub transfer_method: String, // "pix"
    pub pix_key: String,
    pub pix_key_type: String, // "cpf", "phone", "email", "random"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkData {
    pub employer_name: String,
    pub employer_document_number: String,
    pub registration_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Borrower {
    pub name: String,
    pub email: String,
    pub phone: BorrowerPhone,
    pub political_exposition: bool,
    pub address: BorrowerAddress,
    pub birth_date: String, // YYYY-MM-DD
    pub mother_name: String,
    pub nationality: String,
    pub gender: String,
    pub person_type: String, // "natural"
    pub marital_status: String,
    pub individual_document_number: String,
    pub document_identification_date: String,
    pub document_issuer: String,
    pub document_identification_type: String,
    pub document_identification_number: String,
    pub bank: BorrowerBank,
    pub work_data: WorkData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOperationRequest {
    pub borrower: Borrower,
    pub simulation_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOperationResponse {
    pub id: String,
    pub formalization_url: String,
}

// 5. CONSULTAR OPERAÇÃO

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationResponse {
    pub id: String,
    pub status: String,
    pub provider: String,
}
