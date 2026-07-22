import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..");
const failures = [];

const localGates = [
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
];

const remoteGates = ["check remoto `rust`"];

const documents = [
  ["README.md", "README"],
  ["docs/publicacion-candidata.md", "publicación candidata"],
  ["docs/guia-revision-corte.md", "guía de revisión"],
  ["docs/checklist-revision-capitulo.md", "checklist por capítulo"],
  ["docs/paquete-revision-humana.md", "paquete humano"],
  ["docs/compuertas-automaticas.md", "matriz de compuertas"],
  ["docs/bitacora-revision-diferida.md", "bitácora de revisión diferida"],
];

function fail(message) {
  failures.push(message);
}

function hasGate(markdown, gate) {
  return markdown.includes(gate);
}

for (const [relativePath, label] of documents) {
  const markdown = readFileSync(join(root, relativePath), "utf8");

  for (const gate of localGates) {
    if (!hasGate(markdown, gate)) {
      fail(`${label}: falta compuerta ${gate}`);
    }
  }

  for (const gate of remoteGates) {
    if (!hasGate(markdown.toLowerCase(), gate)) {
      fail(`${label}: falta compuerta ${gate}`);
    }
  }
}

if (failures.length > 0) {
  console.error("Compuertas documentadas fuera de sincronía:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(
  `Compuertas sincronizadas: ${localGates.length + remoteGates.length} compuertas en ${documents.length} documentos.`,
);
