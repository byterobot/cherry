use crate::arguments::Arguments;
use crate::row::Row;

pub trait Cherry: Sized + Send + Unpin {

    fn table() -> &'static str;

    fn columns() -> Vec<&'static str>;

    fn arguments<'a>(&'a self, arguments: &mut Arguments<'a>);
    
    fn from_row(row: &Row) -> crate::Result<Self>;

}
