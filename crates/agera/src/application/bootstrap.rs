/// Starts the Agera application, executing an initial asynchronous
/// action. This performs bootstrapping of the application.
/// 
/// Defining a main Rust function instead of using `agera::application::start`
/// for executing initial actions may lead to panics.
/// 
/// # Syntax
/// 
/// ```ignore
/// agera::application::start!(async {
///     // Main action
/// });
/// ```
pub macro start {
    ($start_action:expr) => {
        async fn __boostrap() {
            if unsafe { ::agera::application::__agera_BOOTSTRAPPED } {
                panic!("'agera::application::start' must not be invoked more than once");
            }

            // Bootstrap
            include!(concat!(env!("OUT_DIR"), "/agera_sdk_build/bootstrap.rs")).await;

            // Setup file directories
            ::agera::file::__agera_File_bootstrap().await;

            // Start
            $start_action.await;
        }

        ::agera::platforms::if_native_platform! {
            use ::agera::platforms::tokio as __agera_target_tokio__;

            fn __start_local_set() -> ::agera::platforms::tokio::task::LocalSet {
                let __local_set = ::agera::platforms::tokio::task::LocalSet::new();
                __local_set.run_until(__bootstrap())
            }

            ::agera::common::cfg_if! {
                // Android
                if #[cfg(target_os = "android")] {
                    #[no_mangle]
                    fn android_main(app: AndroidApp) {
                        *(::agera::platforms::APPLICATION.write().unwrap()) = Some(app.clone());
                        ::std::fs::create_dir_all(&(::agera::file::application_installation_directory())).unwrap();
                        ::std::fs::create_dir_all(&(::agera::file::application_storage_directory())).unwrap();

                        #[__agera_target_tokio__::main(crate = "__agera_target_tokio__")]
                        async fn main() {
                            __start_local_set().await;
                        }

                        main();
                    }
                // Not { Android }
                } else {
                    #[__agera_target_tokio__::main(crate = "__agera_target_tokio__")]
                    async fn main() {
                        __start_local_set().await;
                    }
                }
            }
        }

        // Browser
        ::agera::platforms::if_browser! {
            fn main() {
                ::agera::common::future::exec(__bootstrap());
            }
        }
    },
}

/// Internal property.
#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub static mut __agera_BOOTSTRAPPED: bool = false;

pub(crate) macro assert_bootstrapped {
    () => {
        assert!(unsafe { crate::application::__agera_BOOTSTRAPPED }, "The application must be initialized through 'agera::application::start!(action)'.");
    },
}