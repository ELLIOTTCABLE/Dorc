# strawman24-crosskind-residue (plans/240 Stage 2 — the adequacy-seed / GREEN documenting case):
# footprints do NOT cross kinds unaided (23M cross-kind boundary), and this case DISCLOSES that
# residue rather than papering it.
#   site 0  apt-get install nginx — DIVERGED wall, footprint package:nginx (an at-most claim over
#           the PACKAGE kind only).
#   site 1  systemctl enable nginx — CONVERGED (service:nginx#enabled). Backing service:nginx is a
#           DIFFERENT KIND from the footprint's package:nginx ⇒ disjoint ⇒ it SURVIVES ⇒ ELIDES.
# THE RESIDUE (Stage-5 bridge territory, horizon-priced — NOT bridged here): nginx's postinst
# could enable nginx's service — a package→service CROSS-KIND effect. Entity-granular poisoning
# stays WITHIN the package kind, so it does NOT reach service:nginx, and the enable survives. This
# is real, disclosed, and priced at the professed horizon ("past here you trust authors' at-most
# claims"); the footprint-expansion bridge (touching package REACHES its service) is the Stage-5
# non-blunt fix. Behaviour here: the other-kind fact SURVIVES — that is the point.
apt-get install -y nginx
systemctl enable nginx
