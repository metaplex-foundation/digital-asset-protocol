use std::path::PathBuf;
// build.rs
use bebop_tools as bebop;
use bebop_tools::BuildConfig;

fn main() {
    // download the bebop binary automatically and cache it into your target directory
    // it will automatically download the same version as the package you installed
    bebop::download_bebopc(PathBuf::from("target").join("bebopc"));
    // build all `.bop` schemas in `schemas` dir and make a new module `generated` in `src` with all of them.
    bebop::build_schema_dir(
        "schemas",
        "src/generated",
        &BuildConfig {
            skip_generated_notice: false,
            generate_module_file: true,
            format_files: true,
        },
    );
}
