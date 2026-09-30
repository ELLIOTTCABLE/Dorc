import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonNull;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

import edu.mit.csail.sdg.alloy4.A4Reporter;
import edu.mit.csail.sdg.alloy4.Err;
import edu.mit.csail.sdg.alloy4.Pos;
import edu.mit.csail.sdg.alloy4.XMLNode;
import edu.mit.csail.sdg.ast.Command;
import edu.mit.csail.sdg.ast.Expr;
import edu.mit.csail.sdg.ast.Sig;
import edu.mit.csail.sdg.parser.CompModule;
import edu.mit.csail.sdg.parser.CompUtil;
import edu.mit.csail.sdg.translator.A4Options;
import edu.mit.csail.sdg.translator.A4Solution;
import edu.mit.csail.sdg.translator.A4SolutionReader;
import edu.mit.csail.sdg.translator.TranslateAlloyToKodkod;

import kodkod.engine.satlab.SATFactory;
import kodkod.engine.satlab.SATSolver;
import kodkod.solvers.SAT4J;
import kodkod.solvers.SAT4JRef;

import org.sat4j.minisat.SolverFactory;
import org.sat4j.specs.ISolver;

import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
import java.io.PrintStream;
import java.io.PrintWriter;
import java.io.StringReader;
import java.io.StringWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class AlloyAdapter {
   static final PrintStream OUT = new PrintStream(new java.io.FileOutputStream(java.io.FileDescriptor.out), true, StandardCharsets.UTF_8);
   static final Map<String, CompModule> worlds = new HashMap<>();

   // Here and not on the factory: the ticker touching the factory class first would initialise
   // SAT4JRef ahead of SATFactory, whose static init then reads a still-null SAT4JRef.INSTANCE.
   static volatile ISolver solving;

   /** Kodkod's own sat4j factory, keeping the solver it hands out so the ticker can read its effort. */
   static final class WatchedSat4j extends SAT4JRef {
      @Override
      public SATSolver createSolver() {
         ISolver solver = SolverFactory.instance().defaultSolver();
         solving = solver;
         return new SAT4J(solver);
      }
   }

   public static void main(String[] args) throws Exception {
      System.setProperty("org.slf4j.simpleLogger.log.kodkod", "warn");
      System.setOut(new PrintStream(new java.io.OutputStream() { public void write(int b) {} }));
      Thread ticker = new Thread(() -> {
         while (true) {
            // A caller killed mid-solve closes no pipe this thread would notice.
            if (!ProcessHandle.current().parent().map(ProcessHandle::isAlive).orElse(false)) Runtime.getRuntime().halt(1);
            emit(tick());
            try { Thread.sleep(1000); } catch (InterruptedException e) { return; }
         }
      });
      ticker.setDaemon(true);
      ticker.start();
      BufferedReader in = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
      for (String line; (line = in.readLine()) != null; ) {
         if (line.isBlank()) continue;
         JsonObject req = JsonParser.parseString(line).getAsJsonObject();
         String verb = req.get("verb").getAsString();
         JsonObject reply;
         try {
            reply = switch (verb) {
               case "parse" -> parse(req, true);
               case "parse-only" -> parse(req, false);
               case "solve" -> solve(req);
               case "eval" -> eval(req);
               default -> failure("unknown verb " + verb, null);
            };
         } catch (OutOfMemoryError e) {
            worlds.clear();
            reply = new JsonObject();
            reply.addProperty("result", "out-of-memory");
            reply.addProperty("message", String.valueOf(e.getMessage()));
         } catch (Err e) {
            reply = failure(e.toString(), e.pos);
         } catch (Exception e) {
            reply = failure(e.toString(), null);
         }
         reply.addProperty("verb", verb);
         emit(reply);
      }
   }

   static synchronized void emit(JsonObject o) {
      OUT.println(o.toString());
      OUT.flush();
   }

   static long cpuMs() {
      return ProcessHandle.current().info().totalCpuDuration().map(Duration::toMillis).orElse(0L);
   }

   static JsonObject event(String name) {
      JsonObject o = new JsonObject();
      o.addProperty("event", name);
      o.addProperty("cpu_ms", cpuMs());
      return o;
   }

   // Read unsynchronised from the solving thread's live counters: they are shown, never decided on,
   // so a stale value costs nothing. Anything thrown here would stop the ticks the CPU cap needs.
   static JsonObject tick() {
      JsonObject o = event("tick");
      ISolver solver = solving;
      if (solver == null) return o;
      try {
         Map<String, Number> stat = solver.getStat();
         String[][] fields = {{"conflicts", "conflicts"}, {"restarts", "starts"}, {"learned", "learnedclauses"}, {"decisions", "decisions"}};
         for (String[] f : fields) if (!stat.containsKey(f[1])) return o;
         for (String[] f : fields) o.addProperty(f[0], stat.get(f[1]).longValue());
      } catch (Throwable ignored) {
      }
      return o;
   }

   static JsonObject failure(String message, Pos pos) {
      JsonObject o = new JsonObject();
      o.addProperty("ok", false);
      o.addProperty("result", "error");
      o.addProperty("message", message);
      if (pos != null && pos != Pos.UNKNOWN) {
         o.addProperty("file", pos.filename);
         o.addProperty("line", pos.y);
         o.addProperty("column", pos.x);
      }
      return o;
   }

   static File resolutionRoot(String root) throws Exception {
      java.util.regex.Matcher m = java.util.regex.Pattern.compile("(?m)^\\s*module\\s+([A-Za-z0-9_/'\"]+)").matcher(Files.readString(Path.of(root)));
      int depth = m.find() ? m.group(1).split("/").length - 1 : 0;
      File dir = new File(root).getCanonicalFile().getParentFile();
      for (int d = 0; d < depth && dir.getParentFile() != null; d++) dir = dir.getParentFile();
      return dir;
   }

   static Map<String, String> overlay(String root, JsonObject opens) throws Exception {
      Map<String, String> loaded = new HashMap<>();
      if (opens == null) return loaded;
      File dir = resolutionRoot(root);
      for (Map.Entry<String, JsonElement> e : opens.entrySet())
         loaded.put(new File(dir, e.getKey() + ".als").getCanonicalPath(), Files.readString(Path.of(e.getValue().getAsString())));
      return loaded;
   }

   static JsonObject parse(JsonObject req, boolean full) throws Exception {
      String root = req.get("root").getAsString();
      JsonObject opens = req.has("opens") ? req.getAsJsonObject("opens") : null;
      Map<String, String> loaded = overlay(root, opens);
      CompModule world = CompUtil.parseEverything_fromFile(A4Reporter.NOP, loaded, root);
      worlds.put(root, world);
      JsonObject o = new JsonObject();
      o.addProperty("ok", true);
      o.addProperty("module", world.getModuleName());
      if (!full) return o;
      String base = resolutionRoot(root).getCanonicalPath();
      JsonArray files = new JsonArray();
      for (Map.Entry<String, String> e : new TreeMap<>(loaded).entrySet()) {
         JsonObject f = new JsonObject();
         f.addProperty("path", relative(base, e.getKey()));
         f.addProperty("text", e.getValue());
         files.add(f);
      }
      o.add("loaded", files);
      JsonArray commands = new JsonArray();
      List<Command> all = world.getAllCommands();
      for (int i = 0; i < all.size(); i++) {
         Command c = all.get(i);
         JsonObject j = new JsonObject();
         j.addProperty("index", i);
         j.addProperty("label", c.label);
         j.addProperty("check", c.check);
         if (c.expects == 0 || c.expects == 1) j.addProperty("expects", c.expects);
         j.addProperty("scope", scopeOf(c));
         j.addProperty("bitwidth", c.bitwidth < 0 ? 4 : c.bitwidth);
         j.addProperty("unbounded_steps", c.maxprefix == Integer.MAX_VALUE);
         j.addProperty("temporal", CompUtil.isTemporalModel(world.getAllReachableSigs(), c));
         j.addProperty("synthesized", all.size() == 1 && c.pos == Pos.UNKNOWN);
         commands.add(j);
      }
      o.add("commands", commands);
      JsonArray sigs = new JsonArray();
      for (Sig s : world.getAllReachableSigs()) {
         if (s.builtin) continue;
         JsonObject j = new JsonObject();
         j.addProperty("label", s.label);
         JsonArray fields = new JsonArray();
         for (Sig.Field f : s.getFields()) fields.add(f.label);
         j.add("fields", fields);
         sigs.add(j);
      }
      o.add("sigs", sigs);
      o.add("options", describe(options(req)));
      return o;
   }

   static String relative(String base, String path) {
      String p = path.replace('\\', '/');
      int lib = p.indexOf("$alloy4$/");
      if (lib >= 0) return p.substring(lib);
      String b = base.replace('\\', '/') + "/";
      return p.startsWith(b) ? p.substring(b.length()) : p;
   }

   static String scopeOf(Command cmd) {
      String s = cmd.toString().replaceFirst(" expect -?\\d+$", "");
      int at = s.indexOf(" for ");
      return at < 0 ? "" : s.substring(at + 5);
   }

   static A4Options options(JsonObject req) {
      A4Options opt = new A4Options();
      String solver = req.has("solver") ? req.get("solver").getAsString() : "sat4j";
      if (SATFactory.find(solver).isEmpty()) throw new IllegalArgumentException("unknown solver " + solver);
      opt.solver = solver.equals("sat4j") ? new WatchedSat4j() : SATFactory.get(solver);
      return opt;
   }

   static JsonObject describe(A4Options opt) {
      JsonObject o = new JsonObject();
      o.addProperty("solver", opt.solver.id());
      o.addProperty("symmetry", opt.symmetry);
      o.addProperty("skolem_depth", opt.skolemDepth);
      o.addProperty("no_overflow", opt.noOverflow);
      o.addProperty("unrolls", opt.unrolls);
      o.addProperty("decompose_mode", opt.decompose_mode);
      o.addProperty("decompose_threads", opt.decompose_threads);
      o.addProperty("infer_partial_instance", opt.inferPartialInstance);
      o.addProperty("core_minimization", opt.coreMinimization);
      o.addProperty("core_granularity", opt.coreGranularity);
      return o;
   }

   static CompModule world(JsonObject req) throws Exception {
      String root = req.get("root").getAsString();
      CompModule w = worlds.get(root);
      if (w == null) {
         parse(req, false);
         w = worlds.get(root);
      }
      return w;
   }

   static JsonObject solve(JsonObject req) throws Exception {
      CompModule world = world(req);
      Command cmd = world.getAllCommands().get(req.get("index").getAsInt());
      A4Options opt = options(req);
      emit(event("start"));
      long start = System.nanoTime();
      JsonObject size = new JsonObject();
      A4Reporter progress = new A4Reporter() {
         @Override
         public void solve(int step, int primaryVars, int totalVars, int clauses) {
            JsonObject e = event("translated");
            e.addProperty("ms", (System.nanoTime() - start) / 1_000_000);
            e.addProperty("primary_vars", primaryVars);
            e.addProperty("vars", totalVars);
            e.addProperty("clauses", clauses);
            size.addProperty("primary_vars", primaryVars);
            size.addProperty("vars", totalVars);
            size.addProperty("clauses", clauses);
            emit(e);
         }
      };
      A4Solution sol;
      try {
         sol = TranslateAlloyToKodkod.execute_command(progress, world.getAllReachableSigs(), cmd, opt);
      } finally {
         // Otherwise the finished solver's clause database outlives the command into the next one's heap.
         solving = null;
      }
      JsonObject o = new JsonObject();
      o.addProperty("ok", true);
      o.addProperty("result", sol.satisfiable() ? "sat" : "unsat");
      o.addProperty("solve_ms", (System.nanoTime() - start) / 1_000_000);
      o.add("options", describe(opt));
      o.add("size", size.size() == 0 ? JsonNull.INSTANCE : size);
      if (sol.satisfiable() && req.has("xml") && req.get("xml").getAsBoolean()) {
         StringWriter xml = new StringWriter();
         PrintWriter w = new PrintWriter(xml);
         sol.writeXML(w, new ArrayList<>(), new HashMap<>());
         w.flush();
         o.addProperty("instance_xml", xml.toString());
      }
      if (sol.satisfiable() && req.has("text") && req.get("text").getAsBoolean()) o.addProperty("instance", sol.toString());
      return o;
   }

   static JsonObject eval(JsonObject req) throws Exception {
      CompModule world = world(req);
      A4Solution sol = A4SolutionReader.read(world.getAllReachableSigs(), new XMLNode(new StringReader(req.get("instance_xml").getAsString())));
      Expr formula = req.has("expr")
         ? world.parseOneExpressionFromString(req.get("expr").getAsString())
         : world.getAllCommands().get(req.get("index").getAsInt()).formula.and(world.getAllReachableFacts());
      Object value = sol.eval(formula);
      JsonObject o = new JsonObject();
      o.addProperty("ok", true);
      o.addProperty("value", Boolean.TRUE.equals(value));
      o.addProperty("bitwidth", sol.getBitwidth());
      return o;
   }
}
