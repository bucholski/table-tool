//TODO
//every From method should return a Result e.g. Result<Ok(Cell), Err(e)>

#[derive(Debug, PartialEq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub rowspan: u32,
    pub colspan: u32,
    pub data: String,
}

#[derive(Debug, PartialEq)]
pub struct Row {
    pub order: u32,
    pub cells: Vec<Cell>,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub rows: Vec<Row>,
}

// pub struct HTLM {
//     pub data: String,
// }

pub trait ToHtml {
    fn to_html(&self) -> String;
}

impl From<&String> for Cell {
    fn from(html_cell: &String) -> Cell {
        let values: Vec<String> = html_cell
            .trim_start_matches("<td x=")
            .replace(" y=", ",")
            .replace(" rowspan=", ",")
            .replace(" colspan=", ",")
            .replace(" ><input>", ",")
            .trim_end_matches("</input></td>")
            .split(',')
            .map(|x| x.to_string())
            .collect();
        new_cell(
            values[0]
                .parse()
                .unwrap_or(panic!("Value of x failed when paring html cell to struct")),
            values[1]
                .parse()
                .unwrap_or(panic!("Value of y failed when paring html cell to struct")),
            values[2].parse().unwrap_or(panic!(
                "Value of colspan failed when paring html cell to struct"
            )),
            values[3].parse().unwrap_or(panic!(
                "Value of rowspan failed when paring html cell to struct"
            )),
            values[4].to_string(),
        )
    }
}

impl From<&String> for Row {
    fn from(html_row: &String) -> Row {
        let html_cells: Vec<&str> = html_row
            .trim_start_matches("<tr>")
            .trim_end_matches("</tr>")
            .split_inclusive("</td>")
            .collect();
        let cells: Vec<Cell> = html_cells
            .iter()
            .map(|html_cell| Cell::from(&html_cell.to_string()))
            .collect();
        Row {
            order: cells[0].y,
            cells,
        }
    }
}

impl From<&String> for Table {
    fn from(html_table: &String) -> Table {
        let html_rows: Vec<&str> = html_table
            .trim_start_matches("<table>")
            .trim_end_matches("</table>")
            .split_inclusive("</tr>")
            .collect();
        let rows: Vec<Row> = html_rows
            .iter()
            .map(|html_row| Row::from(&html_row.to_string()))
            .collect();

        Table { rows }
    }
}

impl ToHtml for Cell {
    fn to_html(&self) -> String {
        format!(
            "<td x={} y={} rowspan={} colspan={} ><input>{}</input></td>",
            &self.x, &self.y, &self.rowspan, &self.colspan, &self.data
        )
    }
}

impl ToHtml for Row {
    fn to_html(&self) -> String {
        let html_cells: String = self
            .cells
            .iter()
            .map(|rust_cell| rust_cell.to_html())
            .collect();
        format!("<tr>{}</tr>", html_cells)
    }
}

impl ToHtml for Table {
    fn to_html(&self) -> String {
        let html_rows: String = self
            .rows
            .iter()
            .map(|rust_row| rust_row.to_html())
            .collect();
        format!("<table>{}</table>", html_rows)
    }
}

pub fn new_cell(x: u32, y: u32, rowspan: u32, colspan: u32, data: String) -> Cell {
    Cell {
        x,
        y,
        rowspan,
        colspan,

        data: data,
    }
}

pub fn new_row(width: u32, order: u32) -> Row {
    let mut vec_of_cells: Vec<Cell> = Vec::new();
    for x in 0..width {
        vec_of_cells.push(new_cell(x, order, 1, 1, "".to_string()))
    }
    Row {
        order,
        cells: vec_of_cells,
    }
}

pub fn new_table(height: u32, width: u32) -> Table {
    let mut vec_of_rows: Vec<Row> = Vec::new();
    for y in 0..height {
        vec_of_rows.push((new_row(width, y)))
    }
    Table { rows: vec_of_rows }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cell_rust_to_html_test() {
        let rust_cell = new_cell(1, 1, 1, 2, String::from("hioh!"));
        let html_cell = "<td x=1 y=1 rowspan=1 colspan=2 ><input>hioh!</input></td>".to_string();
        assert_eq!(rust_cell.to_html(), html_cell);
    }
    #[test]
    fn cell_html_to_rust_test() {
        let rust_cell = new_cell(1, 2, 2, 2, String::from("hio222h!"));
        let html_cell = "<td x=1 y=2 rowspan=2 colspan=2 ><input>hio222h!</input></td>".to_string();

        assert_eq!(Cell::from(&html_cell), rust_cell);
    }
    #[test]
    fn table_parsing_test() {
        let html_table = String::from("<table><tr><td x=0 y=0 rowspan=1 colspan=1 ><input></input></td><td x=1 y=0 rowspan=1 colspan=1 ><input></input></td><td x=2 y=0 rowspan=1 colspan=1 ><input></input></td></tr><tr><td x=0 y=1 rowspan=1 colspan=1 ><input></input></td><td x=1 y=1 rowspan=1 colspan=1 ><input>hioh</input></td><td x=2 y=1 rowspan=1 colspan=1 ><input></input></td></tr><tr><td x=0 y=2 rowspan=1 colspan=1 ><input></input></td><td x=1 y=2 rowspan=1 colspan=1 ><input>HOH</input></td><td x=2 y=2 rowspan=1 colspan=1 ><input></input></td></tr></table>");
        let rust_table = Table {
            rows: Vec::from([
                Row {
                    order: 0,
                    cells: Vec::from([
                        Cell {
                            x: 0,
                            y: 0,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                        Cell {
                            x: 1,
                            y: 0,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                        Cell {
                            x: 2,
                            y: 0,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                    ]),
                },
                Row {
                    order: 1,
                    cells: Vec::from([
                        Cell {
                            x: 0,
                            y: 1,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                        Cell {
                            x: 1,
                            y: 1,
                            rowspan: 1,
                            colspan: 1,
                            data: "hioh".to_string(),
                        },
                        Cell {
                            x: 2,
                            y: 1,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                    ]),
                },
                Row {
                    order: 2,
                    cells: Vec::from([
                        Cell {
                            x: 0,
                            y: 2,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                        Cell {
                            x: 1,
                            y: 2,
                            rowspan: 1,
                            colspan: 1,
                            data: "HOH".to_string(),
                        },
                        Cell {
                            x: 2,
                            y: 2,
                            rowspan: 1,
                            colspan: 1,
                            data: "".to_string(),
                        },
                    ]),
                },
            ]),
        };
        assert_eq!(html_table, Table::from(&rust_table.to_html()).to_html());
        assert_eq!(Table::from(&Table::from(&html_table).to_html()), rust_table);
    }
}
