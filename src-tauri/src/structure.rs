#[derive(Debug, PartialEq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub rowspan: u32,
    pub colspan: u32,
    pub data: String,
}

#[derive(Debug)]
pub struct Row {
    pub order: u32,
    pub cells: Vec<Cell>,
}

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Row>,
}

trait Parse {
    fn to_html(&self) -> String;
}

impl From<String> for Cell {
    fn from(html_cell: String) -> Cell {
        let data_start = html_cell.find("<input>").unwrap();
        let data_end = html_cell.find("</input>").unwrap();
        let data_string: String = html_cell
            .clone()
            .drain(data_start..data_end)
            .collect::<String>()
            .trim_start_matches("<input>")
            .to_string();

        let data = match data_string.is_empty() {
            true => None,
            false => Some(data_string),
        };

        let cell_split: Vec<&str> = html_cell
            .trim_end_matches("></input></td>")
            .trim_start_matches("<td ")
            .split('=')
            .collect();
        let cell_split_2 = &cell_split
            .iter()
            .flat_map(|x| x.split(' '))
            .collect::<Vec<&str>>();
        let x = cell_split_2[1].parse().unwrap();
        let y = cell_split_2[3].parse().unwrap();
        let rowspan = match cell_split_2[5].parse() {
            Ok(a) => Some(a),
            Err(e) => {
                dbg!(e);
                None
            }
        };
        let colspan = match cell_split_2[7].parse() {
            Ok(a) => Some(a),
            Err(e) => None,
        };

        new_cell(x, y, rowspan, colspan, data)
    }
}

impl Parse for Cell {
    fn to_html(&self) -> String {
        format!(
            "<td x={} y={} rowspan={} colspan={} ><input>{}</input></td>",
            &self.x, &self.y, &self.rowspan, &self.colspan, &self.data
        )
    }
}

impl From<String> for Row {
    fn from(html_row: String) -> Row {
        let html_cells: Vec<&str> = html_row
            .trim_start_matches("<tr>")
            .trim_end_matches("</tr>")
            .split_inclusive("</td>")
            .collect();
        let cells: Vec<Cell> = html_cells
            .iter()
            .map(|html_cell| Cell::from(html_cell.to_string()))
            .collect();
        //finish after TODO above
        new_row(3, 1)
    }
}

impl Parse for Row {
    fn to_html(&self) -> String {
        let html_cells: String = self
            .cells
            .iter()
            .map(|rust_cell| rust_cell.to_html())
            .collect();
        format!("<tr>{}</tr>", html_cells)
    }
}

impl Parse for Table {
    fn to_html(&self) -> String {
        let html_rows: String = self
            .rows
            .iter()
            .map(|rust_row| rust_row.to_html())
            .collect();
        format!("<table>{}</table>", html_rows)
    }
}

pub fn new_cell(
    x: u32,
    y: u32,
    rowspan: Option<u32>,
    colspan: Option<u32>,
    data: Option<String>,
) -> Cell {
    Cell {
        x,
        y,
        rowspan: match rowspan {
            Some(a) => a,
            None => 1,
        },

        colspan: match colspan {
            Some(a) => a,
            None => 1,
        },

        data: match data {
            Some(a) => a,
            None => "".to_string(),
        },
    }
}

pub fn new_row(width: u32, order: u32) -> Row {
    let mut vec_of_cells: Vec<Cell> = Vec::new();
    for x in 0..width {
        vec_of_cells.push(new_cell(x, order, None, None, None))
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
        let rust_cell = new_cell(1, 1, Some(1), Some(2), Some(String::from("hioh!")));
        let html_cell = "<td x=1 y=1 rowspan=1 colspan=2 ><input>hioh!</input></td>".to_string();
        assert_eq!(rust_cell.to_html(), html_cell);
    }
    #[test]
    fn cell_html_to_rust_test() {
        let rust_cell = new_cell(1, 2, Some(2), Some(2), Some(String::from("hio222h!")));
        let html_cell = "<td x=1 y=2 rowspan=2 colspan=2 ><input>hio222h!</input></td>".to_string();

        assert_eq!(Cell::from(html_cell), rust_cell);
    }
}
