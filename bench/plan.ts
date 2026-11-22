// SPDX-License-Identifier: BSD-2-Clause
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parse } from "smol-toml";

export interface Plan {
  lengths: number[];
  distributions: string[];
  samples: number;
}

/** Validate plans against the repository's distribution catalog. */
export function validatePlan(value: unknown, distributions: string[]): Plan {
  if (!value || typeof value !== "object") throw new Error("plan must be a table");
  const plan = value as Record<string, unknown>;
  if (Object.keys(plan).some((key) => !["lengths", "distributions", "samples"].includes(key))) {
    throw new Error("unknown plan field");
  }
  if (
    !Array.isArray(plan.lengths) ||
    plan.lengths.length === 0 ||
    plan.lengths.some((length) => !Number.isSafeInteger(length) || length < 1 || length > 4096)
  ) {
    throw new Error("lengths must contain integers from 1 through 4096");
  }
  if (
    !Array.isArray(plan.distributions) ||
    plan.distributions.length === 0 ||
    plan.distributions.some((name) => !distributions.includes(name))
  ) {
    throw new Error("unknown or empty distributions");
  }
  if (
    typeof plan.samples !== "number" ||
    !Number.isSafeInteger(plan.samples) ||
    plan.samples < 1 ||
    plan.samples > 1000
  ) {
    throw new Error("samples must be an integer from 1 through 1000");
  }
  return plan as unknown as Plan;
}

/** Read a named repository plan or an explicit TOML path. */
export function readPlan(name: string): Plan {
  const catalog = parse(readFileSync(new URL("catalog.toml", import.meta.url), "utf8"));
  const path = name.endsWith(".toml")
    ? resolve(name)
    : new URL(`plans/${name}.toml`, import.meta.url);
  return validatePlan(
    parse(readFileSync(path, "utf8")),
    Object.keys(catalog.distributions as object),
  );
}
