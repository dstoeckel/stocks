extern crate serde_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let files = vec![
        "item.rs",
        "location.rs",
        "product.rs",
        "shelf.rs",
        "user.rs",
    ];

    let src_dir = Path::new("src/routes/");

    for f in files {
        let src = src_dir.join(format!("{}.in", f));
        let dst = Path::new(&out_dir).join(f);

        serde_codegen::expand(&src, &dst).unwrap();
    }
}
