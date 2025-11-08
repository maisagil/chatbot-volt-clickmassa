use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health::health_check,
        crate::routes::cpf::validar_cpf,
        crate::routes::cpf::consultar_cpf, 
        crate::routes::termo::criar_termo,
        crate::routes::termo::autorizar_termo,
        crate::routes::simulacao::gerar_simulacoes,
        crate::routes::pix::validar_pix,  
        crate::routes::proposta::criar_proposta,        
        crate::routes::proposta::consultar_operacao,   
    ),
    components(
        schemas(
            crate::models::chatbot::ValidarCpfRequest,
            crate::models::chatbot::ValidarCpfResponse,
            crate::models::chatbot::ConsultaCpfRequest,
            crate::models::chatbot::ConsultaCpfResponse,
            crate::models::chatbot::CriarTermoRequest,
            crate::models::chatbot::CriarTermoResponse,
            crate::models::chatbot::AutorizarTermoRequest,
            crate::models::chatbot::AutorizarTermoResponse,
            crate::models::chatbot::GerarSimulacoesRequest,
            crate::models::chatbot::GerarSimulacoesResponse,
            crate::models::chatbot::SimulacaoResumo,
            crate::models::chatbot::ValidarPixRequest,
            crate::models::chatbot::ValidarPixResponse,
            crate::models::chatbot::CriarPropostaRequestCompleta,
            crate::models::chatbot::CriarPropostaResponse,
            crate::models::chatbot::ConsultarOperacaoResponse,
        )
    ),
    info(
        title = "Chatbot Volt Crédito - Middleware API",
        version = "0.1.0",
        description = "API middleware para integração entre Chatbot ClickMassa e V8 Sistema",
        contact(
            name = "Volt Crédito",
            url = "https://voltcredito.com",
            email = "suporte@voltcredito.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (
            url = "http://localhost:3000",
            description = "Local Development"
        ),
        (
            url = "http://localhost:3000/api/v1",
            description = "Local Development"
        ),
        (
            url = "https://staging-api.voltcredito.com/api/v1",
            description = "Staging Environment"
        ),
        (
            url = "https://api.voltcredito.com/api/v1",
            description = "Production Environment"
        )
    ),
    tags(
        (name = "health", description = "Status e health check do serviço"),
        (name = "cpf", description = "Validação e consulta de dados de CPF"),
        (name = "pix", description = "Validação de chaves PIX"),  
        (name = "termo", description = "Gerenciamento de termo de autorização"),
        (name = "simulacao", description = "Geração de simulações de crédito"),
        (name = "proposta", description = "Criação de propostas e consulta de operações"),
    )
)]
pub struct ApiDoc;
