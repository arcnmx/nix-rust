extern crate rustc_version;
use rustc_version::version_matches;

fn main() {
	if version_matches(">= 1.3.0") {
        println!("cargo:rustc-cfg=std_has_cstr_toowned");
	}

	if version_matches("< 1.7.0") {
        println!("cargo:rustc-cfg=feature=\"nixstring\"");
	}
}
