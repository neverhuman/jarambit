fn main() {
    let root = std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("current directory: {e}");
        std::process::exit(1);
    });
    std::process::exit(jarambit_runner_policy::run(&root));
}
