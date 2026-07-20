#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# firewall oracle (ufw). `ufw allow <rule>` establishes firewall:<rule>#allowed.
ufw__predict() {
   verb=$1; shift
   rule : sm.dorc.Firewall = "$1"
   case $verb in
      allow) ufw status "$rule" >/dev/null 2>&1 : sm.dorc.Firewall:"$rule"@allowed ;;
      deny) ufw status "$rule" >/dev/null 2>&1 :! sm.dorc.Firewall:"$rule"@allowed ;;
   esac
}
