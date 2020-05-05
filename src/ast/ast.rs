#[derive(Debug)]
pub enum Cell {
    Number(i64),
    Symbol(String),
    FuncCall(String, Vec<Cell>),
    List(Vec<Cell>),
    BuiltIn(fn(Vec<Cell>) -> Cell)
}