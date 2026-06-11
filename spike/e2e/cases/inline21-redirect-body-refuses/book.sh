# inline21-redirect-body-refuses (arch-2, tc-M2 — the write-redirect pole): a wrapper whose
# body carries a WRITE-shaped redirect (`>> /etc/motd`) to a real file. Redirect-effects are
# unmodeled (y-1), and inlining alone would EXPOSE this invisible body file-write as
# wrong-ambience — so the inline REFUSES with a loud diagnostic naming the unmodeled
# write-redirect. The call `note` stays an ordinary unmodeled command (Opaque) ⇒ it runs
# (site:0 skip-unresolvable), poisoning the `apt-get install -y curl` below (site:1
# skip-unresolvable, runs). `>/dev/null`/`2>&1`/fd-dups stay EXEMPT (the devnull-exemption
# keeps the wrapper-pun population alive); only a real-file write fences. Analysis-only: a body
# writing /etc/motd must never execute, and the call runs Opaque anyway.
note() { apt-get install -y nginx; echo done >> /etc/motd; }
note
apt-get install -y curl
