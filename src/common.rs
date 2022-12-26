use ansi_term::Colour;

pub fn print_error<S>(error: S) -> !
where
    S: Sized + ToString,
{
    eprintln!("{}", Colour::Red.paint(error.to_string()));

    std::process::exit(1)
}
