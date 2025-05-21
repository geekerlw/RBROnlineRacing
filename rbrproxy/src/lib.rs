#[allow(unused_imports)]

#[cfg(target_os = "windows")]
mod hacker;
pub mod game;
pub mod plugin;

/**
 * Must initialize before any other funciton calls.
 */
#[cfg(target_os = "windows")]
pub fn rbrproxy_env_init() {
    use hacker::RBR_ProxyInit;
    unsafe { RBR_ProxyInit(); }
}

#[cfg(not(target_os = "windows"))]
pub fn rbrproxy_env_init() {

}