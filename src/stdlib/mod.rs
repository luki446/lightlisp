#[macro_use]
macro_rules! stdlib_expand {
    ($env:ident, $($fn_name:expr => $fn_ident:ident),+) => {
        $($env.data.insert($fn_name.to_string(), Cell::BuiltIn($fn_ident));)+
    };
}

pub mod io;
pub mod logic;
pub mod math;
