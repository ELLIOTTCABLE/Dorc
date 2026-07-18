# apt provider for the `package` kind — declares the kind's read-only probe.
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge kill installed
