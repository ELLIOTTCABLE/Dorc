-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module claims
open species
open words

one sig stdlib__apt_get_is_described extends Vouches {} { speaker = stdlib  verb = apt_get }
one sig stdlib__apt_get_update_disturbs_only_the_package_index extends Disturbs {} { speaker = stdlib  verb = apt_get  sub = update  no at  cells = pkg_index }
one sig stdlib__apt_get_update_is_converged_when_the_package_index_is_fresh extends Reads {} { speaker = stdlib  verb = apt_get  sub = update  no at  cells = pkg_index }
one sig stdlib__apt_get_install_is_converged_when_the_package_database_lists_it extends Reads {} { speaker = stdlib  verb = apt_get  sub = install  no at  cells = dpkg_db }

one sig tessa__cp_is_described extends Vouches {} { speaker = tessa  verb = cp }
one sig tessa__cp_disturbs_only_its_destination extends Disturbs {} { speaker = tessa  verb = cp  no sub  at = 1  no cells }
one sig tessa__cp_is_converged_when_its_destination_has_the_source_content extends Reads {} { speaker = tessa  verb = cp  no sub  at = 0 + 1  no cells }

one sig debsvc__systemctl_is_described extends Vouches {} { speaker = debsvc  verb = systemctl }
one sig debsvc__systemctl_enable_disturbs_only_the_unit_state extends Disturbs {} { speaker = debsvc  verb = systemctl  sub = enable  no at  cells = unit_state }
one sig debsvc__systemctl_enable_is_converged_when_the_unit_is_enabled_and_active extends Reads {} { speaker = debsvc  verb = systemctl  sub = enable  no at  cells = unit_state }
one sig debsvc__ufw_is_described extends Vouches {} { speaker = debsvc  verb = ufw }
one sig debsvc__ufw_allow_disturbs_only_the_rule_table extends Disturbs {} { speaker = debsvc  verb = ufw  sub = allow  no at  cells = ufw_rules }
one sig debsvc__ufw_allow_is_converged_when_the_rule_is_present extends Reads {} { speaker = debsvc  verb = ufw  sub = allow  no at  cells = ufw_rules }
one sig debsvc__ufw_withholds_elision extends Withholds {} { speaker = debsvc  verb = ufw }

one sig foob__foobar_is_described extends Vouches {} { speaker = foob  verb = foobar }
one sig foob__foobar_sync_certs_disturbs_only_the_certs extends Disturbs {} { speaker = foob  verb = foobar  sub = sync_certs  at = 1  no cells }
one sig foob__foobar_sync_certs_is_converged_when_the_certs_are_current extends Reads {} { speaker = foob  verb = foobar  sub = sync_certs  at = 1  no cells }

-- the load files, one set each
fun stdlib_apt: set MDecl {
   stdlib__apt_get_is_described + stdlib__apt_get_update_disturbs_only_the_package_index
   + stdlib__apt_get_update_is_converged_when_the_package_index_is_fresh
   + stdlib__apt_get_install_is_converged_when_the_package_database_lists_it
}
fun tessa_fs: set MDecl {
   tessa__cp_is_described + tessa__cp_disturbs_only_its_destination
   + tessa__cp_is_converged_when_its_destination_has_the_source_content
}
fun debsvc_services: set MDecl {
   debsvc__systemctl_is_described + debsvc__systemctl_enable_disturbs_only_the_unit_state
   + debsvc__systemctl_enable_is_converged_when_the_unit_is_enabled_and_active
   + debsvc__ufw_is_described + debsvc__ufw_allow_disturbs_only_the_rule_table
   + debsvc__ufw_allow_is_converged_when_the_rule_is_present
}
fun debsvc_ufw_guard_only: set MDecl { debsvc__ufw_withholds_elision }
fun foob_certs: set MDecl {
   foob__foobar_is_described + foob__foobar_sync_certs_disturbs_only_the_certs
   + foob__foobar_sync_certs_is_converged_when_the_certs_are_current
}
