# door1-and-form-runs (door-1 && POLE, charter 20V §4 / note 215): the `&&` companion-pole
set -e
dpkg -s nginx >/dev/null 2>&1 && { systemctl reload nginx; }
