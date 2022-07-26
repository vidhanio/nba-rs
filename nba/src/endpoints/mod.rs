mod playercareerstats;

pub trait ResultSet {
    const HEADERS: &'static [&'static str];

    type Row;

    fn row_set(&self) -> &[Self::Row];

    fn get_row(&self, index: usize) -> Option<&Self::Row> {
        self.row_set().get(index)
    }
}
