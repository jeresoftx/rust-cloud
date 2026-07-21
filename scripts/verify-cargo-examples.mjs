import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const root = join(import.meta.dirname, "..");
const cargoToml = readFileSync(join(root, "Cargo.toml"), "utf8");
const failures = [];

function fail(message) {
  failures.push(message);
}

function collectExampleFiles(directory) {
  const files = [];

  for (const entry of readdirSync(directory)) {
    const absolutePath = join(directory, entry);
    const stats = statSync(absolutePath);

    if (stats.isDirectory()) {
      files.push(...collectExampleFiles(absolutePath));
      continue;
    }

    if (entry.endsWith(".rs")) {
      files.push(relative(root, absolutePath));
    }
  }

  return files.sort();
}

const examples = [];
const exampleBlockPattern = /\[\[example\]\]([\s\S]*?)(?=\n\[\[|$)/g;

for (const match of cargoToml.matchAll(exampleBlockPattern)) {
  const block = match[1];
  const name = block.match(/^\s*name\s*=\s*"([^"]+)"\s*$/m)?.[1];
  const path = block.match(/^\s*path\s*=\s*"([^"]+)"\s*$/m)?.[1];

  if (!name || !path) {
    fail("[[example]] debe declarar name y path");
    continue;
  }

  examples.push({ name, path });
}

const declaredNames = new Map();
const declaredPaths = new Map();

for (const example of examples) {
  if (declaredNames.has(example.name)) {
    fail(`nombre de ejemplo duplicado: ${example.name}`);
  }

  if (declaredPaths.has(example.path)) {
    fail(`ruta de ejemplo duplicada: ${example.path}`);
  }

  declaredNames.set(example.name, example);
  declaredPaths.set(example.path, example);

  try {
    const stats = statSync(join(root, example.path));

    if (!stats.isFile()) {
      fail(`${example.path} no es un archivo`);
    }
  } catch {
    fail(`ruta declarada inexistente: ${example.path}`);
  }
}

for (const path of collectExampleFiles(join(root, "examples"))) {
  if (!declaredPaths.has(path)) {
    fail(`ejemplo sin declarar en Cargo.toml: ${path}`);
  }
}

if (failures.length > 0) {
  console.error("Registro de ejemplos Cargo inconsistente:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log("Ejemplos Cargo verificados: todas las rutas existen y están declaradas.");
