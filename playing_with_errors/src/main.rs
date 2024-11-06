use playing_with_errors::run;

fn main() {
    match run() {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{:?}");
        }
    };
}
