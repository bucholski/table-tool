use arboard;
use std::{io, num::ParseIntError};

fn populate_clipboard(new_table: String) {
    let mut clipboard = match arboard::Clipboard::new() {
        Ok(instance) => instance,
        Err(e) => panic!("Clipboard initialization failed:\n\r {e}"),
    };
    match clipboard.set_html(
        new_table,
        Some("This field doesn't accept HTML".to_string()),
    ) {
        Ok(_) => println!("Your table has been copied into the clipboard"),
        Err(e) => println!("Error:\n\r {e}"),
    }
}
