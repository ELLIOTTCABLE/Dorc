# ═══════════════════════════════════════════════════════════════════════════
#  FROZEN EVIDENCE · STRAWMAN · IMAGINATION-TIER
#  NOT RUNNABLE — NEVER EVALUATE, BUILD, OR IMPORT THIS FILE.
#  `dorc compile --fragment`, the `dorc` nixpkgs derivation, and the option
#  namespace below DO NOT EXIST. No format-, option-name-, or wire-compat is
#  promised. Real Home Manager option names, real generated-script variables,
#  and real nixpkgs library functions appear only so the exhibit is grounded
#  in how Home Manager actually behaves (source read 2026-07-28:
#  nix-community/home-manager @ master, modules/home-environment.nix).
# ═══════════════════════════════════════════════════════════════════════════
#
#  The SPLICE FACE — charter shape B: splice a store-path invocation line
#  into somebody else's generated script, rather than splicing compiled text.
#
#  Home Manager's activation slot has no per-block file: every
#  `home.activation.<name>` entry is a `types.str` that gets topologically
#  sorted and CONCATENATED into one generated bash script
#  (`pkgs.writeShellScript "activation-script"`, `set -eu` + `set -o
#  pipefail`, `cd $HOME`). So there is no seat for "point Home Manager at our
#  file" — the only thing we can contribute is TEXT that lands inside a bash
#  script we do not own.
#
#  Shape B says: contribute exactly one line of text, and let that line be a
#  store-path invocation of the real thing. Three properties fall out, and
#  all three are gifts from nix rather than machinery we build:
#
#  - HYGIENE IS FREE. A separate process cannot clobber `$oldGenPath`,
#    `$newGenPath`, `$VERBOSE_ARG`, or the host script's shell options. The
#    subshell-wrapper every other splice target needs is not needed here.
#  - PATH INDEPENDENCE IS REQUIRED, and we get it. `home.emptyActivationPath`
#    defaults to TRUE at stateVersion >= 22.11, documented as "It is
#    recommended to keep this at true to avoid uncontrolled use of tools found
#    in PATH." A spliced `dorc-run` resolved through PATH would be exactly the
#    uncontrolled use that option exists to prevent. A store path is not
#    merely idiomatic here; it is the only correct spelling.
#  - PROVENANCE IS FREE. The charter names embedding provenance / a
#    source-map as the fragment render's one real machinery ask. In the store
#    the fragment's identity IS its path: globally unique, immutable, derived
#    from its bytes. Every other splice target has to invent this.

{ config, lib, pkgs, ... }:

let
   cfg = config.dorc;

   # One activation entry per declared fragment. The entry is a single line.
   mkEntry = name: frag:
      lib.hm.dag.entryAfter [ "writeBoundary" ] ''
         run ${cfg.package}/bin/dorc-run \
            ${lib.optionalString (frag.path != [ ]) ''--path=${lib.makeBinPath frag.path} \''}
            --whylog=''${XDG_STATE_HOME:-$HOME/.local/state}/dorc/hm-${name} \
            ''${DRY_RUN:+--plan-only} ''${VERBOSE:+--verbose} \
            -- ${frag.source}
      '';

in {
   options.dorc = {
      package = lib.mkOption {
         type = lib.types.package;
         description = "The dorc derivation whose store path gets spliced.";
      };

      activation = lib.mkOption {
         default = { };
         description = ''
            Books to run as Home Manager activation blocks. Each is plain sh
            that also runs bare, off this machine, with no nix in sight.
         '';
         type = lib.types.attrsOf (lib.types.submodule {
            options = {
               source = lib.mkOption {
                  type = lib.types.path;
                  description = ''
                     Path to the book. Copied into the store, so the spliced
                     invocation names immutable bytes.

                     STRAWMAN NARROWING, and the load-bearing one: this path is
                     run through `dorc compile --fragment` at BUILD time, which
                     REFUSES rather than rewrites. A book containing `exit`
                     does not compile, because an `exit` spliced into Home
                     Manager's script would take the whole activation with it —
                     every block sorted after ours, silently. Refusal is a
                     narrowing (`chef-solo` no-semantic-fork rule: the fragment
                     face may narrow, never change meaning). We do NOT quietly
                     rewrite `exit` to `return`; that would be a second
                     dialect, which is the trap chef-solo died in.
                  '';
               };

               path = lib.mkOption {
                  type = lib.types.listOf lib.types.package;
                  default = [ ];
                  example = lib.literalExpression "[ pkgs.git pkgs.openssh ]";
                  description = ''
                     Packages whose `bin` dirs are prepended to the book's PATH.

                     This is the splice's hardest interface and it is worth
                     saying plainly rather than hiding: nix's entire value
                     proposition is that nothing resolves by name, and sh's
                     entire idiom is that everything does. The book says `git`
                     because a book that said
                     `/nix/store/xxx-git-2.51.0/bin/git` would not be a book
                     any more — it would not run on the laptop that has no
                     nix, and the off-ramp is the product.

                     So the dependency declaration moves UP, to the nix layer,
                     where closure-completeness is expressible, and the book
                     stays name-resolving sh. Declaring nothing is legal and
                     means "whatever Home Manager put in PATH", which under
                     `emptyActivationPath` is bash, coreutils, diffutils,
                     findutils, gettext, gnugrep, gnused, jq, ncurses, and nix.
                     Not git.
                  '';
               };
            };
         });
      };
   };

   config = lib.mkIf (cfg.activation != { }) {
      home.activation = lib.mapAttrs mkEntry cfg.activation;
   };
}

# ── Why AFTER the write boundary, and never straddling it ──────────────────
#
# Home Manager's DAG has a native two-phase split that reads, at first
# glance, exactly like Dorc's probe/apply split. Verbatim from the
# `home.activation` option description:
#
#   "If the script block produces any observable side effect, such as
#    writing or deleting files, then it *must* be placed after the special
#    `writeBoundary` script block. Prior to the write boundary one can place
#    script blocks that verifies, but does not modify, the state of the
#    system and exits if an unexpected state is found."
#
# The tempting design is to split ourselves across it: probe before, apply
# after. That design is WRONG, and the reason is the most useful thing in
# this file.
#
# `writeBoundary` is where Home Manager commits to writing the home
# directory. Between a pre-boundary probe and a post-boundary apply, Home
# Manager rewrites an arbitrary amount of $HOME and installs a new profile.
# That is a poison wall — an unmodeled command that may invalidate anything
# measured above it — and it is a wall we would have INSERTED OURSELVES, by
# choosing to straddle. Every fact the probe measured would need a guard, so
# the split would buy exactly zero elisions and cost an extra phase.
#
# One post-boundary entry, doing its own probe and apply back-to-back with
# nothing between them, has no staleness by construction. That is the same
# reasoning that makes an in-sequence guard epoch-proof.
#
# The pre-boundary slot is still worth something, but for a different job:
# Home Manager's own words are "verifies, but does not modify ... and exits
# if an unexpected state is found". That is a refuse-early seat, not a probe
# seat — "the private remote is unreachable, do not begin" — and it wants a
# separate, tiny option rather than half of this one.
#
# ── The mandate-idempotence-assist-nothing tally gains a member, in source ──
#
# Verbatim, the same option's description: "Any entry here should be
# idempotent, meaning running twice or more times produces the same result as
# running it once." Mandated by doc, assisted by nothing. That is now
# first-party-sourced for Home Manager rather than inferred, joining chezmoi,
# yadm, dotbot, k8s init containers, and the rest.
#
# ── What we do NOT get, and must not claim ─────────────────────────────────
#
# `DRY_RUN` above is honest only up to a point. Home Manager's contract is "a
# script block should respect DRY_RUN ... the actions taken by the script
# should be logged to standard out and not actually performed", and `dorc
# plan` genuinely is that. But Home Manager's OWN dry run and ours answer
# different questions: theirs is "what would activation do", ours is "what
# does this book still have left to do against the live host". Under a real
# `DRY_RUN`, the home directory has NOT been rewritten, so our plan is
# computed against a pre-activation world and can differ from what the same
# book would do post-activation. The plan is honest about the world it
# measured; it is not a forecast of a world that does not exist yet.
