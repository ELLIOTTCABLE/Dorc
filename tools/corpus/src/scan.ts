/**
 * scan.ts — Dorc corpus scanner (spike instrument), v2.
 *
 * Walks one or more corpus roots, parses shell (tree-sitter-bash, error-tolerant)
 * and Ansible (yaml) DEFENSIVELY, and accumulates a streaming tally.
 *
 * Contract: one bad file must NEVER crash the run — it is skipped, logged, and
 * counted (the parse-failure rate is itself a reported datum, Q-PARSE). Streaming:
 * one AST in memory at a time; the tally is bounded by DISTINCT commands/modules,
 * not file count (the Q-WORKINGSET answer in miniature).
 *
 * v2 adds the de-biasing the first raw tally proved necessary (see note 80):
 *  - normalize module FQCNs (ansible.builtin.apt -> apt) so they stop splitting;
 *  - separate TEST/scaffolding files (tests/, molecule/, .github/, ...) from real
 *    workload — collection test-code (assert/debug) was ~24% of raw tasks;
 *  - split Ansible modules + shell commands into mutating vs control/diagnostic;
 *  - count "guarded" mutating tasks (when/creates/removes) as a crude VALUE proxy;
 *  - stratify by source-type (role / collection / homelab / shell) — Simpson guard.
 *
 * NOT here yet: the full apply-cost x check-depth banding (Q-BAND/Q-ANTICORR). Its
 * cost/depth thresholds are a judgment call parked for the user (note 80 §7).
 */
import type { Parser as TSParser } from "web-tree-sitter";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, extname } from "node:path";
import { fileURLToPath } from "node:url";
import YAML from "yaml";

// web-tree-sitter values are loaded via dynamic import in main(); type-only here.

// Vendored grammar (see grammars/README.md for provenance).
const BASH_WASM = fileURLToPath(new URL("../grammars/tree-sitter-bash.wasm", import.meta.url));

const MAX_FILE_BYTES = 1_000_000; // skip >1MB: generated/minified, not hand-authored ops code
const SKIP_DIRS = new Set([".git", "node_modules", ".hg", ".svn", ".terraform"]);
const SHELL_EXT = new Set([".sh", ".bash", ".ksh", ".dash"]);
const YAML_EXT = new Set([".yml", ".yaml"]);

// Test/scaffolding path segments — not ops workload. Counted separately.
const SCAFFOLD_SEG = /(^|[\\/])(tests?|test|molecule|spec|\.github|examples?|ci)([\\/]|$)/i;

// Ansible task keys that are control/metadata, NOT the module being invoked.
const ANSIBLE_RESERVED = new Set([
   "name", "when", "with_items", "with_dict", "with_fileglob", "loop", "loop_control",
   "register", "become", "become_user", "become_method", "notify", "tags", "vars",
   "block", "rescue", "always", "delegate_to", "run_once", "changed_when", "failed_when",
   "ignore_errors", "no_log", "check_mode", "environment", "args", "until", "retries",
   "delay", "listen", "any_errors_fatal", "throttle", "module_defaults", "diff",
]);
// Modules that do NOT mutate host state (control-flow / diagnostic / read-only).
const ANSIBLE_CONTROL = new Set([
   "assert", "debug", "fail", "set_fact", "set_stats", "meta", "pause", "add_host",
   "group_by", "include_tasks", "import_tasks", "include_role", "import_role",
   "include_vars", "import_playbook", "include", "ping", "setup", "gather_facts",
   "stat", "slurp", "find", "command_facts", "package_facts", "service_facts",
   "getent", "assemble_facts", "wait_for", "wait_for_connection",
]);
// Shell command names that do NOT mutate host state (builtins / read-only utils).
const SHELL_NONMUTATING = new Set([
   "echo", "printf", "print", ":", "true", "false", "test", "[", "[[", "return", "exit",
   "read", "cd", "pwd", "export", "set", "unset", "shift", "local", "declare", "typeset",
   "let", "eval", "source", ".", "trap", "getopts", "type", "command", "builtin", "alias",
   "grep", "egrep", "fgrep", "awk", "sed", "cut", "head", "tail", "sort", "uniq", "wc",
   "tr", "cat", "basename", "dirname", "dirname", "expr", "seq", "tee", "xargs", "find",
   "ls", "stat", "readlink", "realpath", "date", "id", "whoami", "hostname", "uname",
   "which", "env", "sleep", "tput", "logger", "wait",
]);
const ANSIBLE_GUARD_KEYS = ["when", "creates", "removes"]; // crude pre-guard / shallow-check proxy
const ANSIBLE_CHECK_KEYS = ["creates", "removes", "changed_when", "failed_when", "when", "check_mode"];

type Stratum = "role" | "collection" | "homelab" | "shell" | "other";

interface Counts {
   shellFiles: number;
   shellCommands: number;
   shellMutating: number;
   taskFiles: number;
   tasks: number;
   mutating: number;
   mutatingGuarded: number;
}
const newCounts = (): Counts => ({
   shellFiles: 0, shellCommands: 0, shellMutating: 0,
   taskFiles: 0, tasks: 0, mutating: 0, mutatingGuarded: 0,
});

interface Tally {
   roots: string[];
   filesSeen: number;
   filesSkippedBig: number;
   filesUnreadable: number;
   scaffoldFilesSkipped: number; // test/scaffolding shell+yaml not folded into workload
   shell: {
      files: number; parseThrew: number; withErrorNode: number;
      totalCommands: number; mutatingCommands: number;
      evalCount: number; dynamicNameCount: number;
      commands: Map<string, number>;
   };
   ansible: {
      yamlFiles: number; yamlParseThrew: number;
      taskFiles: number; totalTasks: number;
      mutatingTasks: number; controlTasks: number; mutatingGuarded: number;
      modules: Map<string, number>; // normalized, workload-only
      checkKeys: Map<string, number>;
   };
   byStratum: Map<Stratum, Counts>;
}

function emptyTally(roots: string[]): Tally {
   return {
      roots, filesSeen: 0, filesSkippedBig: 0, filesUnreadable: 0, scaffoldFilesSkipped: 0,
      shell: {
         files: 0, parseThrew: 0, withErrorNode: 0, totalCommands: 0, mutatingCommands: 0,
         evalCount: 0, dynamicNameCount: 0, commands: new Map(),
      },
      ansible: {
         yamlFiles: 0, yamlParseThrew: 0, taskFiles: 0, totalTasks: 0,
         mutatingTasks: 0, controlTasks: 0, mutatingGuarded: 0,
         modules: new Map(), checkKeys: new Map(),
      },
      byStratum: new Map(),
   };
}

const bump = (m: Map<string, number>, k: string, n = 1) => m.set(k, (m.get(k) ?? 0) + n);
function stratumCounts(t: Tally, s: Stratum): Counts {
   let c = t.byStratum.get(s);
   if (!c) { c = newCounts(); t.byStratum.set(s, c); }
   return c;
}

/** FQCN -> base module name: ansible.builtin.apt -> apt; community.docker.x -> x. */
function normalizeModule(m: string): string {
   return m.includes(".") ? (m.split(".").pop() ?? m) : m;
}

function stratumOf(path: string): Stratum {
   const p = path.replace(/\\/g, "/");
   if (p.includes("/homelab/")) return "homelab";
   if (p.includes("/ansible/gg-")) return "role";
   if (p.includes("/ansible/coll-")) return "collection";
   if (p.includes("/shell/")) return "shell";
   return "other";
}

function* walk(root: string): Generator<string> {
   let entries: string[];
   try { entries = readdirSync(root); } catch { return; }
   for (const name of entries) {
      const p = join(root, name);
      let st;
      try { st = statSync(p); } catch { continue; }
      if (st.isDirectory()) {
         if (SKIP_DIRS.has(name)) continue;
         yield* walk(p);
      } else if (st.isFile()) {
         yield p;
      }
   }
}

function readText(path: string, tally: Tally): string | null {
   let st;
   try { st = statSync(path); } catch { tally.filesUnreadable++; return null; }
   if (st.size > MAX_FILE_BYTES) { tally.filesSkippedBig++; return null; }
   try { return readFileSync(path, "utf8"); } catch { tally.filesUnreadable++; return null; }
}

function classifyCommandName(raw: string): { name: string; dynamic: boolean } {
   const t = raw.trim();
   if (t === "" || /[$`(]/.test(t)) return { name: "<dynamic>", dynamic: true };
   const base = t.includes("/") ? (t.split("/").pop() ?? t) : t;
   return { name: base, dynamic: false };
}

function tallyShell(src: string, parser: TSParser, tally: Tally, stratum: Stratum): void {
   const s = tally.shell;
   const sc = stratumCounts(tally, stratum);
   s.files++; sc.shellFiles++;
   let tree;
   try { tree = parser.parse(src); } catch { s.parseThrew++; return; }
   if (!tree) { s.parseThrew++; return; }
   if (tree.rootNode.hasError) s.withErrorNode++;
   const stack = [tree.rootNode];
   while (stack.length) {
      const n = stack.pop()!;
      if (n.type === "command") {
         const { name, dynamic } = classifyCommandName(n.childForFieldName("name")?.text ?? "");
         bump(s.commands, name);
         s.totalCommands++; sc.shellCommands++;
         if (dynamic) s.dynamicNameCount++;
         if (name === "eval") s.evalCount++;
         if (!dynamic && !SHELL_NONMUTATING.has(name)) { s.mutatingCommands++; sc.shellMutating++; }
      }
      for (let i = 0; i < n.childCount; i++) { const c = n.child(i); if (c) stack.push(c); }
   }
   tree.delete();
}

function looksLikeAnsibleTasks(doc: unknown): doc is Record<string, unknown>[] {
   if (!Array.isArray(doc) || doc.length === 0) return false;
   return doc.some(
      (item) =>
         item && typeof item === "object" &&
         Object.keys(item as object).some((k) => ANSIBLE_RESERVED.has(k) || k.includes(".")),
   );
}

function moduleOf(task: Record<string, unknown>): string | null {
   for (const key of ["action", "local_action"]) {
      const v = task[key];
      if (typeof v === "string") return v.split(/\s+/)[0] ?? key;
   }
   for (const k of Object.keys(task)) if (!ANSIBLE_RESERVED.has(k)) return k;
   return null;
}

function tallyAnsible(src: string, tally: Tally, stratum: Stratum): void {
   const a = tally.ansible;
   const sc = stratumCounts(tally, stratum);
   a.yamlFiles++;
   let docs: unknown[];
   try { docs = YAML.parseAllDocuments(src).map((d) => d.toJS({ maxAliasCount: -1 })); }
   catch { a.yamlParseThrew++; return; }
   for (const doc of docs) {
      const taskLists: Record<string, unknown>[][] = [];
      if (looksLikeAnsibleTasks(doc)) taskLists.push(doc);
      else if (Array.isArray(doc)) {
         for (const play of doc) {
            if (play && typeof play === "object") {
               for (const key of ["tasks", "pre_tasks", "post_tasks", "handlers"]) {
                  const tl = (play as Record<string, unknown>)[key];
                  if (looksLikeAnsibleTasks(tl)) taskLists.push(tl);
               }
            }
         }
      }
      if (taskLists.length === 0) continue;
      a.taskFiles++; sc.taskFiles++;
      for (const list of taskLists) {
         for (const task of list) {
            if (!task || typeof task !== "object") continue;
            a.totalTasks++; sc.tasks++;
            const rawMod = moduleOf(task);
            if (!rawMod) continue;
            const mod = normalizeModule(rawMod);
            bump(a.modules, mod);
            const mutates = !ANSIBLE_CONTROL.has(mod);
            if (mutates) {
               a.mutatingTasks++; sc.mutating++;
               const args = task["args"];
               const guarded = ANSIBLE_GUARD_KEYS.some(
                  (k) => k in task || (args && typeof args === "object" && k in (args as object)),
               );
               if (guarded) { a.mutatingGuarded++; sc.mutatingGuarded++; }
            } else {
               a.controlTasks++;
            }
            for (const ck of ANSIBLE_CHECK_KEYS) {
               const args = task["args"];
               if (ck in task || (args && typeof args === "object" && ck in (args as object))) bump(a.checkKeys, ck);
            }
         }
      }
   }
}

function topN(m: Map<string, number>, n: number): [string, number][] {
   return [...m.entries()].sort((a, b) => b[1] - a[1]).slice(0, n);
}
const pct = (num: number, den: number) => (den ? ((100 * num) / den).toFixed(1) + "%" : "n/a");

function summarize(t: Tally): void {
   const s = t.shell, a = t.ansible;
   console.log("\n===== Dorc corpus scan (v2) =====");
   console.log("roots:", t.roots.join(", "));
   console.log(`files seen: ${t.filesSeen} (skipped-big ${t.filesSkippedBig}, unreadable ${t.filesUnreadable}, scaffold-skipped ${t.scaffoldFilesSkipped})`);

   console.log("\n--- SHELL (workload only; test/scaffolding excluded) ---");
   console.log(`files ${s.files} | parse-threw ${s.parseThrew} (${pct(s.parseThrew, s.files)}) | ERROR-node ${s.withErrorNode} (${pct(s.withErrorNode, s.files)})`);
   console.log(`commands ${s.totalCommands} (distinct ${s.commands.size}) | mutating ${s.mutatingCommands} (${pct(s.mutatingCommands, s.totalCommands)}) | eval ${s.evalCount} (${pct(s.evalCount, s.totalCommands)}) | dynamic-name ${s.dynamicNameCount} (${pct(s.dynamicNameCount, s.totalCommands)})`);
   console.log("top 25 commands:");
   for (const [k, v] of topN(s.commands, 25)) console.log(`  ${String(v).padStart(6)} ${k}`);

   console.log("\n--- ANSIBLE (workload only; test/scaffolding excluded) ---");
   console.log(`task-files ${a.taskFiles} | tasks ${a.totalTasks} | mutating ${a.mutatingTasks} (${pct(a.mutatingTasks, a.totalTasks)}) | control ${a.controlTasks} (${pct(a.controlTasks, a.totalTasks)})`);
   console.log(`mutating-with-guard (when/creates/removes) ${a.mutatingGuarded} (${pct(a.mutatingGuarded, a.mutatingTasks)} of mutating) — crude VALUE proxy, NOT the apply-cost x check-depth banding`);
   console.log(`yaml parse-threw ${a.yamlParseThrew} (${pct(a.yamlParseThrew, a.yamlFiles)})`);
   console.log("top 25 modules (normalized):");
   for (const [k, v] of topN(a.modules, 25)) console.log(`  ${String(v).padStart(6)} ${k}`);
   console.log("check/idempotency keys:");
   for (const [k, v] of topN(a.checkKeys, 12)) console.log(`  ${String(v).padStart(6)} ${k}`);

   console.log("\n--- BY STRATUM (Simpson's-paradox guard) ---");
   for (const [strat, c] of t.byStratum) {
      console.log(`  ${strat.padEnd(11)} shellFiles ${c.shellFiles} shellCmds ${c.shellCommands} (mut ${c.shellMutating}) | taskFiles ${c.taskFiles} tasks ${c.tasks} mut ${c.mutating} guarded ${c.mutatingGuarded}`);
   }
}

async function main(): Promise<void> {
   const args = process.argv.slice(2);
   const jsonOut = args.includes("--json");
   const limitArg = args.find((x) => x.startsWith("--limit="));
   const limit = limitArg ? Number(limitArg.split("=")[1]) : Infinity;
   const roots = args.filter((x) => !x.startsWith("--"));
   if (roots.length === 0) { console.error("usage: scan <root...> [--limit=N] [--json]"); process.exit(2); }

   const { Parser, Language } = await import("web-tree-sitter");
   await Parser.init();
   const parser = new Parser();
   // Load from bytes (path-independent; avoids Emscripten FS path quirks).
   parser.setLanguage(await Language.load(new Uint8Array(readFileSync(BASH_WASM))));

   const tally = emptyTally(roots);
   const t0 = Date.now();
   for (const root of roots) {
      for (const path of walk(root)) {
         if (tally.filesSeen >= limit) break;
         tally.filesSeen++;
         const ext = extname(path).toLowerCase();
         const isShell = SHELL_EXT.has(ext);
         const isYaml = YAML_EXT.has(ext);
         if (!isShell && !isYaml) continue;
         if (SCAFFOLD_SEG.test(path)) { tally.scaffoldFilesSkipped++; continue; }
         const src = readText(path, tally);
         if (src === null) continue;
         const stratum = stratumOf(path);
         if (isShell) tallyShell(src, parser, tally, stratum);
         else tallyAnsible(src, tally, stratum);
      }
   }
   const secs = ((Date.now() - t0) / 1000).toFixed(1);
   summarize(tally);
   console.log(`\nscanned in ${secs}s`);
   if (jsonOut) {
      const json = {
         ...tally,
         shell: { ...tally.shell, commands: Object.fromEntries(tally.shell.commands) },
         ansible: {
            ...tally.ansible,
            modules: Object.fromEntries(tally.ansible.modules),
            checkKeys: Object.fromEntries(tally.ansible.checkKeys),
         },
         byStratum: Object.fromEntries(tally.byStratum),
      };
      console.log("\n<<<JSON>>>\n" + JSON.stringify(json));
   }
}

main().catch((e) => { console.error("FATAL:", e); process.exit(1); });
