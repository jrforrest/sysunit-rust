use std::env;

pub fn get_path_var(name: &str, default_dirs: &'static [&'static str]) -> Vec<String> {
    match env::var(name) {
        Ok(string) => string.split(":").map(|s| s.to_string()).collect(),
        Err(_) => default_dirs.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(unix)]
pub mod unix {
    use std::os::unix::fs::PermissionsExt;
    use std::fs::Metadata;

    pub fn is_executable_file(metadata: &Metadata) -> bool {
        let permissions = metadata.permissions();

        metadata.is_file() && (permissions.mode() & 0o111 != 0)
    }
}
