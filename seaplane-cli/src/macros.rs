macro_rules! _print {
    (@$color:ident, $ptr:ident, $($args:tt)+) => {{
        use ::std::io::Write;

        let mut ptr = $crate::printer::$ptr();
        ptr.set_color($crate::printer::Color::$color);
        let _ = ::std::write!(ptr, $($args)+);
        ptr.reset();
    }};
    ($ptr:ident, $($args:tt)+) => {{
        use ::std::io::Write;

        let _ = ::std::write!($crate::printer::$ptr(), $($args)+);
    }};
}

// Print is akin to info! level messages
macro_rules! cli_print {
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Info {
            _print!(@$color, printer, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Info {
            _print!(printer, $($args)+);
        }
    }};
}

// Akin to info! level messages
macro_rules! cli_println {
    (@$color:ident, $($args:tt)+) => {{
        cli_print!(@$color, $($args)+);
        cli_print!("\n");
    }};
    // TODO: change to zero or more (*)
    ($($args:tt)+) => {{
        cli_print!($($args)+);
        cli_print!("\n");
    }}
}

// akin to error! level messages
macro_rules! cli_eprint {
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Error {
            _print!(@$color, eprinter, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Error {
            _print!(eprinter, $($args)+);
        }
    }}
}

// Akin to error! level messages
macro_rules! cli_eprintln {
    (@$color:ident, $($args:tt)+) => {{
        cli_eprint!(@$color, $($args)+);
        cli_eprint!("\n");
    }};
    ($($args:tt)*) => {{
        cli_eprint!($($args)+);
        cli_eprint!("\n");
    }}
}

/// Writes an error message to stderr and exits the process
#[allow(unused_macros)]
macro_rules! cli_bail {
    (@impl $prefix:expr, $status:expr, $($args:tt)*) => {{
        cli_eprint!(@Red, $prefix);
        cli_eprintln!($($args)+);
        ::std::process::exit($status);
    }};
    (@prefix $prefix:expr, @status $status:expr, $($args:tt)+) => {{
        cli_bail!(@impl $prefix, $status, $($args)+);
    }};
    (@status $status:expr, $($args:tt)+) => {{
        cli_bail!(@impl "error: ", $status, $($args)+);
    }};
    (@prefix $prefix:expr, $($args:tt)+) => {{
        cli_bail!(@impl $prefix, 1, $($args)+);
    }};
    ($($args:tt)*) => {{
        cli_bail!(@impl "error: ", 1, $($args)+);
    }};
}

// Akin to warn! level messages.
//
// The *ln variants it's more common to want a oneshot message with a
// "warn: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_warn {
    (@prefix, @$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(@Yellow, printer, "warn: ");
            _print!(@$color, printer, $($args)+);
        }
    }};
    (@prefix, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(printer, "warn: ");
            _print!(printer, $($args)+);
        }
    }};
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(@$color, printer, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(printer, $($args)+);
        }
    }};
}

// Akin to warn! level messages.
//
// The *ln variants it's more common to want a oneshot message with a
// "warn: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_warnln {
    (@noprefix, @$color:ident, $($args:tt)+) => {{
        cli_warn!(@$color, $($args)+);
        cli_warn!("\n");
    }};
    // TODO: change to zero or more (*)
    (@noprefix, $($args:tt)+) => {{
        cli_warn!($($args)+);
        cli_warn!("\n");
    }};
    (@$color:ident, $($args:tt)+) => {{
        cli_warn!(@prefix, @$color, $($args)+);
        cli_warn!("\n");
    }};
    // TODO: change to zero or more (*)
    ($($args:tt)+) => {{
        cli_warn!(@prefix, $($args)+);
        cli_warn!("\n");
    }}
}

// Akin to debug! level messages
//
// The *ln variants it's more common to want a oneshot message with a
// "warn: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_debug {
    (@prefix, @$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(@$color, printer, "DEBUG: ");
            _print!(@$color, printer, $($args)+);
        }
    }};
    (@prefix, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(printer, "DEBUG: ");
            _print!(printer, $($args)+);
        }
    }};
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(@$color, printer, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(printer, $($args)+);
        }
    }};
}

// Akin to the debug! level messages.
//
// The *ln variants it's more common to want a oneshot message with a
// "DEBUG: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_debugln {
    (@noprefix, @$color:ident, $($args:tt)+) => {{
        cli_debug!(@$color, $($args)+);
        cli_debug!("\n");
    }};
    // TODO: change to zero or more (*)
    (@noprefix, $($args:tt)+) => {{
        cli_debug!($($args)+);
        cli_debug!("\n");
    }};
    (@$color:ident, $($args:tt)+) => {{
        cli_debug!(@prefix, @$color, $($args)+);
        cli_debug!("\n");
    }};
    // TODO: change to zero or more (*)
    ($($args:tt)+) => {{
        cli_debug!(@prefix, $($args)+);
        cli_debug!("\n");
    }}
}
