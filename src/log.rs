// dependency injection
pub use log::*;

pub fn is_enable(target: &str) -> bool {
    __private_api_enabled(Level::Debug, target)
}



#[macro_export]
macro_rules! info {
    (target: $target:expr, req: $req:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, req: $req, $crate::Level::Info, $($arg)+)
    );
    (req: $req:expr, $($arg:tt)+) => (
        $crate::log!(req: $req, $crate::Level::Info, $($arg)+)
    );
    (exclude: $exclude:expr, target: $target:expr, $($arg:tt)+) => (
        $crate::log!(exclude: $exclude, target: $target, $crate::Level::Info, $($arg)+)
    );
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::Level::Info, $($arg)+)
    );
    (exclude: $exclude:expr, $($arg:tt)+) => (
        $crate::log!(exclude: $exclude, $crate::Level::Info, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Info, $($arg)+)
    );
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, req: $req:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, req: $req, $crate::Level::Debug, $($arg)+)
    );
    (req: $req:expr, $($arg:tt)+) => (
        $crate::log!(req: $req, $crate::Level::Debug, $($arg)+)
    );
    (exclude: $exclude:expr, target: $target:expr, $($arg:tt)+) => (
        $crate::log!(exclude: $exclude, target: $target, $crate::Level::Debug, $($arg)+)
    );
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::Level::Debug, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Debug, $($arg)+)
    );
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, req: $req:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, req: $req, $crate::Level::Trace, $($arg)+)
    );
    (req: $req:expr, $($arg:tt)+) => (
        $crate::log!(req: $req, $crate::Level::Trace, $($arg)+)
    );
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::Level::Trace, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Trace, $($arg)+)
    );
}


#[macro_export]
macro_rules! error {
    (target: $target:expr, req: $req:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, req: $req, $crate::Level::Error, $($arg)+)
    );
    (req: $req:expr, $($arg:tt)+) => (
        $crate::log!(req: $req, $crate::Level::Error, $($arg)+)
    );
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::Level::Error, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Error, $($arg)+)
    );
}

#[macro_export]
macro_rules! log {
    (target: $target:expr, req: $req:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            let mut old: $crate::ReqId = $req.save();
            $crate::__private_api_log(
                format_args!($($arg)+),
                lvl,
                &($target, $crate::__log_module_path!(), $crate::__log_file!(), $crate::__log_line!()),
            );
            let _ = old.save();
        }
    });
    (exclude: $exclude:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if $crate::should_log($exclude) {
            $crate::log!(target: $target, lvl, $($arg)+)
        }
    });
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::__private_api_log(
                format_args!($($arg)+),
                lvl,
                &($target, $crate::__log_module_path!(), $crate::__log_file!(), $crate::__log_line!()),
            );
        }
    });
    (req: $req:expr, $lvl:expr, $($arg:tt)+) => ($crate::log!(target: $crate::__log_module_path!(), req: $req, $lvl, $($arg)+));
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ($crate::log!(target: $target, $lvl, $($arg)+));
    (exclude: $exclude:expr, $lvl:expr, $($arg:tt)+) => ($crate::log!(exclude: $exclude, target: $crate::__log_module_path!(), $lvl, $($arg)+));
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: $crate::__log_module_path!(), $lvl, $($arg)+));
}