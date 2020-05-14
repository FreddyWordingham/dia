//! Columns macro.

/// Report a row of values in equally spaced columns.
#[macro_export]
macro_rules! columns {
    ($one: expr) => {
        log::info!("{:<32}", $one);
    };

    ($one: expr, $two: expr) => {
        log::info!("{:<32}{:^16}", $one, $two);
    };

    ($one: expr, $two: expr, $three: expr) => {
        log::info!("{:<32}{:^16}{:^16}", $one, $two, $three);
    };

    ($one: expr, $two: expr, $three: expr, $four: expr) => {
        log::info!("{:<32}{:^16}{:^16}{:^16}", $one, $two, $three, $four);
    };

    ($one: expr, $two: expr, $three: expr, $four: expr, $five: expr) => {
        log::info!(
            "{:<32}{:^16}{:^16}{:^16}{:^16}",
            $one,
            $two,
            $three,
            $four,
            $five
        );
    };
}
