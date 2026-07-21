# Introducción

Cloud es el curso del Semestre 5 que enseña dónde corre el software y qué
decisiones aparecen cuando una plataforma deja de ser una máquina individual:
cómputo, almacenamiento, red, identidad, servicios manejados, serverless y
costos.

## Concepto

Una plataforma cloud no es una marca ni una consola. Es una colección de
contratos operativos: quién ejecuta cómputo, quién persiste datos, quién
controla identidad, cómo se conectan redes, qué se delega al proveedor y qué
responsabilidad conserva el equipo.

## Problema

El material de Cloud envejece rápido cuando se ata a pantallas, nombres
comerciales o tutoriales de proveedor. Jeresoft Academy necesita un curso que
enseñe fundamentos perdurables y aterrice después esos fundamentos en AWS y GCP.

## Alternativas

- Empezar por AWS o GCP y enseñar servicios concretos desde el día uno.
- Evitar proveedores por completo y dejar el curso abstracto.
- Enseñar conceptos primero y proveedores después.

## Justificación

Se adopta la tercera alternativa porque sigue RFC-0001 §10: los ocho primeros
capítulos son conceptuales y los dos últimos aterrizan la práctica por
proveedor. Así el curso resiste mejor la obsolescencia sin perder utilidad real.

## Frontera con DevOps

Cloud enseña las plataformas: dónde corre el software, qué servicios existen y
qué tradeoffs implican. DevOps enseña la operación: cómo se despliega, libera,
observa y repara. Kubernetes, pipelines y observabilidad profunda viven en
`rust-devops`; VPC, IAM, cómputo, almacenamiento, serverless y FinOps viven
aquí.
