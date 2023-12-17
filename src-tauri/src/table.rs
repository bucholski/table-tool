//TODO
//every From -> TryFrom should return a Result e.g. Result<Ok(Cell), Err(e)>? ?????

use tauri::utils::html;

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub rowspan: usize,
    pub colspan: usize,
    pub data: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Row {
    pub order: usize,
    pub cells: Vec<Cell>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Table {
    pub rows: Vec<Row>,
    pub height: usize,
    pub width: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize, rowspan: usize, colspan: usize, data: String) -> Cell {
        Cell {
            x,
            y,
            rowspan,
            colspan,
            data,
        }
    }
    fn to_html(&self) -> String {
        let controller = match (self.x,self.y) {
          (0,0) => "<section class=\"controller\"><div class=\"row remove\"></div><div class=\"row add\"></div><div class=\"row add before\"></div><div class=\"column remove\"></div><div class=\"column add\"></div><div class=\"column add before\"></div></section>",
          (0,_) => "<section class=\"controller\"><div class=\"row remove\"></div><div class=\"row add\"></div></section>",
          (_,0) => "<section class=\"controller\"><div class=\"column remove\"></div><div class=\"column add\"></div></section>",
          _ =>"",
        };

        format!(
            "<td x={} y={} rowspan={} colspan={} ><input>{}</input>{}</td>",
            &self.x, &self.y, &self.rowspan, &self.colspan, &self.data, controller
        )
    }
}
impl Row {
    pub fn new(width: usize, order: usize) -> Row {
        let mut vec_of_cells: Vec<Cell> = Vec::new();
        for x in 0..width {
            vec_of_cells.push(Cell::new(x, order, 1, 1, "".to_string()))
        }
        Row {
            order,
            cells: vec_of_cells,
        }
    }

    fn to_html(&self) -> String {
        let html_cells: String = self
            .cells
            .iter()
            .map(|rust_cell| rust_cell.to_html())
            .collect();
        format!("<tr>{}</tr>", html_cells)
    }
}

impl Table {
    pub fn new(height: usize, width: usize) -> Table {
        let mut vec_of_rows: Vec<Row> = Vec::new();
        for y in 0..height {
            vec_of_rows.push(Row::new(width, y))
        }
        Table {
            rows: vec_of_rows,
            height,
            width,
        }
    }

    pub fn to_html(&self) -> String {
        let html_rows: String = self
            .rows
            .iter()
            .map(|rust_row| rust_row.to_html())
            .collect();
        format!("<table>{}</table>", html_rows)
    }

    pub fn add_row(&self, y: usize) -> Table {
        let mut new_table = self.clone();
        new_table.rows.insert(y, Row::new(self.width, y));
        new_table.rows.iter_mut().skip(y + 1).for_each(|row| {
            row.order += 1;
            row.cells.iter_mut().for_each(|cell| cell.y += 1)
        });
        new_table.height += 1;
        new_table
    }

    pub fn remove_row(&self, y: usize) -> Table {
        let mut new_table = self.clone();
        new_table.rows.remove(y);
        new_table.rows.iter_mut().skip(y).for_each(|row| {
            row.order -= 1;
            row.cells.iter_mut().for_each(|cell| cell.y -= 1)
        });
        new_table.height -= 1;
        new_table
    }

    pub fn add_column(&self, x: usize) -> Table {
        let mut new_table = self.clone();
        new_table.rows.iter_mut().for_each(|row| {
            row.cells
                .insert(x, Cell::new(x, row.order, 1, 1, "".to_string()));
            row.cells
                .iter_mut()
                .skip(x + 1)
                .for_each(|cell| cell.x += 1)
        });
        new_table.width += 1;
        new_table
    }

    pub fn remove_column(&self, x: usize) -> Table {
        let mut new_table = self.clone();
        new_table.rows.iter_mut().for_each(|row| {
            row.cells.remove(x);
            row.cells.iter_mut().skip(x).for_each(|cell| cell.x -= 1)
        });
        new_table.width -= 1;
        new_table
    }
}

impl From<&String> for Cell {
    fn from(html_cell: &String) -> Cell {
        let controller = (
            html_cell.find("<section").unwrap_or(0),
            html_cell.find("</section>").unwrap_or(0),
        );
        let mut html_cell = html_cell.clone();
        html_cell.drain(controller.0..(controller.1));
        let html_cell = html_cell.replace("</section>", "");

        let values: Vec<String> = html_cell
            .trim_start_matches("<td x=")
            .replace(" y=", "→→!!delimiter!!←←")
            .replace(" rowspan=", "→→!!delimiter!!←←")
            .replace(" colspan=", "→→!!delimiter!!←←")
            .replace(" ><input>", "→→!!delimiter!!←←")
            .trim_end_matches("</input></td>")
            .split("→→!!delimiter!!←←")
            .map(|x| x.to_string())
            .collect();
        dbg!(&values);
        Cell::new(
            values[0].parse().expect(&format!(
                "Parsing of 'x' value resulted in an error. The value was {}",
                values[0]
            )),
            values[1].parse().expect(&format!(
                "Parsing of 'y' value resulted in an error. The value was {}",
                values[1]
            )),
            values[2].parse().expect(&format!(
                "Parsing of 'rowspan' value resulted in an error. The value was {}",
                values[2]
            )),
            values[3].parse().expect(&format!(
                "Parsing of 'colspan' value resulted in an error. The value was {}",
                values[3]
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
        let (height, width): (usize, usize) = rows[0]
            .cells
            .iter()
            .fold((0, 0), |acc: (usize, usize), cell: &Cell| {
                (acc.0 + cell.colspan, acc.1 + cell.rowspan)
            });

        Table {
            rows,
            height,
            width,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cell_parsing_test() {
        let rust_cell = Cell::new(1, 2, 2, 2, String::from("hio222h!"));
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
            height: 3,
            width: 3,
        };
        assert_eq!(Table::from(&Table::from(&html_table).to_html()), rust_table);
    }
}
