//! The production driver: the system `ssh` binary as a child process.
//!
//! `kAGENTLESS`/`executorless` is welded (`142:Resolution`): there is no bootstrapped remote
//! agent and no in-process ssh library. The user's own ssh config is the credential plane —
//! their aliases, `ProxyJump`, keys and agent all apply, and Dorc holds no keys and adds no
//! credential of its own (`law-security-floor`).

use crate::{HostId, SessionDriver, SessionOutcome, SessionRequest};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

/// The connect-phase ceiling. Distinct from the session ceiling: this bounds getting a channel,
/// not doing the work.
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(15);

/// How the invocation is assembled (`260` §5, `dec-26-ssh-config`).
#[derive(Debug, Clone)]
pub struct SshOptions {
    /// Ceiling on establishing the connection.
    pub connect_timeout: Duration,
    /// Accept an unknown host key on first contact (`StrictHostKeyChecking=accept-new`).
    ///
    /// DEFAULT OFF, and it must stay an explicit per-invocation opt-in. With it off, OpenSSH's
    /// own `known_hosts` enforcement applies and `BatchMode` turns an unknown host into a clean
    /// loud refusal rather than a prompt — which is the correct outcome. Blind acceptance
    /// (`UserKnownHostsFile=/dev/null`) is never product behaviour at any flag setting.
    pub accept_new_host_key: bool,
    /// Ignore the user's ssh config entirely and read only this file. An opt-in escape for
    /// hermetic runs; the default composes with the user's config instead of bypassing it.
    pub config_file: Option<PathBuf>,
    /// The remote interpreter that reads the artifact from stdin.
    pub remote_sh: String,
}

impl Default for SshOptions {
    fn default() -> Self {
        Self {
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            accept_new_host_key: false,
            config_file: None,
            remote_sh: "sh".to_owned(),
        }
    }
}

impl SshOptions {
    /// The `-o`/`-F`/`-T` arguments, in a fixed order, before the destination.
    ///
    /// These are passed as options rather than written into a generated config file so they
    /// LAYER over whatever the user already has: their `Host` stanzas, jump hosts and identity
    /// files keep working, and Dorc contributes only its non-negotiables.
    fn args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();
        if let Some(path) = &self.config_file {
            args.push("-F".into());
            args.push(path.clone().into_os_string());
        }
        args.push("-T".into());
        for option in [
            "BatchMode=yes".to_owned(),
            format!("ConnectTimeout={}", self.connect_timeout.as_secs()),
            "ServerAliveInterval=15".to_owned(),
            "ServerAliveCountMax=4".to_owned(),
            "ClearAllForwardings=yes".to_owned(),
            "ForwardAgent=no".to_owned(),
            "LogLevel=ERROR".to_owned(),
            "IgnoreUnknown=UseKeychain".to_owned(),
        ] {
            args.push("-o".into());
            args.push(option.into());
        }
        if self.accept_new_host_key {
            args.push("-o".into());
            args.push("StrictHostKeyChecking=accept-new".into());
        }
        args
    }
}

/// Reaches a real host over the system `ssh` binary.
#[derive(Debug, Clone, Default)]
pub struct SshDriver {
    options: SshOptions,
}

impl SshDriver {
    /// A driver with the given invocation posture.
    #[must_use]
    pub fn new(options: SshOptions) -> Self {
        Self { options }
    }

    /// The argv this driver would run, destination and remote command included.
    ///
    /// Exposed so the posture can be asserted without opening a connection — the option set is
    /// a security surface, and a test that has to reach a host to check it would never run.
    #[must_use]
    pub fn argv(&self, host: &HostId, marker: &crate::SessionMarker) -> Vec<OsString> {
        let mut argv = self.options.args();
        if let Some(port) = host.port() {
            argv.push("-p".into());
            argv.push(port.to_string().into());
        }
        argv.push(host.as_str().into());
        argv.push(marker.remote_command(&self.options.remote_sh).into());
        argv
    }
}

impl SessionDriver for SshDriver {
    fn run(&mut self, request: &SessionRequest<'_>) -> SessionOutcome {
        let mut command = Command::new("ssh");
        command.args(self.argv(request.host, request.marker));
        crate::child::run(command, request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionMarker;

    fn argv_strings(driver: &SshDriver, host: &str) -> Vec<String> {
        let host = HostId::new(host).expect("valid destination");
        let marker = SessionMarker::new("n1", 1).expect("valid nonce");
        driver
            .argv(&host, &marker)
            .into_iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn the_default_posture_carries_every_non_negotiable() {
        let argv = argv_strings(&SshDriver::default(), "web1");
        for required in [
            "BatchMode=yes",
            "ConnectTimeout=15",
            "ServerAliveInterval=15",
            "ServerAliveCountMax=4",
            "ClearAllForwardings=yes",
            "ForwardAgent=no",
            "LogLevel=ERROR",
        ] {
            assert!(argv.iter().any(|a| a == required), "missing {required}");
        }
        assert!(
            argv.iter().any(|a| a == "-T"),
            "a pty cooks and merges the streams the records lane rides on"
        );
    }

    #[test]
    fn host_key_checking_is_never_weakened_without_the_explicit_opt_in() {
        let argv = argv_strings(&SshDriver::default(), "web1");
        assert!(
            !argv.iter().any(|a| a.contains("StrictHostKeyChecking")),
            "the default must defer to OpenSSH's own enforcement, saying nothing about it"
        );
        assert!(
            !argv.iter().any(|a| a.contains("UserKnownHostsFile")),
            "blind acceptance is never product behaviour at any flag setting"
        );

        let opted_in = SshDriver::new(SshOptions {
            accept_new_host_key: true,
            ..SshOptions::default()
        });
        assert!(
            argv_strings(&opted_in, "web1")
                .iter()
                .any(|a| a == "StrictHostKeyChecking=accept-new")
        );
    }

    #[test]
    fn the_user_config_is_composed_with_by_default_and_bypassed_only_on_request() {
        assert!(
            !argv_strings(&SshDriver::default(), "web1")
                .iter()
                .any(|a| a == "-F"),
            "the default must not bypass the user's aliases, ProxyJump or keys"
        );
        let hermetic = SshDriver::new(SshOptions {
            config_file: Some(PathBuf::from("/tmp/dorc-ssh-config")),
            ..SshOptions::default()
        });
        assert!(argv_strings(&hermetic, "web1").iter().any(|a| a == "-F"));
    }

    #[test]
    fn the_destination_is_the_last_argument_before_the_remote_command() {
        let argv = argv_strings(&SshDriver::default(), "deploy@web1.example.net");
        let host_at = argv
            .iter()
            .position(|a| a == "deploy@web1.example.net")
            .expect("destination present");
        assert_eq!(
            host_at,
            argv.len().saturating_sub(2),
            "options precede the destination; the remote command is the sole trailing argument"
        );
        assert!(
            argv.last().is_some_and(|last| last.contains("sh -s")),
            "the artifact is read from stdin so its own bytes are never touched"
        );
    }

    #[test]
    fn a_destination_splits_into_user_host_and_port_the_way_ssh_will_read_it() {
        for (raw, destination, port) in [
            ("web1", "web1", None),
            ("root@web1", "root@web1", None),
            ("localhost:2222", "localhost", Some(2222)),
            ("root@localhost:2222", "root@localhost", Some(2222)),
            ("192.0.2.7:22", "192.0.2.7", Some(22)),
            // Unbracketed IPv6 is an ADDRESS, never a port: its colons are part of it.
            ("::1", "::1", None),
            ("fe80::1", "fe80::1", None),
            ("root@fe80::1", "root@fe80::1", None),
            // A port beside IPv6 needs the bracket, which is the only thing that disambiguates.
            ("[::1]:2222", "::1", Some(2222)),
            ("[fe80::1]", "fe80::1", None),
            ("deploy@[::1]:2222", "deploy@::1", Some(2222)),
        ] {
            let host = HostId::new(raw).unwrap_or_else(|e| panic!("{raw} rejected: {e:?}"));
            assert_eq!(host.as_str(), destination, "destination of {raw}");
            assert_eq!(host.port(), port, "port of {raw}");
        }
    }

    #[test]
    fn a_port_reaches_ssh_as_an_option_never_glued_to_the_destination() {
        let host = HostId::new("localhost:2222").expect("valid destination");
        let marker = SessionMarker::new("n1", 1).expect("valid nonce");
        let argv: Vec<String> = SshDriver::default()
            .argv(&host, &marker)
            .into_iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        let at = argv.iter().position(|a| a == "-p").expect("-p present");
        assert_eq!(
            argv.get(at.saturating_add(1)).map(String::as_str),
            Some("2222")
        );
        assert!(
            argv.iter().any(|a| a == "localhost"),
            "the destination must be the bare host: `localhost:2222` in destination position \
             would be resolved as a hostname"
        );
        assert!(!argv.iter().any(|a| a == "localhost:2222"));
    }

    #[test]
    fn a_malformed_port_or_bracket_is_refused_rather_than_guessed() {
        for (raw, why) in [
            ("web1:", crate::HostIdRejected::PortNotAPort),
            ("web1:0", crate::HostIdRejected::PortNotAPort),
            ("web1:99999", crate::HostIdRejected::PortNotAPort),
            ("web1:ssh", crate::HostIdRejected::PortNotAPort),
            ("[::1", crate::HostIdRejected::BracketUnclosed),
            (":2222", crate::HostIdRejected::NoHost),
        ] {
            assert_eq!(HostId::new(raw), Err(why), "{raw}");
        }
    }

    #[test]
    fn an_option_shaped_destination_is_refused_before_it_can_reach_argv() {
        assert_eq!(
            HostId::new("-oProxyCommand=touch /tmp/pwned"),
            Err(crate::HostIdRejected::LeadingDash)
        );
    }
}
