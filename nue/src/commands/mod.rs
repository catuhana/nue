pub mod clean;
pub mod env;
pub mod install;
pub mod list;
pub mod uninstall;

pub fn is_node_in_path() -> bool {
    match std::env::var("PATH") {
        Ok(path) => {
            #[cfg(unix)]
            let node_path_string = ".nue/node/bin";
            #[cfg(windows)]
            let node_path_string = r"AppData\\Local\\Programs\\nue\\node";

            for path in std::env::split_paths(&path) {
                if path.ends_with(node_path_string) {
                    return true;
                }
            }

            false
        }
        Err(_) => false,
    }
}
