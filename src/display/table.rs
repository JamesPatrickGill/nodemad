use console::measure_text_width;

#[derive(Debug)]
pub struct Table {
    headings: Vec<String>,
    rows: Vec<Vec<String>>,
    col_widths: Vec<usize>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Table {
        let mut col_widths: Vec<usize> = Vec::new();
        for heading in headers.iter() {
            col_widths.push(measure_text_width(heading));
        }
        Table {
            headings: headers,
            rows: Vec::new(),
            col_widths,
        }
    }
}

impl Table {
    pub fn print_table(&self) {
        let mut table: String = String::new();
        table = format!(
            "{}{}\n",
            table,
            construct_headers(&self.col_widths, &self.headings),
        );
        for row in &self.rows {
            table = format!("{}{}\n", table, construct_row(&self.col_widths, row));
        }

        table = format!("{}└", table);
        for (idx, width) in self.col_widths.iter().enumerate() {
            table = format!("{}{}", table, "─".repeat(width + 4));

            if idx != self.col_widths.len() - 1 {
                table = format!("{}{}", table, "┴");
            }
        }
        table = format!("{}┘", table);
        println!("{}", table);
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        row.iter().enumerate().for_each(|(idx, item)| {
            let item_length = measure_text_width(item);
            if item_length > self.col_widths[idx] {
                self.col_widths[idx] = item_length;
            }
        });
        self.rows.push(row);
    }
}

fn construct_headers(widths: &Vec<usize>, headings: &Vec<String>) -> String {
    let mut headers: String = String::from("┌");

    // Top line
    for (index, width) in widths.iter().enumerate() {
        headers = format!("{}{}", headers, "─".repeat(width + 4));

        if index != widths.len() - 1 {
            headers = format!("{}{}", headers, "┬");
        }
    }

    headers = format!("{}{}", headers, "┐\n");

    // Row
    headers = format!("{}{}\n", headers, construct_row(widths, headings));

    headers = format!("{}╞", headers);
    // Header divider
    for (idx, width) in widths.iter().enumerate() {
        headers = format!("{}{}", headers, "═".repeat(width + 4));

        if idx != widths.len() - 1 {
            headers = format!("{}{}", headers, "╪");
        }
    }

    format!("{}{}", headers, "╡")
}

fn construct_row(widths: &Vec<usize>, row: &Vec<String>) -> String {
    let mut row_string: String = String::from("│");
    for (index, item) in row.iter().enumerate() {
        let item_length = measure_text_width(item);
        row_string = format!(
            "{}  {}{}  ",
            row_string,
            item,
            " ".repeat(widths[index] - item_length)
        );

        if index != row.len() - 1 {
            row_string = format!("{}{}", row_string, "│");
        }
    }

    format!("{}{}", row_string, "|")
}
