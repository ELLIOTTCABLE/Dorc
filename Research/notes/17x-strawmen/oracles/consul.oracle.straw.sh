#!/bin/sh
# Oracle EXTRACTED from books/consul-provision.book.sh (scottslowe/learning-tools @7fc9e09). [STRAWMAN]
# The CLEAN end of the spectrum: every kind it touches, it already probes. Lifted via grep, body unread.
# Kinds: user:consul · file:/usr/local/bin/consul · dir:/var/consul · dir:/etc/consul.d · service:consul
#
# (A) LIFTED — all faithful:
consul_user() { [ -n "$(getent passwd consul)" ]; }            # book L31: if [ -z "$(getent passwd consul)" ]
consul_bin()  { [ -e /usr/local/bin/consul ]; }                # book ~L50: if [[ ! -e /usr/local/bin/consul ]]
consul_data() { [ -d /var/consul ]; }                          # book L39
consul_conf() { [ -d /etc/consul.d ]; }                        # book L45
consul_svc_enabled() { systemctl is-enabled --quiet consul; }  # book L60
consul_svc_active()  { systemctl is-active  --quiet consul; }  # book L66
#
# Kind tags: user:consul=getent passwd · service:consul=systemctl is-enabled/is-active.
# Nothing SUPPLIED — this book is the demonstration that a careful author already writes the whole oracle.
