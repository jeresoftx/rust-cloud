# Ejercicios: servicios manejados

- **Curso:** rust-cloud
- **Capítulo:** 06. Servicios manejados
- **Estado:** implemented
- **Issue:** #24

Estos ejercicios practican servicios manejados como frontera de delegación. La
meta no es memorizar catálogos de proveedor, sino declarar qué capacidad se
consume, qué operación se delega, qué conserva el equipo y qué recuperación se
ha probado.

## Ejercicio 1: modelar una base de datos administrada `[Nivel 1]`

Construye un `ManagedServiceProfile` para `academy-db`:

- capacidad `ManagedCapability::Database`;
- estado durable;
- criticidad alta;
- `RecoveryStrategy::TestedRestore`;
- dueño operativo declarado;
- propósito explícito.

**Entrada/Salida esperada:** la evaluación debe ser de bajo riesgo educativo.

<details>
<summary>Pista</summary>
Una base de datos administrada no elimina diseño de datos ni recuperación. El
caso es de bajo riesgo porque declara dueño y restauración probada.
</details>

Solución compilable: `examples/soluciones/managed_services_nivel_1.rs`.

## Ejercicio 2: detectar estado durable sin recuperación `[Nivel 2]`

Modela un índice de búsqueda durable para contenido publicado, pero sin
estrategia de recuperación.

**Entrada/Salida esperada:** la evaluación debe contener
`ManagedServiceFinding::DurableStateWithoutRecovery`.

<details>
<summary>Pista</summary>
Aunque un índice pueda reconstruirse, debes declarar si realmente existe esa
capacidad. Si no hay recuperación, el riesgo debe quedar visible.
</details>

Solución compilable: `examples/soluciones/managed_services_nivel_2.rs`.

## Ejercicio 3: comparar cola temporal y servicio crítico `[Nivel 3]`

Construye dos perfiles:

- una cola para eventos temporales de publicación;
- una base de datos crítica con backup, pero sin restore probado.

Compara sus hallazgos.

**Entrada/Salida esperada:** la cola temporal debe ser de bajo riesgo
educativo. La base de datos crítica debe reportar
`CriticalServiceWithoutTestedRecovery`.

<details>
<summary>Pista</summary>
No todo servicio necesita la misma recuperación. La criticidad y el estado
durable cambian el criterio.
</details>

Solución compilable: `examples/soluciones/managed_services_nivel_3.rs`.

## Ejercicio 4: matriz de responsabilidades `[Nivel 4]`

Escribe una matriz para una plataforma educativa con:

- base de datos de progreso;
- cola de trabajos;
- caché de lecturas frecuentes;
- gestor de secretos;
- observabilidad;
- búsqueda de contenidos.

Para cada servicio, declara capacidad, estado, criticidad, responsabilidad
delegada, responsabilidad retenida, recuperación, dueño y costo/riesgo.

**Entrada/Salida esperada:** una tabla con ocho columnas: servicio, capacidad,
estado, criticidad, delegación, retención, recuperación y costo/riesgo.

<details>
<summary>Pista</summary>
El punto no es elegir "todo manejado". El punto es saber qué se está delegando
y qué deuda queda aunque el proveedor opere la infraestructura base.
</details>

Discusión sugerida:

- Una cola temporal puede tolerar pérdida distinta a una base de datos crítica.
- Un caché administrado sigue necesitando estrategia de expiración e invalidación.
- Un gestor de secretos reduce operación, pero exige permisos y rotación.
- Observabilidad administrada no reemplaza buenas señales ni alertas útiles.
- Búsqueda administrada puede introducir costo por índice, almacenamiento,
  consultas y reconstrucción.
