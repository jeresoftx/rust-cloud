use rust_cloud::iam::{
    AccessAction, AccessGrant, AccessPlan, CredentialDuration, Principal, PrincipalKind,
    ResourceKind, ResourceScope, TrustBoundary,
};

fn main() {
    let workload = Principal::new(
        "academy-web",
        PrincipalKind::Workload,
        TrustBoundary::SameAccount,
        CredentialDuration::TemporarySession { hours: 1 },
    )
    .unwrap();
    let grant = AccessGrant::new(
        Some(workload),
        "assets-publicos",
        ResourceKind::ObjectStorage,
        Some(AccessAction::Write),
        Some(ResourceScope::SingleResource),
        "publicar assets generados por el pipeline",
    )
    .unwrap()
    .with_audit_event("object.write");
    let mut plan = AccessPlan::new("publicacion-assets").unwrap();

    plan.add_grant(grant);

    assert!(plan.evaluate().is_low_risk());
}
