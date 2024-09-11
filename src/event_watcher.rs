use notify::{RecursiveMode, Result};
use notify_debouncer_mini::{
    new_debouncer, DebounceEventResult, DebouncedEvent, DebouncedEventKind,
};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn create_debouncer() -> Result<()> {
    let mut debouncer =
        new_debouncer(
            Duration::from_millis(1000),
            |res: DebounceEventResult| match res {
                Ok(events) => events
                    .iter()
                    .for_each(|e| process_event(e, "js").expect("reason")),
                Err(e) => println!("Error: {:?}", e),
            },
        )?;

    let _ = debouncer
        .watcher()
        .watch(Path::new(""), RecursiveMode::Recursive);

    println!("Watching directory... press enter to exit.");

    // on stdin enter pressed, code will resume and call Ok(())
    std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}

fn process_event(event: &DebouncedEvent, required_ext: &str) -> Result<()> {
    let event_kind: &DebouncedEventKind = &event.kind;
    let event_path: &PathBuf = &event.path;
    // notify mini debouncer does not have modify type, but "Any" type works for
    // modifications/on save
    let required_ext_pathbuf = PathBuf::from(required_ext);
    let event_path_extension = event.path.extension();
    match event_path_extension {
        Some(path) => {
            let event_path_extension_pathbuf = PathBuf::from(path);
            if required_ext_pathbuf == event_path_extension_pathbuf
                && event_kind == &DebouncedEventKind::Any
                && event_path.exists()
            {
                println!("{:?}", event_path);
            }
        }
        None => {
            eprintln!("No path extension.");
        }
    };

    Ok(())
}

fn run_file(path: &PathBuf) {
    todo!()
}
