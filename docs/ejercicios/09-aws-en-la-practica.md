# Ejercicios: AWS en la práctica

- **Curso:** rust-cloud
- **Capítulo:** 09. AWS en la práctica
- **Estado:** implemented
- **Issue:** #36

Estos ejercicios practican AWS como traducción de fundamentos. La meta no es
crear recursos reales, sino explicar por qué un servicio representa una decisión
de arquitectura y qué señales deben existir antes de desplegar.

## Ejercicio 1: S3 gobernable `[Nivel 1]`

Construye un `AwsWorkload` para assets publicados:

- concepto `CloudConcept::Storage`;
- servicio `AwsService::S3`;
- ambiente de producción;
- región declarada;
- dueño y propósito explícitos;
- permisos mínimos;
- credenciales temporales;
- red privada;
- límite o ciclo de vida visible;
- observabilidad y tags de costo;
- `uses_real_credentials: false`.

**Entrada/Salida esperada:** la evaluación debe ser de bajo riesgo educativo.

<details>
<summary>Pista</summary>
S3 no es "un bucket". En el curso representa almacenamiento de objetos con
frontera de acceso, ciclo de vida, dueño y costo atribuible.
</details>

Solución compilable: `examples/soluciones/aws_practice_nivel_1.rs`.

## Ejercicio 2: runner de previews riesgoso `[Nivel 2]`

Modela un runner de previews que:

- declara concepto serverless;
- usa `AwsService::Ec2`;
- vive en desarrollo;
- no tiene dueño;
- usa permisos amplios;
- no usa credenciales temporales;
- queda expuesto públicamente sin frontera;
- no declara límites, observabilidad ni tags;
- retiene datos indefinidamente;
- intenta usar credenciales reales.

**Entrada/Salida esperada:** la evaluación debe reportar servicio que no
corresponde al concepto, falta de dueño, permisos amplios, credenciales
permanentes, ejemplo inseguro, red pública sin frontera, falta de límites,
observabilidad ausente, falta de tags y ciclo de vida indefinido.

<details>
<summary>Pista</summary>
El problema no es EC2. El problema es declararlo como serverless y además
normalizar atajos peligrosos.
</details>

Solución compilable: `examples/soluciones/aws_practice_nivel_2.rs`.

## Ejercicio 3: matriz por capacidad `[Nivel 3]`

Construye tres workloads:

- assets publicados con S3;
- red base con VPC;
- presupuesto de staging con Budgets.

Compara sus evaluaciones.

**Entrada/Salida esperada:** los tres deben quedar sin hallazgos si cada
servicio representa su concepto y declara dueño, región, permisos, límites,
observabilidad y tags.

<details>
<summary>Pista</summary>
Una matriz sana no elige servicios por familiaridad. Empieza por capacidad:
guardar, aislar, ejecutar, observar o gobernar costo.
</details>

Solución compilable: `examples/soluciones/aws_practice_nivel_3.rs`.

## Ejercicio 4: diseño AWS para Jeresoft Academy `[Nivel 4]`

Diseña una matriz para una versión mínima de Jeresoft Academy en AWS:

- frontend estático;
- API de cursos;
- assets públicos;
- base de datos;
- cola de publicación;
- observabilidad;
- presupuesto por ambiente.

Para cada componente declara concepto, servicio candidato, región, ambiente,
dueño, propósito, responsabilidad retenida, señal operativa y señal de costo.

**Entrada/Salida esperada:** una tabla con diez columnas: componente, concepto,
servicio, región, ambiente, dueño, propósito, responsabilidad retenida, señal
operativa y señal de costo.

<details>
<summary>Pista</summary>
Evita escribir "usar AWS" como respuesta. El valor está en explicar qué contrato
cumple cada servicio y qué riesgo conserva el equipo.
</details>

Discusión sugerida:

- S3 y CloudFront pueden servir frontend y assets, pero la frontera pública debe
  ser intencional.
- ECS Fargate puede servir una API si el curso quiere hablar de contenedores sin
  operar hosts.
- Lambda y SQS ayudan a enseñar eventos si se declara idempotencia y límite.
- RDS representa estado relacional, pero exige recuperación y costo visible.
- Budgets no arregla la arquitectura; vuelve visible la responsabilidad.
