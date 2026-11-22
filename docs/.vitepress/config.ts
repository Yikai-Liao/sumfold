// SPDX-License-Identifier: BSD-2-Clause
import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Sumfold",
  description: "Exact sparse nonnegative integer convolution",
  base: process.env.DOCS_BASE ?? "/",
  srcDir: "..",
  srcExclude: [
    "AGENTS.md",
    "CLAUDE.md",
    "LICENSE",
    "node_modules/**",
    "target/**",
    "rust/**",
    "bench/**",
    "docs/README.md",
    "docs/decisions/README.md",
    ".github/**",
  ],
  rewrites: { "README.md": "index.md" },
  cleanUrls: true,
  lastUpdated: true,
  themeConfig: {
    nav: [
      { text: "Contract", link: "/conformance/semantics" },
      { text: "Roadmap", link: "/ROADMAP" },
    ],
    sidebar: [
      {
        text: "Use Sumfold",
        items: [
          { text: "Introduction", link: "/" },
          { text: "CLI", link: "/docs/guide/cli" },
          { text: "Semantics", link: "/conformance/semantics" },
        ],
      },
      {
        text: "Understand the engine",
        items: [
          { text: "Moment recovery", link: "/docs/explain/recovery" },
          { text: "Architecture", link: "/docs/design/engine" },
          { text: "Research sources", link: "/docs/research/sources" },
        ],
      },
      {
        text: "Develop",
        items: [
          { text: "Contributing", link: "/CONTRIBUTING" },
          { text: "CI", link: "/docs/development/ci" },
          { text: "Benchmarks", link: "/docs/development/benchmarking" },
          { text: "Release", link: "/RELEASING" },
        ],
      },
    ],
    socialLinks: [{ icon: "github", link: "https://github.com/Yikai-Liao/sumfold" }],
    editLink: { pattern: "https://github.com/Yikai-Liao/sumfold/edit/main/:path" },
  },
});
