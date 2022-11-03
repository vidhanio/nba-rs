mod playercareerstats;

pub trait ResultSet {
    const ENDPOINT: &'static str;
    const HEADERS: &'static [&'static str];

    type Row;

    fn rows(&self) -> &[Self::Row];

    fn set_rows(&mut self, rows: &[Self::Row]);
}
