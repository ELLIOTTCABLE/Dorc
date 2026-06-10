#!/bin/sh
# ============================================================================
# LLM-GENERATED TESTING CORPUS — NOT REAL SECURITY CODE. This runbook was
# written by three AI authors with INTENTIONALLY VARIED quality and
# defensiveness, as a static-analysis test target (an at-least-partly
# artificial corpus: it cannot expose the truth of real-world ops-code, and
# must never be used to actually secure a server).
#
# FROZEN EVIDENCE — NEVER EXECUTE. Reconfigures SSH, firewall (default-deny
# both directions), PAM, fail2ban, unattended-upgrades with auto-reboot.
# Running any fragment of it can lock you out of the machine. Validation is
# `dash -n` only. See Research/corpora/H2SaLS/README.md.
#
# Rendition of: ELLIOTTCABLE/How-To-Secure-A-Linux-Server-With-Ansible
#   @ 34975f13406ec6541ee3c3a6499c0af1041e402d (ordering/conditionals ground
#   truth), rationale from imthenachoman/How-To-Secure-A-Linux-Server
#   README.md @ 5abb8c77 (section anchors in headers below).
# The original is TWO plays — a root-run "requirements" bootstrap, then the
# main play as the created user (per-task become). Run-as-root dissolves that
# split: section 1 is the bootstrap, sections 2-11 the main play's roles in
# role order.
# ============================================================================

set -eu

# ---- deployment configuration (placeholders are intentional; keep them) ----
USER_NAME='USERNAME_HERE'
USER_PW='PASSWORD_HERE'
# the public key TEXT itself (the play reads a controller-local file here)
SSH_PUBKEY='ssh-ed25519 AAAA_PUBKEY_HERE you@workstation'
SSH_PORT=55899
MAIL_TO='mailto@example.com'
MAIL_FROM='mailfrom@example.com'
MAIL_SMTP_SERVER='smtp.example.com'
MAIL_PW='PASSWORD_HERE'
MAIL_PORT=587

[ "$(id -u)" -eq 0 ] || { echo 'harden.sh: must run as root' >&2; exit 1; }

# every apt step runs unattended (Ansible's apt module exports this per-task)
export DEBIAN_FRONTEND=noninteractive

#=======================================================================
#== Section 1: requirements                               [author: A]
#==   Play: requirements-playbook.yml, roles/requirements/tasks/main.yml @ 34975f1
#==   Guide: "Pre/Post Installation Requirements" — README.md lines 250-265 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server/blob/master/README.md#prepost-installation-requirements
#=======================================================================

# apt update (cache only; the play does not upgrade here)
apt-get update

# install sudo
apt-get install -y sudo

# the three access-control groups; getent guards re-creation
for grp in sshusers suusers sudousers; do
    getent group "$grp" >/dev/null || groupadd "$grp"
done

# create the user with hashed password, supplementary groups, bash shell.
# openssl passwd -6 is the natural single-binary spelling of the play's
# sha512 password_hash; -m gives the home dir the user module creates by default.
if ! getent passwd "$USER_NAME" >/dev/null; then
    useradd -m -s /bin/bash \
        -G sshusers,sudousers,suusers \
        -p "$(openssl passwd -6 "$USER_PW")" \
        "$USER_NAME"
fi

# limit sudo to the sudousers group. NOTE: the play applies this edit to
# /etc/sudoers WITHOUT a visudo validate (unlike the passwordless edit below),
# so neither do we — see strain note s-req-4.
sudoers_line='%sudousers   ALL=(ALL:ALL) ALL'
if grep -q '^%sudousers' /etc/sudoers; then
    sed -i "s|^%sudousers.*|$sudoers_line|" /etc/sudoers
else
    printf '%s\n' "$sudoers_line" >> /etc/sudoers
fi

# limit who can use su to the suusers group. The play runs the statoverride
# unconditionally and tolerates only the "already exists" failure, so we
# mirror that: capture stderr, re-raise anything that is not the exists case.
su_err=$(dpkg-statoverride --update --add root suusers 4750 /bin/su 2>&1) || {
    case "$su_err" in
        *exist*) : ;;
        *) printf '%s\n' "$su_err" >&2; exit 1 ;;
    esac
}

# passwordless sudo for the new user. The play validates with `visudo -cf`
# and forces mode 0440, so we build the candidate file, validate it, and only
# then install it over /etc/sudoers.
nopasswd_line="$USER_NAME ALL=(ALL) NOPASSWD: ALL"
sudoers_tmp=$(mktemp)
cp /etc/sudoers "$sudoers_tmp"
if grep -q "^$USER_NAME" "$sudoers_tmp"; then
    sed -i "s|^$USER_NAME.*|$nopasswd_line|" "$sudoers_tmp"
else
    printf '%s\n' "$nopasswd_line" >> "$sudoers_tmp"
fi
visudo -cf "$sudoers_tmp"
install -m 0440 "$sudoers_tmp" /etc/sudoers
rm -f "$sudoers_tmp"

# install the new user's authorized key. The play reads a controller-local
# pubkey file; here the key text lives in SSH_PUBKEY. Resolve the home dir via
# getent (POSIX tilde does not expand "~$USER_NAME").
user_home=$(getent passwd "$USER_NAME" | cut -d: -f6)
install -d -m 0700 -o "$USER_NAME" -g "$USER_NAME" "$user_home/.ssh"
if ! grep -qF "$SSH_PUBKEY" "$user_home/.ssh/authorized_keys" 2>/dev/null; then
    printf '%s\n' "$SSH_PUBKEY" >> "$user_home/.ssh/authorized_keys"
fi
chmod 0600 "$user_home/.ssh/authorized_keys"
chown "$USER_NAME:$USER_NAME" "$user_home/.ssh/authorized_keys"


#=======================================================================
#== Section 2: packages                                   [author: A]
#==   Play: roles/packages/tasks/main.yml @ 34975f1
#==   Guide: (no direct section)
#=======================================================================

# update cache and upgrade everything (the play's update_cache + upgrade: yes;
# Ansible's upgrade:yes is the safe upgrade, not dist-upgrade)
apt-get update
apt-get -y upgrade

# the full package set, one install command as the play does
apt-get install -y \
    apt-listchanges \
    apt-transport-https \
    apticron \
    audispd-plugins \
    auditd \
    ca-certificates \
    clamav \
    clamav-daemon \
    clamav-freshclam \
    curl \
    fail2ban \
    host \
    libpam-pwquality \
    mailutils \
    msmtp \
    msmtp-mta \
    psad \
    rkhunter \
    ufw \
    unattended-upgrades


#=======================================================================
#== Section 3: ssh                                        [author: A]
#==   Play: roles/ssh/tasks/main.yml, roles/ssh/handlers/main.yml @ 34975f1
#==   Guide: "Secure /etc/ssh/sshd_config" — README.md lines 480-659 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server/blob/master/README.md#secure-etcsshsshd_config
#=======================================================================

sshd_changed=0

# replace-or-append an exact sshd_config line keyed on a regexp, the honest
# rendering of the play's lineinfile loop. Sets sshd_changed only on a real
# change (no-op when the exact line is already present), mirroring Ansible's
# changed -> notify. See strain note s-ssh-3.
set_sshd_line() {
    line_re=$1
    line_val=$2
    if grep -qxF "$line_val" /etc/ssh/sshd_config; then
        return 0
    fi
    if grep -q "$line_re" /etc/ssh/sshd_config; then
        sed -i "s|$line_re.*|$line_val|" /etc/ssh/sshd_config
    else
        printf '%s\n' "$line_val" >> /etc/ssh/sshd_config
    fi
    sshd_changed=1
}

# ufw - preemptively limit the default and new SSH ports. ufw is NOT yet
# enabled here (that happens in a later firewall section); the play adds these
# rules pre-emptively on purpose. See strain note s-ssh-1.
ufw limit in 22
ufw limit in "$SSH_PORT"

# the play's `meta: flush_handlers` here forces the ufw restart to run NOW,
# inline, rather than at section end. Deliberate, load-bearing ordering.
service ufw restart

# the secured sshd_config block (the play's blockinfile, Mozilla-modern
# content). Replace-or-insert between our own markers; rewrite only if the
# managed region actually differs, so the restart fires only on change.
# See strain note s-ssh-2.
block_begin='# >>> hardening: mozilla-modern sshd block >>>'
block_end='# <<< hardening: mozilla-modern sshd block <<<'
block_body=$(cat <<'EOF'
########################################################################################################
# start settings from https://infosec.mozilla.org/guidelines/openssh#modern-openssh-67 as of 2019-01-01
########################################################################################################
# Supported HostKey algorithms by order of preference.
HostKey /etc/ssh/ssh_host_ed25519_key
HostKey /etc/ssh/ssh_host_rsa_key
HostKey /etc/ssh/ssh_host_ecdsa_key
KexAlgorithms curve25519-sha256@libssh.org,ecdh-sha2-nistp521,ecdh-sha2-nistp384,ecdh-sha2-nistp256,diffie-hellman-group-exchange-sha256
Ciphers chacha20-poly1305@openssh.com,aes256-gcm@openssh.com,aes128-gcm@openssh.com,aes256-ctr,aes192-ctr,aes128-ctr
MACs hmac-sha2-512-etm@openssh.com,hmac-sha2-256-etm@openssh.com,hmac-sha2-512,hmac-sha2-256,umac-128@openssh.com
# LogLevel VERBOSE logs user's key fingerprint on login. Needed to have a clear audit track of which key was using to log in.
LogLevel VERBOSE
# Use kernel sandbox mechanisms where possible in unprivileged processes
# Systrace on OpenBSD, Seccomp on Linux, seatbelt on MacOSX/Darwin, rlimit elsewhere.
# Note: This setting is deprecated in OpenSSH 7.5 (https://www.openssh.com/txt/release-7.5)
# UsePrivilegeSeparation sandbox
########################################################################################################
# end settings from https://infosec.mozilla.org/guidelines/openssh#modern-openssh-67 as of 2019-01-01
########################################################################################################
# don't let users set environment variables
PermitUserEnvironment no
# only use the newer, more secure protocol
Protocol 2
# disable port forwarding
AllowTcpForwarding no
AllowStreamLocalForwarding no
GatewayPorts no
PermitTunnel no
# don't allow login if the account has an empty password
PermitEmptyPasswords no
# ignore .rhosts and .shosts
IgnoreRhosts yes
# verify hostname matches IP
UseDNS yes
Compression no
TCPKeepAlive no
AllowAgentForwarding no
# don't allow .rhosts or /etc/hosts.equiv
HostbasedAuthentication no
EOF
)

sshd_desired=$(mktemp)
sshd_current=$(mktemp)
# desired = current file with any old managed region stripped, fresh block appended
sed "/^$block_begin\$/,/^$block_end\$/d" /etc/ssh/sshd_config > "$sshd_desired"
{
    printf '%s\n' "$block_begin"
    printf '%s\n' "$block_body"
    printf '%s\n' "$block_end"
} >> "$sshd_desired"
# extract just the current managed region (if any) to compare against desired
sed -n "/^$block_begin\$/,/^$block_end\$/p" /etc/ssh/sshd_config > "$sshd_current"
if ! cmp -s "$sshd_current" - <<EOF
$block_begin
$block_body
$block_end
EOF
then
    cp "$sshd_desired" /etc/ssh/sshd_config
    sshd_changed=1
fi
rm -f "$sshd_desired" "$sshd_current"

# the play's 13-item lineinfile loop, in order
set_sshd_line '^AllowGroups' 'AllowGroups sshusers'
set_sshd_line '^ClientAliveCountMax' 'ClientAliveCountMax 0'
set_sshd_line '^ClientAliveInterval' 'ClientAliveInterval 300'
set_sshd_line '^ListenAddress' 'ListenAddress 0.0.0.0'
set_sshd_line '^LoginGraceTime' 'LoginGraceTime 30'
set_sshd_line '^MaxAuthTries' 'MaxAuthTries 5'
set_sshd_line '^MaxSessions' 'MaxSessions 2'
set_sshd_line '^MaxStartups' 'MaxStartups 2'
set_sshd_line '^PasswordAuthentication' 'PasswordAuthentication no'
set_sshd_line '^PermitRootLogin' 'PermitRootLogin no'
set_sshd_line '^X11Forwarding' 'X11Forwarding no'
set_sshd_line '^Subsystem' 'Subsystem sftp  internal-sftp -f AUTHPRIV -l INFO'

# Port: the play's `^Port (?!22$)` is a PCRE negative lookahead; POSIX has no
# lookahead, so the behavior is spelled as branches. Literal lineinfile
# behavior: an exact `Port 22` is left alone (new-machine safety, per the
# play's comment) but the new Port line is still APPENDED — the host listens
# on BOTH ports during setup (matching the pre-emptive ufw rules for both).
# Any other Port value is replaced. See strain note s-ssh-4.
if grep -qxF 'Port 22' /etc/ssh/sshd_config; then
    if ! grep -qxF "Port $SSH_PORT" /etc/ssh/sshd_config; then
        printf '%s\n' "Port $SSH_PORT" >> /etc/ssh/sshd_config
        sshd_changed=1
    fi
elif grep -qxF "Port $SSH_PORT" /etc/ssh/sshd_config; then
    : # already converged on the new port
elif grep -q '^Port ' /etc/ssh/sshd_config; then
    sed -i "s|^Port .*|Port $SSH_PORT|" /etc/ssh/sshd_config
    sshd_changed=1
else
    printf '%s\n' "Port $SSH_PORT" >> /etc/ssh/sshd_config
    sshd_changed=1
fi

# shorten the DH moduli (drop everything under 3071-bit). Already shell in the
# play; the `creates:` guard maps to the moduli.short existence check, and we
# preserve the pipeline. changed_when 'differ:' -> restart only on real change,
# so we set sshd_changed inside the cmp-differs branch. See strain note s-ssh-5.
if [ ! -f /etc/ssh/moduli.short ]; then
    cp /etc/ssh/moduli /etc/ssh/moduli.short
    awk '$5 >= 3071' /etc/ssh/moduli | tee /etc/ssh/moduli.tmp
    if ! cmp /etc/ssh/moduli /etc/ssh/moduli.tmp; then
        mv /etc/ssh/moduli.tmp /etc/ssh/moduli
        sshd_changed=1
    fi
fi

# sshd restart: deferred to the end-of-play handlers block at the bottom of
# this script (Ansible runs notified handlers at the end of the PLAY, not at
# role end; a mid-run failure means no restarts at all).


#=======================================================================
#== Section 4: password-quality                           [author: A]
#==   Play: roles/password-quality/tasks/main.yml @ 34975f1
#==   Guide: "Force Accounts To Use Secure Passwords" — README.md lines 1234-1298 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server/blob/master/README.md#force-accounts-to-use-secure-passwords
#=======================================================================

# enforce strong passwords via pam_pwquality. The play's regexp matches any
# existing pam_pwquality.so line and replaces it. NOTE: the upstream `line`
# ends with the typo `gecoschec` (should be `gecoscheck`); preserved verbatim.
# See strain note s-pam-1.
pam_line='password        requisite                       pam_pwquality.so retry=3 minlen=10 difok=3 ucredit=-1 lcredit=-1 dcredit=-1 ocredit=-1 maxrepeat=3 gecoschec'
if grep -q '^.*pam_pwquality.so.*$' /etc/pam.d/common-password; then
    sed -i "s|^.*pam_pwquality.so.*\$|$pam_line|" /etc/pam.d/common-password
else
    printf '%s\n' "$pam_line" >> /etc/pam.d/common-password
fi

#=======================================================================
#== Section 5: unattended-upgrades                        [author: B]
#==   Play: roles/unattended-upgrades/tasks/main.yml @ 34975f1
#==   Guide: "Automatic Security Updates and Alerts" — README.md lines 1299-1439 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server#automatic-security-updates-and-alerts
#=======================================================================

# new file, package ships 50unattended-upgrades; ours wins by being later
cat > /etc/apt/apt.conf.d/51myunattended-upgrades <<'EOF'
// Enable the update/upgrade script (0=disable)
APT::Periodic::Enable "1";

// Do "apt-get update" automatically every n-days (0=disable)
APT::Periodic::Update-Package-Lists "1";

// Do "apt-get upgrade --download-only" every n-days (0=disable)
APT::Periodic::Download-Upgradeable-Packages "1";

// Do "apt-get autoclean" every n-days (0=disable)
APT::Periodic::AutocleanInterval "7";

// Send report mail to root
//     0:  no report             (or null string)
//     1:  progress report       (actually any string)
//     2:  + command outputs     (remove -qq, remove 2>/dev/null, add -d)
//     3:  + trace on    APT::Periodic::Verbose "2";
APT::Periodic::Unattended-Upgrade "1";

// Automatically upgrade packages from these
Unattended-Upgrade::Origins-Pattern {
      // "o=Debian,a=stable";
      //"o=Debian,a=stable-updates";
      "origin=Debian,codename=${distro_codename},label=Debian-Security";
};

// You can specify your own packages to NOT automatically upgrade here
Unattended-Upgrade::Package-Blacklist {
};

// Run dpkg --force-confold --configure -a if a unclean dpkg state is detected to true to ensure that updates get installed even when the system got interrupted during a previous run
Unattended-Upgrade::AutoFixInterruptedDpkg "true";

//Perform the upgrade when the machine is running because we wont be shutting our server down often
Unattended-Upgrade::InstallOnShutdown "false";

// Send an email to this address with information about the packages upgraded.
Unattended-Upgrade::Mail "root";

// Always send an e-mail
Unattended-Upgrade::MailOnlyOnError "false";

// Remove all unused dependencies after the upgrade has finished
Unattended-Upgrade::Remove-Unused-Dependencies "true";

// Remove any new unused dependencies after the upgrade has finished
Unattended-Upgrade::Remove-New-Unused-Dependencies "true";

// Automatically reboot WITHOUT CONFIRMATION if the file /var/run/reboot-required is found after the upgrade.
Unattended-Upgrade::Automatic-Reboot "true";

// Automatically reboot even if users are logged in.
Unattended-Upgrade::Automatic-Reboot-WithUsers "true";
EOF

#=======================================================================
#== Section 6: firewall                                   [author: B]
#==   Play: roles/firewall/tasks/main.yml, roles/firewall/handlers/main.yml @ 34975f1
#==   Guide: "Firewall With UFW (Uncomplicated Firewall)" — README.md lines 1593-1877 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server#firewall-with-ufw-uncomplicated-firewall
#==   (also PSAD lines 1878-2037, Fail2Ban lines 2038-2181)
#=======================================================================

ufw_changed=0
psad_changed=0
fail2ban_changed=0

# enable + logging; --force so the enable doesn't prompt (no tty here)
ufw logging on
ufw --force enable
ufw_changed=1

ufw default deny incoming
ufw default deny outgoing
ufw_changed=1

ufw limit in "$SSH_PORT"
ufw_changed=1

for p in 43 53 123 80 443 "$MAIL_PORT"; do
  ufw allow out "$p"
done
ufw_changed=1

# psad.conf already exists from the package; replace the keys we care about,
# append if somehow missing. one line carries the box's own hostname.
set_conf() {
  key=$1
  line=$2
  file=$3
  if grep -Eq "$key" "$file"; then
    esc=$(printf '%s' "$line" | sed 's/[&/\]/\\&/g')
    sed -i "s/$key.*/$esc/" "$file"
  else
    printf '%s\n' "$line" >> "$file"
  fi
}

set_conf '^EMAIL_ADDRESSES'      "EMAIL_ADDRESSES $MAIL_TO;"          /etc/psad/psad.conf
set_conf '^EXPECT_TCP_OPTIONS'   'EXPECT_TCP_OPTIONS Y;'             /etc/psad/psad.conf
set_conf '^ENABLE_AUTO_IDS '     'ENABLE_AUTO_IDS Y;'                /etc/psad/psad.conf
set_conf '^ENABLE_AUTO_IDS_EMAILS' 'ENABLE_AUTO_IDS_EMAILS Y;'      /etc/psad/psad.conf
set_conf '^AUTO_IDS_DANGER_LEVEL' 'AUTO_IDS_DANGER_LEVEL 3;'        /etc/psad/psad.conf
set_conf '^HOSTNAME'             "HOSTNAME $(hostname -s);"          /etc/psad/psad.conf
psad_changed=1

# log everything just before COMMIT so psad can read it; do both v4 and v6.
# guard the insert so a re-run doesn't stack duplicate LOG rules.
add_psad_logging() {
  file=$1
  if ! grep -q 'ANSIBLE MANAGED BLOCK' "$file"; then
    tmp=$(mktemp)
    awk '
      /^COMMIT/ && !done {
        print "# BEGIN ANSIBLE MANAGED BLOCK"
        print "# log all traffic so psad can analyze"
        print "-A INPUT -j LOG --log-tcp-options --log-prefix \"[IPTABLES] \""
        print "-A FORWARD -j LOG --log-tcp-options --log-prefix \"[IPTABLES] \""
        print "# END ANSIBLE MANAGED BLOCK"
        done = 1
      }
      { print }
    ' "$file" > "$tmp"
    cat "$tmp" > "$file"
    rm -f "$tmp"
  fi
}

add_psad_logging /etc/ufw/before.rules
add_psad_logging /etc/ufw/before6.rules
ufw_changed=1

psad --sig-update

cat > /etc/fail2ban/jail.local <<EOF
[DEFAULT]
# the IP address range we want to ignore
ignoreip = 127.0.0.1/8

# who to send e-mail to
destemail = $MAIL_TO

# who is the email from
sender = $MAIL_FROM

# since we're using exim4 to send emails
mta = mail

# get email alerts
action = %(action_mwl)s
EOF
fail2ban_changed=1

cat > /etc/fail2ban/jail.d/ssh.local <<EOF
[sshd]
enabled = true
banaction = ufw
port = $SSH_PORT
filter = sshd
logpath = %(sshd_log)s
maxretry = 5
EOF
fail2ban_changed=1

# restarts: deferred to the end-of-play handlers block at the bottom of this
# script (Ansible handlers run at the end of the PLAY; a mid-run failure
# means none of them run).

#=======================================================================
#== Section 7: mail                                       [author: B]
#==   Play: roles/mail/tasks/main.yml @ 34975f1
#==   Guide: "MSMTP (Simple Sendmail)" — README.md lines 3516-3590 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server#msmtp-alternative
#=======================================================================

cat > /etc/msmtprc <<EOF
defaults
port $MAIL_PORT
tls on
tls_trust_file /etc/ssl/certs/ca-certificates.crt
account $MAIL_FROM
host $MAIL_SMTP_SERVER
set_from_header on
from $MAIL_FROM
auth on
user $MAIL_FROM
password $MAIL_PW
account default: $MAIL_FROM
aliases /etc/aliases
logfile /var/log/msmtp
EOF

chgrp msmtp /etc/msmtprc
chmod 640 /etc/msmtprc

set_conf '^root:'    "root: $MAIL_TO"    /etc/aliases
set_conf '^default:' "default: $MAIL_TO" /etc/aliases

set_conf '^set sendmail' 'set sendmail="/usr/bin/msmtp -t"' /etc/mail.rc

echo "Testmail content" | mail -s "Testmail subject" "$MAIL_TO"

#=======================================================================
#== Section 8: clamav                                     [author: C]
#==   Play: roles/clamav/tasks/main.yml @ 34975f1
#==   Guide: "Anti-Virus Scanning With ClamAV (WIP)" — README.md lines 2584-2698 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server#anti-virus-scanning-with-clamav-wip
#=======================================================================

# The play uses cronvar (sets MAILTO in root's crontab) + cron (a named daily
# job). Neither maps to a plain primitive; a drop-in cron.d file is the closest
# faithful admin spelling. MAILTO here scopes to jobs in this file only.
cat > /etc/cron.d/clamav-daily <<EOF
MAILTO=$MAIL_TO
0 3 * * * root /usr/bin/clamscan -ri --exclude-dir="^/sys" --no-summary /
EOF
chmod 0644 /etc/cron.d/clamav-daily

#=======================================================================
#== Section 9: rkhunter                                   [author: C]
#==   Play: roles/rkhunter/tasks/main.yml @ 34975f1
#==   Guide: "Rootkit Detection With Rkhunter (WIP)" — README.md lines 2699-2784 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server#rootkit-detection-with-rkhunter-wip
#=======================================================================

# Unguarded in the play (no creates:): this overwrites .local every run,
# discarding prior edits. The loops below re-apply them, so the net is stable.
cp -p /etc/rkhunter.conf /etc/rkhunter.conf.local

# lineinfile semantics: replace the matching line, else append. Driven from a
# heredoc of "regexp<TAB>replacement" pairs to avoid unrolled repetition.
while read -r pattern line; do
    [ -z "$pattern" ] && continue
    if grep -Eq "$pattern" /etc/rkhunter.conf.local; then
        sed -i -E "s|$pattern.*|$line|" /etc/rkhunter.conf.local
    else
        printf '%s\n' "$line" >> /etc/rkhunter.conf.local
    fi
done <<EOF
^UPDATE_MIRRORS	UPDATE_MIRRORS=1
^MIRRORS_MODE	MIRRORS_MODE=0
^MAIL-ON-WARNING	MAIL-ON-WARNING=$MAIL_TO
^COPY_LOG_ON_ERROR	COPY_LOG_ON_ERROR=1
^PHALANX2_DIRTEST	PHALANX2_DIRTEST=1
^WEB_CMD	WEB_CMD=""
^USE_LOCKING	USE_LOCKING=1
^SHOW_SUMMARY_WARNINGS_NUMBER	SHOW_SUMMARY_WARNINGS_NUMBER=1
EOF

while read -r pattern line; do
    [ -z "$pattern" ] && continue
    if grep -Eq "$pattern" /etc/default/rkhunter; then
        sed -i -E "s|$pattern.*|$line|" /etc/default/rkhunter
    else
        printf '%s\n' "$line" >> /etc/default/rkhunter
    fi
done <<EOF
^CRON_DAILY_RUN	CRON_DAILY_RUN="true"
^CRON_DB_UPDATE	CRON_DB_UPDATE="true"
^DB_UPDATE_EMAIL	DB_UPDATE_EMAIL="false"
^REPORT_EMAIL	REPORT_EMAIL="root"
^APT_AUTOGEN	APT_AUTOGEN="true"
^NICE	NICE="0"
^RUN_CHECK_ON_BATTERY	RUN_CHECK_ON_BATTERY="false"
EOF

# Both network/state-touching, both unguarded; the play runs them in ONE
# shell task, so only the LAST command's rc decides it — an --update failure
# (rc 1 on download error, rc 2 after installing updates) never fails the
# play. The || true renders that tolerance under set -e.
rkhunter --update || true
rkhunter --propupd

#=======================================================================
#== Section 10: auditd                                    [author: C]
#==   Play: roles/auditd/tasks/main.yml @ 34975f1
#==   Guide: (play-only addition — no matching guide section)
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server
#=======================================================================

# The play runs these three in ONE shell task: only the restart's rc decides
# it — a failed rm (file absent after an earlier bad download) or a failed
# wget is tolerated, and auditd restarts regardless, possibly with an empty
# rules dir (upstream wart preserved). || true renders that per-line
# tolerance under set -e.
rm /etc/audit/rules.d/audit.rules || true
wget -P /etc/audit/rules.d/ https://raw.githubusercontent.com/Neo23x0/auditd/master/audit.rules || true
service auditd restart

#=======================================================================
#== Section 11: lynis                                     [author: C]
#==   Play: roles/lynis/tasks/main.yml @ 34975f1
#==   Guide: "Lynis - Linux Security Auditing" — README.md lines 3006-3059 @ 5abb8c7
#==   https://github.com/imthenachoman/How-To-Secure-A-Linux-Server#lynis---linux-security-auditing
#=======================================================================

apt-get update
apt-get upgrade -y

apt-get install -y gpg

get_url() {
    wget -O "$2" "$1"
    chmod "$3" "$2"
}
get_url https://packages.cisofy.com/keys/cisofy-software-public.key \
    /usr/share/keyrings/cisofy-archive-keyring.asc 0644

# creates:-guarded in the play — only dearmor if the .gpg is not already present.
if [ ! -e /usr/share/keyrings/cisofy-archive-keyring.gpg ]; then
    gpg --dearmor --batch --yes -o /usr/share/keyrings/cisofy-archive-keyring.gpg \
        /usr/share/keyrings/cisofy-archive-keyring.asc
fi

# apt_repository also refreshes the cache, so the write is followed by an update.
cat > /etc/apt/sources.list.d/cisofy-lynis.list <<EOF
deb [signed-by=/usr/share/keyrings/cisofy-archive-keyring.gpg] https://packages.cisofy.com/community/lynis/deb stable main
EOF
apt-get update

# The play really does repeat the update+upgrade here; preserved.
apt-get update
apt-get upgrade -y

apt-get install -y lynis

# ansi2html is piped here but no task ever installs it — preserved upstream
# wart. In the play all three lines are ONE shell task whose rc is the mail's:
# the dead pipeline still creates an empty report via its redirect and the
# mail goes out anyway. || true renders that tolerance under set -e.
lynis update info || true
lynis audit system | ansi2html -l > /tmp/lynis-report.html || true
echo "First Lynis report see attachment" | mail -A /tmp/lynis-report.html -s "Lynis report" "$MAIL_TO"


#=======================================================================
#== Handlers (end of play)                                [assembler]
#==   Ansible runs notified handlers ONCE at the end of the play, in
#==   handler-definition order (ssh role's, then firewall's three) —
#==   not at role/section end. A mid-run abort means no restarts run,
#==   matching the play without force_handlers. The one inline
#==   exception is Section 3's flush_handlers ufw restart.
#=======================================================================

if [ "$sshd_changed" -eq 1 ]; then
    service ssh restart
fi
if [ "$ufw_changed" -eq 1 ]; then
    service ufw restart
fi
if [ "$psad_changed" -eq 1 ]; then
    service psad restart
fi
if [ "$fail2ban_changed" -eq 1 ]; then
    service fail2ban restart
fi
