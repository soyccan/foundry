fn main() {
    pkg_config::probe_library("libpq").unwrap_or_else(|_| {
        panic!(
            "libpq not found, please install it with your package manager. Ubuntu: sudo apt install libpq5 libpq-dev"
        );
    });
}
