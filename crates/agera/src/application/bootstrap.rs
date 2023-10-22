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
        ::agera::target::if_native_target! {
            use ::agera::target::tokio as __agera_target_tokio__;

            ::agera::common::cfg_if! {
                // Android
                if #[cfg(target_os = "android")] {
                    #[no_mangle]
                    fn android_main(app: AndroidApp) {
                        *(::agera::target::APPLICATION.write().unwrap()) = Some(app.clone());
                        ::std::fs::create_dir_all(&(::agera::file::application_installation_directory())).unwrap();
                        ::std::fs::create_dir_all(&(::agera::file::application_storage_directory())).unwrap();

                        let local_task_set = ::agera::target::tokio::task::LocalSet::new();
                        local_task_set.run_until(async {
                            unsafe { ::agera::application::BOOTSTRAPPED = true; }
                            $start_action.await;
                        }).await;
                    }
                // Not { Android }
                } else {
                    #[__agera_target_tokio__::main(crate = "__agera_target_tokio__")]
                    async fn main() {
                        let local_task_set = ::agera::target::tokio::task::LocalSet::new();
                        local_task_set.run_until(async {
                            unsafe { ::agera::application::BOOTSTRAPPED = true; }
                            $start_action.await;
                        }).await;
                    }
                }
            }
        }

        // Browser
        ::agera::target::if_browser_target! {
            fn main() {
                ::agera::common::future::exec(async {
                    unsafe { ::agera::application::BOOTSTRAPPED = true; }
                    $start_action.await;
                });
            }
        }
    },
}

/// Internal property.
#[doc(hidden)]
pub static mut BOOTSTRAPPED: bool = false;

pub(crate) macro assert_bootstrapped {
    () => {
        assert!(unsafe { crate::application::BOOTSTRAPPED }, "The application must be initialized through 'agera::application::start!(action)'.");
    },
}