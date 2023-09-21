#[cfg(feature = "compile_error")]
#[test]
fn compile_error() {
    use std::path::Path;

    let t = trybuild::TestCases::new();

    for entry in Path::new(file!())
        .parent()
        .unwrap()
        .join("compile_error")
        .read_dir()
        .unwrap()
    {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "rs" {
                if path.with_extension("stderr").exists() {
                    t.compile_fail(&path);
                } else {
                    t.pass(&path)
                }
            }
        }
    }
}
