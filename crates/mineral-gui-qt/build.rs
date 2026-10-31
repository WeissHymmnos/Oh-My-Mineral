use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/qt_ui.cpp");
    println!("cargo:rerun-if-changed=src/qt_widgets.hpp");
    println!("cargo:rerun-if-changed=src/logo_data.hpp");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("src/qt_ui.cpp")
        .define("NOMINMAX", None)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17")
        .flag_if_supported("/permissive-")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/utf-8")
        .flag_if_supported("/EHsc");

    if let Ok(qt) = pkg_config::Config::new().probe("Qt6Widgets") {
        for include in &qt.include_paths {
            build.include(include);
        }
        for fw_path in &qt.framework_paths {
            build.flag(&format!("-F{}", fw_path.display()));
            for fw in ["QtWidgets", "QtGui", "QtCore"] {
                let hdr = fw_path.join(format!("{fw}.framework/Headers"));
                if hdr.is_dir() {
                    build.include(hdr);
                }
            }
        }
    } else if let Ok(qt_root) = std::env::var("QT_ROOT_DIR") {
        let root = PathBuf::from(qt_root);
        let inc = root.join("include");
        build.include(&inc);
        for sub in ["QtCore", "QtGui", "QtWidgets"] {
            let sub_inc = inc.join(sub);
            if sub_inc.is_dir() {
                build.include(sub_inc);
            }
        }
        let lib_dir = root.join("lib");
        if lib_dir.is_dir() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            if cfg!(target_os = "macos") {
                build.flag(&format!("-F{}", lib_dir.display()));
                println!("cargo:rustc-link-search=framework={}", lib_dir.display());
                for fw in ["QtWidgets", "QtGui", "QtCore"] {
                    let hdr = lib_dir.join(format!("{fw}.framework/Headers"));
                    if hdr.is_dir() {
                        build.include(hdr);
                    }
                    println!("cargo:rustc-link-lib=framework={fw}");
                }
            } else {
                println!("cargo:rustc-link-lib=Qt6Widgets");
                println!("cargo:rustc-link-lib=Qt6Gui");
                println!("cargo:rustc-link-lib=Qt6Core");
            }
        }
    } else {
        panic!("Qt6Widgets not found via pkg-config or QT_ROOT_DIR");
    }

    build.compile("mineral_qt_ui");
}
