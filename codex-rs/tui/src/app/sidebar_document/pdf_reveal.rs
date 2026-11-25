use std::path::Path;
use std::process::Command;

pub(super) fn command(path: &Path) -> Option<Command> {
    if !path.is_absolute()
        || !path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
    {
        return None;
    }

    let mut command = Command::new("nautilus");
    command.arg("--select").arg("--").arg(path);
    Some(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn selects_absolute_pdfs_in_nautilus() {
        for path in ["/tmp/Guide.pdf", "/tmp/Scans/Study Guide (2026).PDF"] {
            let command = command(Path::new(path)).expect("PDF reveal command");
            assert_eq!(command.get_program(), "nautilus");
            assert_eq!(
                command.get_args().collect::<Vec<_>>(),
                ["--select", "--", path].map(OsStr::new)
            );
        }
    }

    #[test]
    fn skips_relative_paths_and_other_file_types() {
        for path in ["guide.pdf", "/tmp/guide.md", "/tmp/guide.pdf.bak"] {
            assert!(command(Path::new(path)).is_none(), "{path}");
        }
    }
}
