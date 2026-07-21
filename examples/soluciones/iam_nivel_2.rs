use rust_cloud::iam::{
    AccessAction, AccessFinding, AccessGrant, AccessPlan, CredentialDuration, Principal,
    PrincipalKind, ResourceKind, ResourceScope, TrustBoundary,
};

fn main() {
    let human = Principal::new(
        "operador-incidente",
        PrincipalKind::HumanUser,
        TrustBoundary::SameAccount,
        CredentialDuration::BreakGlass { hours: 2 },
    )
    .unwrap();
    let grant = AccessGrant::new(
        Some(human),
        "*",
        ResourceKind::PlatformAccount,
        Some(AccessAction::Administer),
        Some(ResourceScope::Wildcard),
        "recuperar producción durante incidente",
    )
    .unwrap();
    let mut plan = AccessPlan::new("incidente-prod").unwrap();

    plan.add_grant(grant);
    let findings = plan.evaluate().findings().to_vec();

    assert!(findings.contains(&AccessFinding::WildcardAdministration("*")));
    assert!(findings.contains(&AccessFinding::HumanAdminWithoutMfa("operador-incidente")));
    assert!(findings.contains(&AccessFinding::PrivilegedAccessWithoutAudit("*")));
}
