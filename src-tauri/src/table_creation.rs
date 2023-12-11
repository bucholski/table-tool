use arboard;
use std::{io, num::ParseIntError};

pub fn generate_new_table() {}

pub fn new_table(height: &str, width: &str) -> String {
    let dimensions: (u32, u32) = (height.parse().unwrap(), width.parse().unwrap());
    let new_table = generate_html_table(dimensions.0, dimensions.1);
    new_table
}

fn generate_html_table(height: u32, width: u32) -> String {
    let mut table = String::from(
        "<meta charset=\"utf-8\"><b style=\"font-weight:normal;\"> 
    <div dir=\"ltr\">
    <table>
    <colgroup>
    <col/><col/><col/><col/></colgroup>
    <tbody>
    ",
    );

    for _ in 0..height {
        table.push_str("<tr style=\"height:0pt\">");
        for _ in 0..width {
            table.push_str(
                "
                <td>
                <input/>
                </td>
                ",
            );
        }
        table.push_str("</tr>");
    }
    table.push_str("</tbody></table></div><br /></b>");

    table
}

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
