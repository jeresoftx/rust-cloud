import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..");
const manifest = JSON.parse(
  readFileSync(join(root, "course.manifest.json"), "utf8"),
);
const forbidden = new Set(["reviewed", "published"]);
const allowed = new Set([
  "planned",
  "draft",
  "implemented",
  "tested",
  "benchmarked",
]);
const failures = [];

function validateStatus(location, status) {
  if (!status) {
    failures.push(`${location}: falta estado editorial`);
    return;
  }

  if (forbidden.has(status)) {
    failures.push(`${location}: ${status} requiere revisión humana explícita`);
    return;
  }

  if (!allowed.has(status)) {
    failures.push(`${location}: estado desconocido ${status}`);
  }
}

function markdownFiles(dir) {
  return readdirSync(dir).flatMap((entry) => {
    const path = join(dir, entry);
    const stat = statSync(path);

    if (stat.isDirectory()) {
      return markdownFiles(path);
    }

    return path.endsWith(".md") ? [path] : [];
  });
}

validateStatus("course.status", manifest.course.status);

for (const chapter of manifest.chapters) {
  validateStatus(`chapters.${chapter.number}.status`, chapter.status);
}

for (const path of markdownFiles(join(root, "docs"))) {
  const content = readFileSync(path, "utf8");
  const match = content.match(/^- \*\*Estado:\*\* ([a-z-]+)$/m);

  if (match) {
    validateStatus(path.replace(`${root}/`, ""), match[1]);
  }
}

if (failures.length > 0) {
  console.error("Estados editoriales inválidos:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log("Estados editoriales verificados: sin reviewed ni published.");
