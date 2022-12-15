use sqlx::{Database, Encode, Type};

use crate::query_builder::end::EndStatement;
use crate::query_builder::end::section::EndSection;
use crate::query_builder::r#where::condition::Condition;

pub trait WhereProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn make_wrap(&mut self);

    fn take_wrap(&mut self) -> Vec<Condition<'a>>;

    fn add_statement(&mut self, c: Condition<'a>);
}

pub trait EndProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn add_section(&mut self, section: EndSection<'a>);
}