import edu.mit.csail.sdg.alloy4.A4Reporter;
import edu.mit.csail.sdg.alloy4.Err;
import edu.mit.csail.sdg.alloy4.Pos;
import edu.mit.csail.sdg.ast.Command;
import edu.mit.csail.sdg.parser.CompModule;
import edu.mit.csail.sdg.parser.CompUtil;
import edu.mit.csail.sdg.translator.A4Options;
import edu.mit.csail.sdg.translator.A4Solution;
import edu.mit.csail.sdg.translator.TranslateAlloyToKodkod;

import kodkod.engine.satlab.SATFactory;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeUnit;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class AlloyRunner {
   static final int USAGE = 2;
   // The child solving now, for the shutdown hook: a runner killed mid-command must not leave it running.
   static volatile Process live;

   public static void main(String[] args) throws Exception {
      // Kodkod logs every native-library probe at INFO, about thirty lines a run, none of them a verdict.
      System.setProperty("org.slf4j.simpleLogger.log.kodkod", "warn");
      // Both defaults sit under an agent harness's 600s foreground ceiling: a harness kill
      // mid-batch is what would otherwise orphan a solver child.
      long timeoutSeconds = 120;
      long cpuSeconds = -1;
      long batchSeconds = 540;
      Caps caps = new Caps(2048, 2);
      List<String> files = new ArrayList<>();
      List<String> given = new ArrayList<>();
      List<String> opens = new ArrayList<>();
      int one = -1;
      boolean instances = false;
      boolean parseOnly = false;
      String only = null;
      String solver = "sat4j";
      for (int i = 0; i < args.length; i++) {
         if (args[i].equals("--timeout") && i + 1 < args.length) timeoutSeconds = Long.parseLong(args[++i]);
         else if (args[i].equals("--cpu") && i + 1 < args.length) cpuSeconds = Long.parseLong(args[++i]);
         else if (args[i].equals("--batch-timeout") && i + 1 < args.length) batchSeconds = Long.parseLong(args[++i]);
         else if (args[i].equals("--heap") && i + 1 < args.length) caps = new Caps(Long.parseLong(args[++i]), caps.procs());
         else if (args[i].equals("--procs") && i + 1 < args.length) caps = new Caps(caps.heapMb(), Integer.parseInt(args[++i]));
         else if (args[i].equals("--instances")) instances = true;
         else if (args[i].equals("--parse-only")) parseOnly = true;
         else if (args[i].equals("--solver") && i + 1 < args.length) solver = args[++i];
         else if (args[i].equals("--command") && i + 1 < args.length) only = args[++i];
         else if (args[i].equals("--open") && i + 1 < args.length && args[i + 1].contains("=")) opens.add(args[++i]);
         else if (args[i].equals("--one") && i + 1 < args.length) one = Integer.parseInt(args[++i]);
         else if (args[i].startsWith("-")) usage("unknown option " + args[i]);
         else if (new File(resolve(args[i])).isDirectory()) {
            // A directory stands for its `.als` files in sorted order, so no shell glob is needed.
            File[] found = new File(resolve(args[i])).listFiles((d, name) -> name.endsWith(".als"));
            List<File> sorted = new ArrayList<>(found == null ? List.of() : List.of(found));
            sorted.sort(null);
            for (File file : sorted) {
               files.add(file.getCanonicalPath());
               given.add(file.getPath());
            }
         }
         else {
            files.add(resolve(args[i]));
            given.add(args[i]);
         }
      }
      if (files.isEmpty()) usage("no .als files given");
      if (SATFactory.find(solver).isEmpty()) usage("unknown solver " + solver + " (see `java -jar <alloy jar> solvers`)");
      if (one >= 0) {
         solveOne(files.get(0), one, opens, instances, solver);
         return;
      }
      if (parseOnly) System.exit(parseAll(files, given, opens) ? 0 : 1);

      if (cpuSeconds < 0) cpuSeconds = timeoutSeconds;
      Runtime.getRuntime().addShutdownHook(new Thread(() -> {
         Process p = live;
         if (p != null) {
            p.descendants().forEach(ProcessHandle::destroyForcibly);
            p.destroyForcibly();
         }
      }));

      // Parse everything first, so the caps line can say how much work is coming.
      CompModule[] worlds = new CompModule[files.size()];
      String[] parseErrors = new String[files.size()];
      int total = 0;
      boolean anyParseError = false;
      List<String> labels = new ArrayList<>();
      for (int f = 0; f < files.size(); f++) {
         try {
            worlds[f] = CompUtil.parseEverything_fromFile(A4Reporter.NOP, overlay(files.get(f), opens), files.get(f));
            for (Command cmd : worlds[f].getAllCommands()) {
               if (synthesized(worlds[f], cmd)) continue;
               labels.add(cmd.label);
               if (only == null || only.equals(cmd.label)) total++;
            }
         } catch (Err e) {
            anyParseError = true;
            parseErrors[f] = e.toString();
         }
      }
      // An unparsed file may hold the command, so its error row, not this refusal, is the honest answer.
      if (only != null && total == 0 && !anyParseError) {
         System.err.println("alloy runner: --command " + only + " matches no command; available: " + String.join(", ", labels));
         System.exit(USAGE);
      }
      System.err.println("alloy runner: " + total + " commands; caps: " + timeoutSeconds + "s wall and " + cpuSeconds
         + "s cpu per command, " + caps.heapMb() + " MB heap, " + caps.procs() + " processors, " + batchSeconds + "s for the batch");

      boolean red = false;
      boolean first = true;
      long batchStart = System.nanoTime();
      System.out.println("[");
      for (int f = 0; f < files.size(); f++) {
         String path = files.get(f);
         CompModule world = worlds[f];
         if (world == null) {
            red = true;
            first = row(first, given.get(f), null, null, null, -1, "error", -1, -1, parseErrors[f], null);
            continue;
         }
         String module = world.getModuleName();
         List<Command> commands = world.getAllCommands();
         for (int i = 0; i < commands.size(); i++) {
            Command cmd = commands.get(i);
            if (synthesized(world, cmd) || (only != null && !only.equals(cmd.label))) continue;
            String kind = cmd.check ? "check" : "run";
            if ((System.nanoTime() - batchStart) / 1_000_000_000L >= batchSeconds) {
               red = true;
               first = row(first, module, cmd.label, kind, scopeOf(cmd), cmd.expects, "not-run", -1, -1, "the batch cap of " + batchSeconds + "s was reached before this command started", null);
               continue;
            }
            Outcome o = runChild(path, i, opens, instances, solver, timeoutSeconds, cpuSeconds, caps);
            String result = switch (o.status) {
               case "sat" -> cmd.check ? "counterexample" : "sat";
               case "unsat" -> cmd.check ? "no-counterexample" : "unsat";
               default -> o.status;
            };
            if (!agrees(result, cmd.expects)) red = true;
            first = row(first, module, cmd.label, kind, scopeOf(cmd), cmd.expects, result, o.wallMs, o.solveMs, o.message, o.instance);
         }
      }
      System.out.println("]");
      System.exit(red ? 1 : 0);
   }

   // Alloy invents a `Default` run for a module that declares no command; it carries no source
   // position, which an authored `run Default` always does (measured).
   static boolean synthesized(CompModule world, Command cmd) {
      return world.getAllCommands().size() == 1 && cmd.pos == Pos.UNKNOWN;
   }

   // An `expect` is the author's recorded verdict, so disagreeing with it is the red; without
   // one, a run is meant to be satisfiable and a check to hold. Errors and caps are always red.
   static boolean agrees(String result, int expects) {
      return switch (expects) {
         case 1 -> result.equals("sat") || result.equals("counterexample");
         case 0 -> result.equals("unsat") || result.equals("no-counterexample");
         default -> result.equals("sat") || result.equals("no-counterexample");
      };
   }

   // One JVM, no solver: whether each root parses and typechecks, as one row per file.
   static boolean parseAll(List<String> files, List<String> given, List<String> opens) throws Exception {
      boolean ok = true;
      System.out.println("[");
      for (int f = 0; f < files.size(); f++) {
         String module = given.get(f);
         String message = null;
         try {
            module = CompUtil.parseEverything_fromFile(A4Reporter.NOP, overlay(files.get(f), opens), files.get(f)).getModuleName();
         } catch (Err e) {
            ok = false;
            message = e.toString();
         }
         StringBuilder b = new StringBuilder(f == 0 ? "  {" : ", {");
         b.append("\"module\": ").append(json(module));
         b.append(", \"file\": ").append(json(given.get(f)));
         b.append(", \"result\": ").append(json(message == null ? "parsed" : "error"));
         if (message != null) b.append(", \"message\": ").append(json(message));
         System.out.println(b.append("}"));
      }
      System.out.println("]");
      return ok;
   }

   static void usage(String why) {
      System.err.println("alloy runner: " + why + "\nusage: AlloyRunner [--timeout <seconds>] [--cpu <seconds>] [--heap <MB>] [--procs <n>] [--batch-timeout <seconds>] [--parse-only] [--command <name>] [--instances] [--solver <id>] [--open <module>=<file.als>]... <file.als>...");
      System.exit(USAGE);
   }

   static String resolve(String arg) throws Exception {
      String origin = System.getenv("MISE_ORIGINAL_CWD");
      File file = origin != null && !new File(arg).isAbsolute() ? new File(origin, arg) : new File(arg);
      return file.getCanonicalPath();
   }

   // Alloy resolves every `open X` against ONE root: the root file's directory with the root's own
   // `module a/b` path stripped. `--open X=file` serves file's text at that spot without copying it.
   static Map<String, String> overlay(String rootPath, List<String> opens) throws Exception {
      Map<String, String> loaded = new HashMap<>();
      if (opens.isEmpty()) return loaded;
      Matcher m = Pattern.compile("(?m)^\\s*module\\s+([A-Za-z0-9_/'\"]+)").matcher(Files.readString(Path.of(rootPath)));
      int depth = m.find() ? m.group(1).split("/").length - 1 : 0;
      File root = new File(rootPath).getParentFile();
      for (int d = 0; d < depth && root.getParentFile() != null; d++) root = root.getParentFile();
      for (String spec : opens) {
         String[] kv = spec.split("=", 2);
         loaded.put(new File(root, kv[0] + ".als").getCanonicalPath(), Files.readString(Path.of(resolve(kv[1]))));
      }
      return loaded;
   }

   // Command.toString() is "Check name for 6 but ..."; everything after " for " is the scope clause as Alloy read it.
   static String scopeOf(Command cmd) {
      String s = cmd.toString();
      int at = s.indexOf(" for ");
      return at < 0 ? "" : s.substring(at + 5);
   }

   record Outcome(String status, long wallMs, long solveMs, String message, String instance) {}

   record Caps(long heapMb, int procs) {}

   // One JVM per command, so a blown-up scope is killed at a cap instead of hanging the batch.
   static Outcome runChild(String path, int index, List<String> opens, boolean instances, String solver, long timeoutSeconds, long cpuSeconds, Caps caps) throws Exception {
      List<String> argv = new ArrayList<>(List.of(ProcessHandle.current().info().command().orElse("java"),
         "-Xmx" + caps.heapMb() + "m", "-XX:ActiveProcessorCount=" + caps.procs(),
         "-cp", System.getProperty("java.class.path"), System.getProperty("jdk.launcher.sourcefile"),
         "--one", Integer.toString(index), "--solver", solver));
      for (String spec : opens) argv.addAll(List.of("--open", spec));
      if (instances) argv.add("--instances");
      argv.add(path);
      Path out = Files.createTempFile("alloy-runner-", ".out");
      Path err = Files.createTempFile("alloy-runner-", ".err");
      try {
         ProcessBuilder pb = new ProcessBuilder(argv);
         pb.redirectOutput(out.toFile()).redirectError(err.toFile());
         long start = System.nanoTime();
         Process p = pb.start();
         live = p;
         // Polled once a second: the wall-clock cap, and the CPU-time cap a multi-threaded solver
         // can reach first.
         String exceeded = null;
         while (!p.waitFor(1, TimeUnit.SECONDS)) {
            if ((System.nanoTime() - start) / 1_000_000_000L >= timeoutSeconds) exceeded = timeoutSeconds + "s wall-clock";
            else if (p.toHandle().info().totalCpuDuration().map(Duration::getSeconds).orElse(0L) >= cpuSeconds) exceeded = cpuSeconds + "s cpu";
            if (exceeded != null) break;
         }
         long wallMs = (System.nanoTime() - start) / 1_000_000;
         if (exceeded != null) {
            p.descendants().forEach(ProcessHandle::destroyForcibly);
            p.destroyForcibly().waitFor();
            String progress = Files.readAllLines(err, StandardCharsets.UTF_8).stream()
               .filter(l -> l.startsWith("translated ")).reduce((a, b) -> b).orElse("still translating");
            return new Outcome("timeout", wallMs, -1, "exceeded " + exceeded + "; " + progress, null);
         }
         String stdout = Files.readString(out, StandardCharsets.UTF_8).strip();
         String[] lines = stdout.split("\n", 2);
         String[] parts = lines[0].strip().split("\t", 3);
         String instance = lines.length == 2 ? lines[1].strip() : null;
         if (p.exitValue() == 0 && parts.length == 2 && (parts[0].equals("sat") || parts[0].equals("unsat")))
            return new Outcome(parts[0], wallMs, Long.parseLong(parts[1]), null, instance);
         if (parts.length == 3 && parts[0].equals("error")) return new Outcome("error", wallMs, -1, stdout.split("\t", 3)[2], null);
         String stderr = Files.readString(err, StandardCharsets.UTF_8).strip();
         return new Outcome("error", wallMs, -1, "exit " + p.exitValue() + ": " + (stderr.isEmpty() ? stdout : stderr), null);
      } finally {
         live = null;
         Files.deleteIfExists(out);
         Files.deleteIfExists(err);
      }
   }

   static void solveOne(String path, int index, List<String> opens, boolean instances, String solver) throws Exception {
      try {
         CompModule world = CompUtil.parseEverything_fromFile(A4Reporter.NOP, overlay(path, opens), path);
         Command cmd = world.getAllCommands().get(index);
         A4Options options = new A4Options();
         options.solver = SATFactory.get(solver);
         long start = System.nanoTime();
         // On stderr, so a killed child still says whether translation finished and how large the SAT problem was.
         A4Reporter progress = new A4Reporter() {
            @Override
            public void solve(int step, int primaryVars, int totalVars, int clauses) {
               System.err.println("translated in " + (System.nanoTime() - start) / 1_000_000 + "ms: "
                  + primaryVars + " primary vars, " + totalVars + " vars, " + clauses + " clauses");
               System.err.flush();
            }
         };
         A4Solution sol = TranslateAlloyToKodkod.execute_command(progress, world.getAllReachableSigs(), cmd, options);
         long solveMs = (System.nanoTime() - start) / 1_000_000;
         System.out.println((sol.satisfiable() ? "sat" : "unsat") + "\t" + solveMs);
         if (instances && sol.satisfiable()) System.out.println(sol);
      } catch (Err e) {
         System.out.println("error\t-1\t" + e.toString());
         System.exit(1);
      }
   }

   static boolean row(boolean first, String module, String command, String kind, String scope, int expects, String result, long wallMs, long solveMs, String message, String instance) {
      StringBuilder b = new StringBuilder(first ? "  {" : ", {");
      b.append("\"module\": ").append(json(module));
      b.append(", \"command\": ").append(json(command));
      b.append(", \"kind\": ").append(json(kind));
      b.append(", \"scope\": ").append(json(scope));
      if (expects == 0 || expects == 1) b.append(", \"expect\": ").append(expects);
      b.append(", \"result\": ").append(json(result));
      b.append(", \"wall_ms\": ").append(wallMs < 0 ? "null" : Long.toString(wallMs));
      b.append(", \"solve_ms\": ").append(solveMs < 0 ? "null" : Long.toString(solveMs));
      if (message != null) b.append(", \"message\": ").append(json(message));
      if (instance != null) b.append(", \"instance\": ").append(json(instance));
      System.out.println(b.append("}"));
      System.out.flush();
      return false;
   }

   static String json(String s) {
      if (s == null) return "null";
      StringBuilder b = new StringBuilder("\"");
      for (char c : s.toCharArray()) {
         switch (c) {
            case '"' -> b.append("\\\"");
            case '\\' -> b.append("\\\\");
            case '\n' -> b.append("\\n");
            case '\r' -> b.append("\\r");
            case '\t' -> b.append("\\t");
            default -> {
               if (c < 0x20) b.append(String.format("\\u%04x", (int) c));
               else b.append(c);
            }
         }
      }
      return b.append('"').toString();
   }
}
