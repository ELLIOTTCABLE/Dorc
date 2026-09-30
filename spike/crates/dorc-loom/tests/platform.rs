//! Tests for conditions that only some platforms have.
//!
//! A `for_<platform>` module makes the condition of that platform in a scratch directory. Its tests
//! run on each platform that can make the condition. A `<platform>` module runs only on that
//! platform.

/// macOS keeps its temporary directory below `/var`, and `/var` is a link to `/private/var`.
///
/// The staging store refuses a root that has a link in its path. The `-C` resolution and the tests
/// of the store must give the store a resolved path.
#[cfg(unix)]
mod for_macos {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;

    use dorc_loom::{FsStagingStore, Roots};

    struct Scratch(PathBuf);

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_root_below_a_link_is_refused_until_resolved() {
        let temp = fs::canonicalize(std::env::temp_dir()).expect("temp dir");
        let scratch = Scratch(temp.join(format!("dorc-loom-platform-root-{}", std::process::id())));
        let _ = fs::remove_dir_all(&scratch.0);
        fs::create_dir_all(scratch.0.join("private").join("root")).expect("root");
        symlink(scratch.0.join("private"), scratch.0.join("var")).expect("link");
        let through_link = scratch.0.join("var").join("root");

        assert!(FsStagingStore::new(&through_link).is_err());
        let resolved = fs::canonicalize(&through_link).expect("resolve");
        assert!(FsStagingStore::new(resolved).is_ok());
    }

    #[test]
    fn a_tree_named_through_a_link_resolves_before_the_store_checks_it() {
        let temp = fs::canonicalize(std::env::temp_dir()).expect("temp dir");
        let scratch = Scratch(temp.join(format!("dorc-loom-platform-tree-{}", std::process::id())));
        let _ = fs::remove_dir_all(&scratch.0);
        fs::create_dir_all(scratch.0.join("private").join("target")).expect("tree");
        symlink(scratch.0.join("private"), scratch.0.join("var")).expect("link");

        let roots = Roots::at(scratch.0.join("var").to_str().expect("UTF-8 path")).expect("-C");
        assert!(FsStagingStore::new(roots.staging_root()).is_ok());
    }
}
