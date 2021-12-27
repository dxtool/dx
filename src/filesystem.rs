use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Filesystem operations
pub trait Filesystem {
    /// Searches for a file named `name` starting in the path given by `start`. If the file is
    /// not found in the current search directory, the search continues in the parent of the
    /// directory, unless the current search directory is the path given by `stop`.__rust_force_expr!
    ///
    /// Returns the path to the file if found, or None if the file was not found or an error
    /// occurred during the search.
    fn find_file_ascending<P1: AsRef<Path>, P2: AsRef<Path>, S: AsRef<OsStr>>(
        &self,
        start: &P1,
        stop: &P2,
        name: &S,
    ) -> Option<PathBuf>;
}

/// Implementation of filesystem operations for the local filesystem
pub struct LocalFilesystem;

/// The local filesystem
pub static FS_LOCAL: LocalFilesystem = LocalFilesystem;

impl Filesystem for LocalFilesystem {
    fn find_file_ascending<P1: AsRef<Path>, P2: AsRef<Path>, S: AsRef<OsStr>>(
        &self,
        start: &P1,
        stop: &P2,
        name: &S,
    ) -> Option<PathBuf> {
        let (start, stop, name) = (start.as_ref(), stop.as_ref(), name.as_ref());

        for p in start.ancestors() {
            if p.file_name() == Some(name) && p.is_file() {
                return Some(p.into());
            } else if p.join(name).is_file() {
                return Some(p.join(name));
            } else if p == stop {
                return None;
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;

    #[test]
    fn not_found() {
        let stop = fs::canonicalize("test/filesystem/test-not-found").unwrap();
        let start = fs::canonicalize("test/filesystem/test-not-found/a/b").unwrap();

        let result = FS_LOCAL.find_file_ascending(&start, &stop, &"not-found");

        assert_eq!(result, None);
    }

    #[test]
    fn found_at_start() {
        let stop = fs::canonicalize("test/filesystem/test-found-at-start").unwrap();
        let start = fs::canonicalize("test/filesystem/test-found-at-start/a/b").unwrap();

        let result = FS_LOCAL.find_file_ascending(&start, &stop, &"found");

        let expected: PathBuf = [start.clone(), PathBuf::from("found")].iter().collect();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn found_at_stop() {
        let stop = fs::canonicalize("test/filesystem/test-found-at-stop").unwrap();
        let start = fs::canonicalize("test/filesystem/test-found-at-stop/a/b").unwrap();

        let result = FS_LOCAL.find_file_ascending(&start, &stop, &"found");

        let expected: PathBuf = [stop.clone(), PathBuf::from("found")].iter().collect();
        assert_eq!(result, Some(expected));
    }
}
