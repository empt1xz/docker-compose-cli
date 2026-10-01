#!/usr/bin/env node
const { spawn } = require("child_process");
const path = require("path");
const fs = require("fs");

const platform = process.platform;
const arch = process.arch;         
const ext = platform === "win32" ? ".exe" : "";
const binary = path.join(
  __dirname, "..", "binaries",
  `cli_docker-${platform}-${arch}${ext}`
);

if (!fs.existsSync(binary)) {
  console.error(`Plataforma não suportada: ${platform}-${arch}`);
  process.exit(1);
}

if (platform !== "win32") {
  try { fs.chmodSync(binary, 0o755); } catch {}
}

const child = spawn(binary, process.argv.slice(2), { stdio: "inherit" });
child.on("exit", (code) => process.exit(code ?? 0));
child.on("error", (err) => {
  console.error("Erro ao executar:", err.message);
  process.exit(1);
});
