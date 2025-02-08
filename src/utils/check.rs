use std::env;

pub fn is_node_in_path() -> bool {
    match env::var("PATH") {
        Ok(path) => {
            #[cfg(unix)]
            let node_path_string = ".nue/node/bin";
            #[cfg(windows)]
            let node_path_string = r"nue\\node";

            for path in env::split_paths(&path) {
                if path.ends_with(node_path_string) {
                    return true;
                }
            }

            false
        }
        Err(_) => false,
    }
}
