import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..");
const logPath = join(root, "docs", "bitacora-revision-diferida.md");
const log = readFileSync(logPath, "utf8");
const normalizedLog = log.replace(/\s+/g, " ");
const failures = [];

function fail(message) {
  failures.push(message);
}

for (const section of [
  "# Bitácora de revisión diferida",
  "## PRs del corte candidato",
  "## Evidencia técnica",
  "## Incidentes de higiene documentados",
  "## Sincronía documental reciente",
  "## Decisiones que no tomó la IA",
  "## Pendientes para revisión humana",
]) {
  if (!log.includes(section)) {
    fail(`falta sección: ${section}`);
  }
}

for (const row of [
  "| #118 | #112 | Higiene obligatoria para ramas, commits únicos y PRs autónomos. |",
  "| #121 | #120 | Guía de revisión alineada con las compuertas actuales. |",
  "| #122 | #119 | Verificador de sincronía entre listas de compuertas documentadas. |",
  "| #125 | #123 | Checklist por capítulo alineada con la evidencia mínima del corte. |",
  "| #126 | #124 | Checklist por capítulo protegida por la sincronía de compuertas. |",
]) {
  if (!log.includes(row)) {
    fail(`falta fila de PR reciente: ${row}`);
  }
}

for (const command of [
  "node scripts/verify-course-links.mjs",
  "node scripts/verify-manifest-consistency.mjs",
  "node scripts/verify-cargo-examples.mjs",
  "node scripts/verify-chapter-anatomy.mjs",
  "node scripts/verify-review-packet-coverage.mjs",
  "node scripts/verify-deferred-review-log.mjs",
  "node scripts/verify-editorial-status.mjs",
  "node scripts/verify-gate-sync.mjs",
  "cargo fmt --check",
  "cargo clippy --all-targets --all-features -- -D warnings",
  "cargo test --all-targets",
  "cargo test --doc",
  "cargo bench --all-targets",
  "git diff --check",
]) {
  if (!log.includes(command)) {
    fail(`falta evidencia técnica: ${command}`);
  }
}

for (const phrase of [
  "No se marcó ningún capítulo como `reviewed`.",
  "No se marcó ningún capítulo como `published`.",
  "no publica el curso",
  "reemplaza la lectura humana",
]) {
  if (!normalizedLog.includes(phrase)) {
    fail(`falta frontera editorial: ${phrase}`);
  }
}

if (failures.length > 0) {
  console.error("Bitácora de revisión diferida incompleta:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log("Bitácora de revisión diferida verificada: cobertura mínima completa.");
