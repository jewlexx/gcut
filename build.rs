use cc::Build;

fn main() {
    if cfg!(windows) {
        Build::new()
            .file("src/win/coretemp.h")
            .file("src/win/plugin/CoreTempPlugin.h")
            .compile("coretemp");
    } else {
        panic!("Unsupported platform")
    }
}
