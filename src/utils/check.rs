use std::env;

pub fn is_node_in_path() -> bool {
    match env::var("PATH") {
        Ok(path) => {
            let node_path_string = if cfg!(unix) {
                ".nue/node/bin"
            } else {
                "nue\\\\\\node"
            };

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
