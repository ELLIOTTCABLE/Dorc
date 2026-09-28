module claims
open species
open words

one sig tessa__a_slash_separated_path_names_the_inode_of_its_last_entry extends Yields {} { speaker = tessa  of = slash_path  under = Path  to = Inode }
one sig tessa__a_bare_word_is_not_a_path extends Yields {} { speaker = tessa  of = bare_word  under = Path  no to }
one sig tessa__an_inode_is_the_primary_key_of_a_file extends PrimaryOf {} { speaker = tessa  ofScheme = Inode  ofSort = File }
one sig tessa__a_file_is_identified_in_its_filesystem extends IdentifiedIn {} { speaker = tessa  ofScheme = Inode  within = Filesystem }
one sig tessa__equal_inodes_in_one_filesystem_reach_one_file extends GuaranteesUniqueReferent {} { speaker = tessa  on = Inode }
one sig tessa__a_file_has_one_inode_in_its_filesystem extends GuaranteesUniqueName {} { speaker = tessa  on = Inode }

one sig simon__a_device_number_is_the_primary_key_of_a_filesystem extends PrimaryOf {} { speaker = simon  ofScheme = DeviceNumber  ofSort = Filesystem }
one sig simon__a_filesystem_is_identified_in_the_boot_that_mounted_it extends IdentifiedIn {} { speaker = simon  ofScheme = DeviceNumber  within = Boot }
one sig simon__a_filesystem_has_one_device_number_in_its_boot extends GuaranteesUniqueName {} { speaker = simon  on = DeviceNumber }
one sig simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem extends AliasesNothingElse {} { speaker = simon  store = Filesystem }

one sig stdlib__a_boot_id_is_the_primary_key_of_a_boot extends PrimaryOf {} { speaker = stdlib  ofScheme = BootId  ofSort = Boot }
one sig stdlib__a_boot_id_is_a_root extends Root {} { speaker = stdlib  on = BootId }

one sig carl__chmod_reads_its_operand_as_a_path extends Operand {} { speaker = carl  verb = chmod  at = 1  under = Path }
one sig carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else extends MayWrite {} { speaker = carl  verb = chmod }
one sig carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode extends ChecksRead {} { speaker = carl  verb = chmod }

one sig foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle extends PrimaryOf {} { speaker = foob  ofScheme = BundlePath  ofSort = CertificateBundle }

one sig tessa__a_singly_linked_path_names_one_file extends GuaranteesUniqueName {} { speaker = tessa  on = Path }

-- the load files, one set each
fun tessa_fs: set MDecl {
   tessa__a_slash_separated_path_names_the_inode_of_its_last_entry + tessa__a_bare_word_is_not_a_path
   + tessa__an_inode_is_the_primary_key_of_a_file + tessa__a_file_is_identified_in_its_filesystem
   + tessa__equal_inodes_in_one_filesystem_reach_one_file + tessa__a_file_has_one_inode_in_its_filesystem
}
fun simon_fs: set MDecl {
   simon__a_device_number_is_the_primary_key_of_a_filesystem + simon__a_filesystem_is_identified_in_the_boot_that_mounted_it
   + simon__a_filesystem_has_one_device_number_in_its_boot + simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem
}
fun stdlib_boot: set MDecl { stdlib__a_boot_id_is_the_primary_key_of_a_boot + stdlib__a_boot_id_is_a_root }
fun carl_chmod: set MDecl {
   carl__chmod_reads_its_operand_as_a_path + carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else
   + carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode
}
fun foob_certs: set MDecl { foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle }
fun tessa_singly_linked: set MDecl { tessa_fs + tessa__a_singly_linked_path_names_one_file }
