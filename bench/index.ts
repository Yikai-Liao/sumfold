// SPDX-License-Identifier: BSD-2-Clause
import { execFileSync } from "node:child_process";
import { arch, cpus, platform } from "node:os";
import { fileURLToPath } from "node:url";
import { readPlan } from "./plan.ts";

const root = fileURLToPath(new URL("../", import.meta.url));
const args = process.argv.slice(2);
const help = "Usage: pnpm bench plan [NAME-OR-FILE] | run [--plan NAME-OR-FILE]";
const command = args.shift();
try {
  if (command === "--help" || command === undefined) {
    console.log(help);
  } else if (command === "plan") {
    if (args.length > 1) throw new Error(help);
    console.log(JSON.stringify(readPlan(args[0] ?? "smoke"), null, 2));
  } else if (command === "run") {
    if (args.length !== 0 && (args.length !== 2 || args[0] !== "--plan")) throw new Error(help);
    const plan = readPlan(args[1] ?? "smoke");
    const capture = (program: string, arguments_: string[]) =>
      execFileSync(program, arguments_, { cwd: root, encoding: "utf8" }).trim();
    execFileSync("cargo", ["build", "--locked", "--release", "-p", "sumfold-bench"], {
      cwd: root,
      stdio: ["ignore", "ignore", "inherit"],
    });
    console.log(
      JSON.stringify({
        kind: "metadata",
        head: capture("git", ["rev-parse", "HEAD"]),
        dirty: capture("git", ["status", "--porcelain"]) !== "",
        rustc: capture("rustc", ["--version"]),
        platform: platform(),
        arch: arch(),
        cpu: cpus()[0]?.model,
        plan,
      }),
    );
    for (const length of plan.lengths) {
      for (const distribution of plan.distributions) {
        execFileSync(
          fileURLToPath(
            new URL(
              `../target/release/sumfold-bench${platform() === "win32" ? ".exe" : ""}`,
              import.meta.url,
            ),
          ),
          [String(length), distribution, String(plan.samples)],
          { cwd: root, stdio: "inherit" },
        );
      }
    }
  } else {
    throw new Error(help);
  }
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
}
