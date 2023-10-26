#![feature(decl_macro)]

use agera_sdk_application_descriptor::ApplicationDescriptor;
use maplit::hashmap;
use cfg_if::cfg_if;

/// Starts the build script. The build script must not define a `main`
/// function and rather write a main action using this macro.
/// 
/// # Syntax
/// 
/// ```
/// agera_sdk_build::start!({
///     // Main action
/// });
/// ```
pub macro start {
    ($start_action:expr) => {
        fn main() {
            ::agera_sdk_build::__bootstrap(::std::env::var("OUT_DIR").unwrap().as_ref());
            $start_action;
        }
    },
}

#[doc(hidden)]
pub fn __bootstrap(output_directory: &str) {
    use late_format::LateFormat;

    let output_directory = std::path::PathBuf::from(output_directory);
    let project_path = std::env::current_dir().unwrap();
    let project_path = project_path.to_str().unwrap();
    let descriptor = ApplicationDescriptor::from_project(project_path).unwrap();

    // Create {output_directory}/agera_sdk_build
    let build_path = output_directory.join("agera_sdk_build");
    std::fs::create_dir_all(build_path).unwrap();

    // Write to {output_directory}/agera_sdk_build/bootstrap.rs
    let bootstrap_rs_path = output_directory.join("agera_sdk_build/bootstrap.rs");
    std::fs::write(bootstrap_rs_path, include_str!("./template_code/bootstrap.rs").late_format(hashmap! {
        "id".into() => descriptor.id.clone(),
        "install_files".into() => install_files_web(&descriptor),
    })).unwrap();

    #[cfg(debug_assertions)] {
        // Reset directory {output_directory}/agera_sdk_build/debug_storage_files
        let storage_path = output_directory.join("agera_sdk_build/debug_storage_files");
        drop(std::fs::remove_dir_all(storage_path.clone()));
        std::fs::create_dir_all(storage_path.clone()).unwrap();
    }
}

/// Application's installation files are embedded in the web export
/// through:
/// 
/// ```
/// let app_root = File::application_directory();
/// let _ = app_root.resolve_path(relative_path).write_async(include_bytes!("path")).await;
/// ```
fn install_files_web(descriptor: &ApplicationDescriptor) -> String {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use file_paths::FlexPath;
            let mut write_calls = vec![
                "    // \"app:\" files".to_owned(),
            ];
            let file_paths = descriptor.glob_install_files().expect("'applicationDescriptor.installFiles' contains invalid paths");
            for file_path in file_paths {
                let current_directory = std::env::current_dir().unwrap().to_string_lossy().into_owned();
                let current_directory_flex = FlexPath::new_native(&current_directory);
                let file_path = current_directory_flex.resolve(&file_path).to_string();
                let relative_file_path = current_directory_flex.relative(&file_path);
                write_calls.push(format!("    let _ = app_root.resolve_path(\"{relative_file_path}\").write_async(include_bytes!(\"{file_path}\")).await.unwrap();"));
            }
            write_calls.join("\n")
        } else {
            let _ = descriptor;
            "".into()
        }
    }
}