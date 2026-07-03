# strawman24-floor-no-oracle (plans/240 Stage-1 yardstick — the honest FLOOR). A book with
# ZERO oracles supplied: every command is un-modeled ⇒ Opaque ⇒ unresolvable ⇒ nothing is
# probed ⇒ NOTHING elides. This is where the yardstick reads zero — the baseline every later
# stage must beat. "You get out what you put in": no oracle, no elision (DESIGN 'Sensitivities').
apt-get install -y nginx
systemctl enable nginx
ufw allow 80/tcp
