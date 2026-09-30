//! Tests for conditions that only some platforms have.
//!
//! A `for_<platform>` module makes the condition of that platform, and runs on each platform that
//! can. An `on_<platform>` module runs only on that platform.

/// macOS keeps both roots in one directory below `HOME`, and reads no XDG variable.
mod for_macos {
    use dorc_cli::durable::{RootEnvironment, standard_roots};
    use dorc_receipt_local::{RootPlatform, RootRole};

    struct Variables;

    impl RootEnvironment for Variables {
        fn var(&self, name: &str) -> Option<String> {
            match name {
                "HOME" => Some("/Users/x".to_owned()),
                "XDG_CONFIG_HOME" => Some("/xdg/config".to_owned()),
                "XDG_STATE_HOME" => Some("/xdg/state".to_owned()),
                _ => None,
            }
        }
    }

    #[test]
    fn both_roots_are_application_support_below_home() {
        let roots = standard_roots(RootPlatform::MacOs, &Variables).expect("HOME is set");
        for role in RootRole::ALL {
            assert_eq!(roots.base(role), "/Users/x/Library/Application Support");
        }
    }
}
