/// Starts the Agera application, executing an initial
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
            // Bootstrap
            include!(concat!(env!("OUT_DIR"), "/agera_sdk_build/bootstrap.rs"));
            
            // Setup file directories
            ::agera::file::__agera_File_bootstrap();

            // Start
            $start_action.await;
        }

        ::agera::target::if_native_target! {
            use ::agera::target::tokio as __agera_target_tokio__;

            fn __start_local_set() -> ::agera::target::tokio::task::LocalSet {
                let __local_set = ::agera::target::tokio::task::LocalSet::new();
                __local_set.run_until(__bootstrap())
            }

            ::agera::common::cfg_if! {
                // Android
                if #[cfg(target_os = "android")] {
                    #[no_mangle]
                    fn android_main(app: AndroidApp) {
                        *(::agera::target::APPLICATION.write().unwrap()) = Some(app.clone());
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
        ::agera::target::if_browser_target! {
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