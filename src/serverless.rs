//! Serverless como contrato explícito de ejecución por eventos.
//!
//! Este módulo no simula una plataforma serverless real. Hace visibles
//! decisiones educativas: disparador, propósito, límite temporal, concurrencia,
//! retries, idempotencia, estado externo y observabilidad.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerKind {
    /// Petición HTTP o API gateway.
    Http,
    /// Mensaje en cola o stream.
    Queue,
    /// Cambio en almacenamiento de objetos.
    Storage,
    /// Ejecución programada.
    Schedule,
    /// Evento interno del dominio.
    DomainEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeProfile {
    /// Handler pequeño de función.
    Function,
    /// Contenedor serverless.
    Container,
    /// Workflow orquestado.
    Workflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryPolicy {
    /// Sin retry automático.
    None,
    /// Retry acotado por intentos.
    Fixed { attempts: u8 },
    /// Retry con backoff.
    Backoff { attempts: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdempotencyStrategy {
    /// No se declaró estrategia.
    None,
    /// Clave idempotente por evento.
    EventKey,
    /// Escritura condicional en estado externo.
    ConditionalWrite,
    /// Compensación explícita si se duplica trabajo.
    Compensation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateAccess {
    /// No toca estado durable.
    Stateless,
    /// Lee estado externo.
    ExternalRead,
    /// Escribe estado externo.
    ExternalWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservabilityPlan {
    /// Registra inicio, salida y errores.
    pub logs: bool,
    /// Emite métricas de latencia, error o throughput.
    pub metrics: bool,
    /// Permite seguir un evento entre componentes.
    pub correlation_id: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerlessRequirements {
    /// Disparador de la ejecución.
    pub trigger: TriggerKind,
    /// Runtime educativo.
    pub runtime: RuntimeProfile,
    /// Timeout máximo en segundos.
    pub timeout_seconds: Option<u16>,
    /// Concurrencia máxima declarada.
    pub max_concurrency: Option<u16>,
    /// Política de retry.
    pub retry_policy: RetryPolicy,
    /// Estrategia de idempotencia.
    pub idempotency: IdempotencyStrategy,
    /// Estado durable tocado.
    pub state_access: StateAccess,
    /// Observabilidad mínima.
    pub observability: ObservabilityPlan,
    /// Propósito humano.
    pub purpose: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerlessWorkload {
    name: &'static str,
    requirements: ServerlessRequirements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerlessDecisionError {
    /// Falta nombre.
    MissingName,
    /// Falta propósito humano.
    MissingPurpose,
    /// Falta timeout.
    MissingTimeout,
    /// Timeout inválido.
    InvalidTimeout(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerlessFinding {
    /// Retry sin idempotencia.
    RetryWithoutIdempotency(&'static str),
    /// Concurrencia sin límite explícito.
    UnboundedConcurrency(&'static str),
    /// Timeout alto para una función pequeña.
    HighFunctionTimeout(&'static str),
    /// Escritura de estado sin idempotencia.
    StatefulWriteWithoutIdempotency(&'static str),
    /// Observabilidad insuficiente.
    MissingObservability(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerlessEvaluation {
    findings: Vec<ServerlessFinding>,
}

impl ObservabilityPlan {
    /// Observabilidad mínima recomendada para este modelo.
    pub const fn standard() -> Self {
        Self {
            logs: true,
            metrics: true,
            correlation_id: true,
        }
    }

    /// Sin observabilidad explícita.
    pub const fn none() -> Self {
        Self {
            logs: false,
            metrics: false,
            correlation_id: false,
        }
    }

    const fn is_complete(self) -> bool {
        self.logs && self.metrics && self.correlation_id
    }
}

impl ServerlessWorkload {
    /// Crea una unidad serverless educativa.
    pub fn new(
        name: &'static str,
        requirements: ServerlessRequirements,
    ) -> Result<Self, ServerlessDecisionError> {
        if name.is_empty() {
            return Err(ServerlessDecisionError::MissingName);
        }

        if requirements.purpose.is_empty() {
            return Err(ServerlessDecisionError::MissingPurpose);
        }

        let timeout = requirements
            .timeout_seconds
            .ok_or(ServerlessDecisionError::MissingTimeout)?;
        if timeout == 0 {
            return Err(ServerlessDecisionError::InvalidTimeout(
                "timeout_seconds debe ser mayor que cero",
            ));
        }

        Ok(Self { name, requirements })
    }

    /// Evalúa señales de riesgo.
    pub fn evaluate(&self) -> ServerlessEvaluation {
        let mut findings = Vec::new();

        if retry_attempts(self.requirements.retry_policy) > 0
            && self.requirements.idempotency == IdempotencyStrategy::None
        {
            findings.push(ServerlessFinding::RetryWithoutIdempotency(self.name));
        }

        if self.requirements.max_concurrency.is_none() {
            findings.push(ServerlessFinding::UnboundedConcurrency(self.name));
        }

        if self.requirements.runtime == RuntimeProfile::Function
            && self.requirements.timeout_seconds.unwrap_or_default() > 60
        {
            findings.push(ServerlessFinding::HighFunctionTimeout(self.name));
        }

        if self.requirements.state_access == StateAccess::ExternalWrite
            && self.requirements.idempotency == IdempotencyStrategy::None
        {
            findings.push(ServerlessFinding::StatefulWriteWithoutIdempotency(
                self.name,
            ));
        }

        if !self.requirements.observability.is_complete() {
            findings.push(ServerlessFinding::MissingObservability(self.name));
        }

        ServerlessEvaluation { findings }
    }

    /// Nombre del workload.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Requisitos declarados.
    pub const fn requirements(&self) -> ServerlessRequirements {
        self.requirements
    }
}

impl ServerlessEvaluation {
    /// Indica si no hay hallazgos educativos.
    pub fn is_low_risk(&self) -> bool {
        self.findings.is_empty()
    }

    /// Hallazgos detectados.
    pub fn findings(&self) -> &[ServerlessFinding] {
        &self.findings
    }
}

const fn retry_attempts(policy: RetryPolicy) -> u8 {
    match policy {
        RetryPolicy::None => 0,
        RetryPolicy::Fixed { attempts } | RetryPolicy::Backoff { attempts } => attempts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workload_requires_timeout() {
        let requirements = ServerlessRequirements {
            trigger: TriggerKind::Http,
            runtime: RuntimeProfile::Function,
            timeout_seconds: None,
            max_concurrency: Some(10),
            retry_policy: RetryPolicy::None,
            idempotency: IdempotencyStrategy::None,
            state_access: StateAccess::Stateless,
            observability: ObservabilityPlan::standard(),
            purpose: "responder healthcheck",
        };

        assert_eq!(
            ServerlessWorkload::new("healthcheck", requirements),
            Err(ServerlessDecisionError::MissingTimeout)
        );
    }
}
