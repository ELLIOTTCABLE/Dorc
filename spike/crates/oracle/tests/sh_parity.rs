//! The sh-parity pin battery, oracle tier (`30A` — the doctrine and its inventory).
//!
//! Every pin here asserts a NAME-BINDING fact about sh and then asserts what the loaded-source index
//! does with it, because `rul-unsure-falls-toward-sh-parity` makes sh the answer wherever the engine
//! is unsure. The facts themselves are measured, not argued: the `floor28-*`/`floor30-*` manifests ran
//! these shapes under `posh ∩ dash` and their `expected.emitted` sections are what the shells said.
//!
//! What is NOT here, deliberately. `p-last-wins-helper-binding` (`30A` §2) is already pinned in full
//! by `closure.rs`'s own tests — `a_plural_helper_resolving_into_the_vouchers_file_still_ships`
//! asserts the LAST declaration travels and the shadowed one does not, and the three custody arms sit
//! beside it. Restating it here would be a second copy of one law.
//!
//! The pipeline tier (does a site keep its LICENSE?) is `crates/cli/tests/sh_parity.rs`.

use dorc_oracle::closure::{DenialReason, HelperIndex};

const MARKER: &str = "# dorc-lang/v0.2\n";

/// The verdict body every cell below vouches with — one call to `_dest`, and nothing else, so what
/// the closure carries is unambiguous.
const VOUCHER_BODY: &str = "wombat__is_converged() {\n   _dest \"$1\"\n}";

/// The voucher beside its OWN `_dest`, whose body carries `tag` so a shipped closure's bytes name
/// their author. One file, because `rul-vouch-reaches-own-custody-only` de-licenses every reach out
/// of the voucher's custody: a two-file setup would measure the custody rule rather than the
/// name-binding fact each pin below is about.
fn voucher_owning_dest(tag: &str) -> String {
    format!("{MARKER}_dest() {{\n   wombat cmp --{tag} -- \"$1\"\n}}\n{VOUCHER_BODY}\n")
}

/// `p-helper-unset-f` (`30A` §2 P-green) — after `unset -f _dest` the name resolves to NOTHING, and a
/// body reaching it ships without it rather than borrowing the declaration above.
///
/// The sh fact: `unset -f` removes a function definition, so a call below it is an ordinary
/// command-word lookup again. Measured under `posh ∩ dash` by
/// `floor28-unset-f-and-redefinition` — which is also why this is a genuine parity assertion and not
/// an engine preference.
///
/// HOW the engine gets there is worth stating, because it is not by modelling `unset -f`: a top-level
/// `unset -f` is a COMMAND, so the file is not load-inert
/// (`oracle/CLAUDE.md only-load-inert-sources-contribute`) and contributes NO declarations at all.
/// The outcome is right and the reasoning is blunter than the rule — which is why the cross-file
/// cousin below does NOT come out right, and why the coming blessing of read-only top-level commands
/// has to arrive with a real model rather than a widened allow-list.
///
/// Why an engine choice depends on it: a borrowed-back helper would put a body the shell had unbound
/// into a shipped check, so the check's answer would come from a judgment no execution would have
/// reached — `271:rul-sin-ordering`'s mis-attribution tier.
#[test]
fn a_helper_unset_at_oracle_top_level_resolves_to_nothing() {
    let unset_beside_it = format!(
        "{MARKER}_dest() {{\n   wombat cmp -- \"$1\"\n}}\nunset -f _dest\n{VOUCHER_BODY}\n"
    );
    let index = HelperIndex::build(&[&unset_beside_it], None);
    let closure = index
        .closure_for(0, VOUCHER_BODY)
        .expect("nothing is contested — the name simply resolves to no declaration")
        .sh();
    assert_eq!(
        closure, "",
        "the name binds nothing below the `unset -f`, so the body ships alone (and declines at rc \
         127 on the host, which is the safe direction)"
    );

    let control = format!("{MARKER}_dest() {{\n   wombat cmp -- \"$1\"\n}}\n{VOUCHER_BODY}\n");
    let control_closure = HelperIndex::build(&[&control], None)
        .closure_for(0, VOUCHER_BODY)
        .expect("one source")
        .sh();
    assert!(
        control_closure.contains("_dest() {"),
        "control: without the `unset -f` the same file's declaration DOES travel, so the emptiness \
         above is the unset's doing:\n{control_closure}"
    );
}

/// `p-x-helper-unset-f-across-files` — THE TARGET: an `unset -f` in a LATER loaded source removes the
/// earlier declaration from resolution, exactly as it removes it from a shell.
///
/// The sh fact is the same one the pin above rests on, one file over: load `a.sh` then `b.sh` where
/// `b.sh` unsets what `a.sh` defined, and the name is unbound afterwards.
/// `floor28-unset-f-and-redefinition` measured the removal under both floor binaries; nothing about
/// crossing a file boundary changes it, because `.`-sourcing applies definitions into ONE environment.
///
/// FAILS TODAY, measured 2026-08-16: the later file is not load-inert, so it contributes nothing —
/// including its removal — and `_dest` still resolves to the earlier declaration, which then TRAVELS
/// into the shipped closure. The definition table models `unset -f` for ROLE names
/// (`analysis::funcenv`'s `Undefined`, the blessing behind `contest28-unset-f-blesses-elision`), but
/// helpers are not role funcdefs and are not in it, so the helper lane has no model of removal at all.
///
/// Why an engine choice depends on it: the same blessing of read-only top-level commands that makes
/// `p-x-blessed-toplevel-conditional` live makes THIS shape legal oracle text — `unset -f` reads and
/// writes nothing on the host, so it is a natural candidate for the blessed set — and the moment it
/// is legal, a shipped check can carry a helper body the author explicitly removed.
#[test]
fn a_later_unset_f_removes_an_earlier_helper_declaration() {
    // Setup outside the closure: a panic in there would read as the target still failing.
    let entry = voucher_owning_dest("plain");
    let removal = format!("{MARKER}unset -f _dest\n");
    let control = HelperIndex::build(&[&entry], None)
        .closure_for(0, VOUCHER_BODY)
        .map(|closure| closure.sh());
    let after_removal = HelperIndex::build(&[&entry, &removal], None)
        .closure_for(0, VOUCHER_BODY)
        .map(|closure| closure.sh());
    assert_eq!(
        control.as_deref().map(str::trim),
        Ok("_dest() {\n   wombat cmp --plain -- \"$1\"\n}"),
        "control: with no removal the voucher's own declaration ships"
    );

    internal_tooling::xfail::xfail_until("p-x-helper-unset-f-across-files", || {
        assert!(
            !after_removal
                .as_deref()
                .is_ok_and(|closure| closure.contains("_dest() {")),
            "a declaration the loaded set later UNSET must not travel into a shipped check — got \
             {after_removal:?}"
        );
    });
}

/// `p-subshell-helper-death` (`30A` §2 P-green) — a helper the BOOK defines inside `( … )` suspends a
/// reaching vouch exactly as a top-level one does. Conservative, safe, and blunter than sh.
///
/// The sh fact: a definition made inside a subshell dies at the `)`, so a site after it binds whatever
/// was ambient. Measured under `posh ∩ dash` by `floor28-subshell-scoped-re-source` (ambient, then
/// regional, then ambient again) and by `floor30-subshell-nesting-and-removal-scope`. The regional
/// preference is not an accident of shell behaviour either — it is the whole of Dorc's answer to "I
/// want a different oracle for this region" (`28K` §1's re-source idiom).
///
/// WHAT THE ENGINE DOES, corrected against `30A`'s own parenthetical. The doctrine guessed a
/// subshell-scoped book helper "never enters the index at all". It does enter: `HelperIndex::build`
/// censuses the book's funcdefs at ANY depth, deliberately, because what the book defines decides
/// whether somebody else's vouch survives at apply. Being depth-blind, it cannot tell a regional
/// definition from an ambient one, so it suspends for BOTH — the same `BookRedefinesHelper` denial,
/// the same site running. That is the safe direction (a suspended vouch loses a license, never
/// licenses wrongly) and it is a real forfeit: the regional idiom the shells support costs the whole
/// book its guard tier. The target is `p-x-regional-helper`.
#[test]
fn a_book_subshell_helper_suspends_like_an_ambient_one() {
    let entry = voucher_owning_dest("plain");
    let regional = "( _dest() { printf 'regional\\n' ;}\n  wombat sync a )\nwombat sync b\n";
    let ambient = "_dest() { printf 'ambient\\n' ;}\nwombat sync b\n";

    let denied = |book: &str| {
        HelperIndex::build(&[&entry, book], Some(1))
            .closure_for(0, VOUCHER_BODY)
            .err()
            .map(|denial| denial.reason)
    };
    assert_eq!(
        denied(regional),
        Some(DenialReason::BookRedefinesHelper),
        "a definition sh confines to one region suspends the whole book's reaching vouch"
    );
    assert_eq!(
        denied(ambient),
        denied(regional),
        "and it is INDISTINGUISHABLE from the ambient redefinition — the census is depth-blind, \
         which is what `p-x-regional-helper` targets"
    );

    let unshadowed = HelperIndex::build(&[&entry, "wombat sync b\n"], Some(1))
        .closure_for(0, VOUCHER_BODY)
        .map(|closure| closure.sh());
    assert!(
        unshadowed
            .as_deref()
            .is_ok_and(|closure| closure.contains("_dest() {")),
        "control: with no book definition the vouch keeps its closure, so the suspensions above are \
         a real loss — {unshadowed:?}"
    );
}

/// `p-x-regional-helper` — THE TARGET: a book helper confined to a region it shares with NO site
/// reaching this vouch leaves the vouch alone; the ambient body still travels.
///
/// The sh fact: a definition made inside `( … )` dies at the `)`. Measured under `posh ∩ dash` by
/// `floor28-subshell-scoped-re-source` (ambient, then regional, then ambient again) and by
/// `floor30-subshell-nesting-and-removal-scope`. It is also the mechanism Dorc SELLS as its answer to
/// "I want a different oracle for this region" (`28K` §1's re-source idiom), so the engine reading it
/// as an ambient rebinding contradicts a documented feature.
///
/// WHAT THIS PIN CAN AND CANNOT SAY. The full target is per-site: in-region sites get the regional
/// body, post-region sites the ambient one. `closure_for` takes no site, so today's API cannot even
/// SPELL that half — the site-keyed closure question is the table-widening's own deliverable. What is
/// expressible, and what this asserts, is the unambiguous world: a region containing no site at all,
/// where every site is post-region and sh binds the ambient body everywhere. Getting that world right
/// is necessary for the per-site answer and is where the value being forfeited actually sits.
///
/// FAILS TODAY, measured 2026-08-16: the book census is depth-blind (see the pin above), so the
/// regional definition suspends the vouch exactly as an ambient one would, and the whole book loses
/// its guard tier for a definition no site can reach.
#[test]
fn a_regional_book_helper_leaves_an_unreachable_vouch_alone() {
    // Setup outside the closure: a panic in there would read as the target still failing.
    let entry = voucher_owning_dest("plain");
    // The region holds the definition and NOTHING else — so no site is in-region, and sh binds the
    // ambient `_dest` at every site in the book.
    let region_only = "( _dest() { printf 'regional\\n' ;} )\nwombat sync b\n";
    let regional = HelperIndex::build(&[&entry, region_only], Some(1))
        .closure_for(0, VOUCHER_BODY)
        .map(|closure| closure.sh());

    internal_tooling::xfail::xfail_until("p-x-regional-helper", || {
        assert!(
            regional
                .as_deref()
                .is_ok_and(|closure| closure.contains("_dest() {")),
            "a definition sh confines to a region containing no reaching site rebinds nothing, so \
             the ambient body must still travel — {regional:?}"
        );
    });
}

/// An oracle whose top level `.`-sources is legal text, and its OWN declarations contribute
/// alongside (`28Q:pin-oracle-side-sourcing-amendment`, built; promoted from the xfail pin
/// `p-x-blessed-toplevel-source`).
///
/// The sh fact: `.` applies a file's definitions into the CURRENT environment, so after the line the
/// sourced names are bound exactly as if they had been written there
/// (`floor30-sourcing-is-transitive` measures it under dash ∩ posh). That is why the construct is
/// worth admitting rather than merely tolerating — it is how sh spells composition, and
/// `28M` §7 `tune-explicit-composition-is-sanctioned` already sanctions explicitly-spelled
/// composition as the community-critical package shape.
///
/// Two conjuncts, and the second is the one that surprises. (1) The gate admits the file: before the
/// amendment a top-level `.` was a COMMAND, so `lint_load_inert` fired and the file made no dialect
/// claim. (2) The file's OWN helper still contributes — because the refusal is WHOLE-FILE, one
/// unadmitted line would cost the file every declaration it makes, and a role body declared beside
/// its helper would ship with an EMPTY closure and rc-127 at the host.
///
/// The admission is a CONTRACT, never an engine proof of inertness
/// (`30G:rul-inertness-is-contract-never-engine-fact`).
#[test]
fn an_oracle_that_sources_at_top_level_keeps_its_own_declarations() {
    let sourcing = format!(
        "{MARKER}. ./helpers.sh\n_dest() {{\n   wombat cmp -- \"$1\"\n}}\n{VOUCHER_BODY}\n"
    );
    let refused: Vec<&str> = dorc_oracle::load_inert::lint_load_inert(&sourcing)
        .iter()
        .map(|diag| diag.code.slug())
        .collect();
    let closure = HelperIndex::build(&[&sourcing], None)
        .closure_for(0, VOUCHER_BODY)
        .map(|closure| closure.sh());

    let without_the_source =
        format!("{MARKER}_dest() {{\n   wombat cmp -- \"$1\"\n}}\n{VOUCHER_BODY}\n");
    let control = HelperIndex::build(&[&without_the_source], None)
        .closure_for(0, VOUCHER_BODY)
        .map(|closure| closure.sh());
    assert!(
        control
            .as_deref()
            .is_ok_and(|closure| closure.contains("_dest() {")),
        "control: the same file WITHOUT the `.` line keeps its helper — {control:?}"
    );

    assert!(
        refused.is_empty(),
        "a top-level `.` is legal oracle text — got {refused:?}"
    );
    assert!(
        closure
            .as_deref()
            .is_ok_and(|closure| closure.contains("_dest() {")),
        "and the file's own declarations still contribute: a whole-file refusal would cost this \
         author every helper they wrote — {closure:?}"
    );
}
