// SPDX-License-Identifier: BSD-2-Clause
import assert from "node:assert/strict";
import { test } from "node:test";
import { readPlan, validatePlan } from "./plan.ts";

test("repository plans cover each output regime", () => {
  for (const name of ["smoke", "local"]) {
    assert.deepEqual(readPlan(name).distributions, ["overlap", "wide", "distinct"]);
  }
});

test("invalid plans fail before starting a benchmark", () => {
  const plan = { lengths: [8], distributions: ["wide"], samples: 3 };
  for (const change of [
    { lengths: [] },
    { lengths: [-1] },
    { lengths: [1.5] },
    { samples: 0 },
    { samples: 1.5 },
    { distributions: ["unknown"] },
    { typo: 1 },
  ]) {
    assert.throws(() => validatePlan({ ...plan, ...change }, ["wide"]));
  }
});
