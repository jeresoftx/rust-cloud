# Ejercicios: identidad y accesos

- **Curso:** rust-cloud
- **Capítulo:** 05. Identidad y accesos
- **Estado:** implemented
- **Issue:** #20

Estos ejercicios practican IAM como contrato entre identidad, acción, recurso,
alcance, duración, propósito y evidencia. La meta no es memorizar políticas de
proveedor, sino aprender a reconocer permisos sanos y permisos que dejan deuda.

## Ejercicio 1: construir un acceso de bajo riesgo `[Nivel 1]`

Crea un `Principal` de tipo `Workload` para `academy-web`, con frontera
`SameAccount` y sesión temporal de una hora. Concede escritura sobre
`assets-publicos` como `ObjectStorage` y `SingleResource`, con propósito y
evento auditable.

**Entrada/Salida esperada:** `AccessPlan::evaluate().is_low_risk()` debe ser
`true`.

<details>
<summary>Pista</summary>
El plan sano combina alcance pequeño, credencial temporal, propósito explícito
y evento auditable.
</details>

Solución compilable: `examples/soluciones/iam_nivel_1.rs`.

## Ejercicio 2: detectar administración amplia sin MFA `[Nivel 2]`

Modela un acceso de emergencia para `operador-incidente` con acción
`Administer`, recurso `*`, alcance `Wildcard` y sin MFA.

**Entrada/Salida esperada:** la evaluación debe contener
`WildcardAdministration`, `HumanAdminWithoutMfa` y
`PrivilegedAccessWithoutAudit`.

<details>
<summary>Pista</summary>
Break-glass no significa acceso sin control. Incluso una emergencia necesita
duración corta, MFA, propósito, condición y evento auditable.
</details>

Solución compilable: `examples/soluciones/iam_nivel_2.rs`.

## Ejercicio 3: revisar una identidad externa `[Nivel 3]`

Modela una identidad externa permanente para `proveedor-analitica`, con lectura
sobre un grupo de recursos de base de datos, sin condición explícita.

**Entrada/Salida esperada:** la evaluación debe contener
`LongLivedExternalAccess` y `CrossBoundaryWithoutCondition`.

<details>
<summary>Pista</summary>
La identidad externa puede ser legítima, pero necesita duración acotada,
condición, alcance pequeño y auditoría clara.
</details>

Solución compilable: `examples/soluciones/iam_nivel_3.rs`.

## Ejercicio 4: diseñar IAM para una academia `[Nivel 4]`

Diseña una propuesta de acceso inicial para:

- publicación de assets;
- pipeline de despliegue;
- lectura de métricas;
- acceso administrativo de emergencia;
- proveedor externo de analítica;
- rotación de secretos.

Para cada flujo, declara principal, acción, recurso, alcance, duración,
condición, evento auditable, hallazgos esperados y mitigación.

**Entrada/Salida esperada:** una tabla con nueve columnas: flujo, principal,
acción, recurso, alcance, duración, condición, auditoría y riesgo/mitigación.

<details>
<summary>Pista</summary>
No busques que todos los flujos sean idénticos. Un workload interno, un humano
en emergencia y un proveedor externo tienen fronteras de confianza distintas.
</details>

Discusión sugerida:

- Publicar assets puede ser un permiso temporal y acotado a un recurso.
- Desplegar infraestructura suele necesitar privilegios mayores, pero debe
  tener condición, auditoría y revisión.
- Leer métricas no debería implicar administrar recursos.
- Break-glass debe caducar, exigir MFA y dejar evento auditable.
- Una identidad externa permanente es una alerta de diseño; si existe, debe
  tener justificación, contrato, condición y revisión recurrente.
