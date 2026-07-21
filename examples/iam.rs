use rust_cloud::iam::{
    AccessAction, AccessFinding, AccessGrant, AccessPlan, CredentialDuration, Principal,
    PrincipalKind, ResourceKind, ResourceScope, TrustBoundary,
};

fn main() {
    let mut academy_assets = AccessPlan::new("academy-assets").unwrap();
    academy_assets.add_grant(
        AccessGrant::new(
            Some(
                Principal::new(
                    "academy-web",
                    PrincipalKind::Workload,
                    TrustBoundary::SameAccount,
                    CredentialDuration::TemporarySession { hours: 1 },
                )
                .unwrap(),
            ),
            "assets-publicos",
            ResourceKind::ObjectStorage,
            Some(AccessAction::Write),
            Some(ResourceScope::SingleResource),
            "publicar assets generados por el pipeline",
        )
        .unwrap()
        .with_audit_event("object.write"),
    );

    assert!(academy_assets.evaluate().is_low_risk());
    println!("{}: acceso de bajo riesgo educativo", academy_assets.name());

    let mut incident = AccessPlan::new("incidente-prod").unwrap();
    incident.add_grant(
        AccessGrant::new(
            Some(
                Principal::new(
                    "operador-incidente",
                    PrincipalKind::HumanUser,
                    TrustBoundary::SameAccount,
                    CredentialDuration::BreakGlass { hours: 2 },
                )
                .unwrap(),
            ),
            "*",
            ResourceKind::PlatformAccount,
            Some(AccessAction::Administer),
            Some(ResourceScope::Wildcard),
            "recuperar producción durante incidente",
        )
        .unwrap(),
    );

    let findings = incident.evaluate().findings().to_vec();
    assert!(findings.contains(&AccessFinding::WildcardAdministration("*")));
    assert!(findings.contains(&AccessFinding::HumanAdminWithoutMfa("operador-incidente")));

    println!(
        "{}: {} hallazgos educativos",
        incident.name(),
        findings.len()
    );
}
