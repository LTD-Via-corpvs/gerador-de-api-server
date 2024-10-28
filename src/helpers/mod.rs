macro_rules! ternary {
    ($test:expr => $true_expr:expr, $false_expr:expr) => {
        if $test { $true_expr } else { $false_expr }
    }
}

pub(crate) use ternary;