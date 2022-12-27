use ansi_term::Colour;

pub fn print_error<S>(error: S) -> !
where
    S: Sized + ToString,
{
    eprintln!("{}", Colour::Red.paint(error.to_string()));

    std::process::exit(1)
}

pub fn print_warning<S>(warning: S) -> !
where
    S: Sized + ToString,
{
    eprintln!("{}", Colour::Yellow.paint(warning.to_string()));

    std::process::exit(1)
}
