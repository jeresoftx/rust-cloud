use rust_cloud::iam::{
    AccessAction, AccessFinding, AccessGrant, AccessPlan, CredentialDuration, Principal,
    PrincipalKind, ResourceKind, ResourceScope, TrustBoundary,
};

fn main() {
    let external = Principal::new(
        "proveedor-analitica",
        PrincipalKind::ExternalIdentity,
        TrustBoundary::ExternalProvider,
        CredentialDuration::Permanent,
    )
    .unwrap();
    let grant = AccessGrant::new(
        Some(external),
        "eventos-curso",
        ResourceKind::Database,
        Some(AccessAction::Read),
        Some(ResourceScope::ResourceGroup),
        "integración externa para analítica agregada",
    )
    .unwrap();
    let mut plan = AccessPlan::new("analytics-externo").unwrap();

    plan.add_grant(grant);
    let findings = plan.evaluate().findings().to_vec();

    assert!(findings.contains(&AccessFinding::LongLivedExternalAccess(
        "proveedor-analitica"
    )));
    assert!(
        findings.contains(&AccessFinding::CrossBoundaryWithoutCondition(
            "proveedor-analitica"
        ))
    );
}
