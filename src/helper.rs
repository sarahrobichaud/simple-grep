use std::env;

const THRUTHY_VALUES: [&str; 4] = ["1", "true", "yes", "on"];
const FALSY_VALUES: [&str; 5] = ["0", "false", "no", "off", ""];

pub fn print_divider() {
    println!("-----------------------");
}

pub fn env_bool(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => match value.trim().to_ascii_lowercase().as_str() {
            v if THRUTHY_VALUES.contains(&v) => true,
            v if FALSY_VALUES.contains(&v) => false,
            _ => {
                eprintln!("warning: {name}={value:?} is not a valid bool, using {default}");
                default
            }
        },
        Err(_) => default,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn lock_env() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap()
    }

    #[test]
    fn env_parses_truthy() {
        let _guard = lock_env();

        for value in THRUTHY_VALUES {
            #[allow(unused_unsafe)]
            unsafe {
                env::set_var("IO_PROJECT_TEST_BOOL", value)
            };

            assert!(env_bool("IO_PROJECT_TEST_BOOL", false));
        }
        unsafe { env::remove_var("IO_PROJECT_TEST_BOOL") };
    }

    #[test]
    fn env_parses_falsies() {
        let _guard = lock_env();

        for value in FALSY_VALUES {
            #[allow(unused_unsafe)]
            unsafe {
                env::set_var("IO_PROJECT_TEST_BOOL", value)
            };

            assert!(!env_bool("IO_PROJECT_TEST_BOOL", false));
        }
        unsafe { env::remove_var("IO_PROJECT_TEST_BOOL") };
    }

    #[test]
    fn env_invalid_value_uses_default() {
        let _guard = lock_env();

        #[allow(unused_unsafe)]
        unsafe {
            env::set_var("IO_PROJECT_TEST_BOOL", "invalid_value")
        };

        assert!(!env_bool("IO_PROJECT_TEST_BOOL", false));
        assert!(env_bool("IO_PROJECT_TEST_BOOL", true));

        unsafe { env::remove_var("IO_PROJECT_TEST_BOOL") };
    }

    #[test]
    fn env_unset_uses_default() {
        let _guard = lock_env();

        assert!(!env_bool("IO_PROJECT_TEST_UNSET", false));
        assert!(env_bool("IO_PROJECT_TEST_UNSET", true));
    }
}
