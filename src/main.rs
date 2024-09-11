use std::error::Error;
mod event_watcher;

fn main() -> Result<(), Box<dyn Error>> {
    let debouncer = event_watcher::create_debouncer();

    Ok(())
}
