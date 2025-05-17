#[allow(unused_imports)]
use hacker::RBR_ProxyInit;

mod hacker;
pub mod game;
pub mod plugin;

/**
 * Must initialize before any other funciton calls.
 */
pub fn rbrproxy_env_init() {
    #[cfg(target_os = "windows")]
    unsafe { RBR_ProxyInit(); }
}