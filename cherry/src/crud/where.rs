use sqlx::{Database, Encode, Type};

use crate::crud::provider::WhereProvider;
use crate::statement::r#where::condition::Condition;

pub trait Where<'a, DB>: WhereProvider<'a, DB> + Sized where DB: Database {

    // wrap conditions
    fn and<F>(mut self, mut f: F) -> Self where F: FnMut(&mut Self) -> &mut Self {
        self.where_statement().make_temp();
        f(&mut self); // closure will add to temp conditions
        let conditions = self.where_statement().take_temp();
        self.where_statement().add(Condition::And(conditions));
        self
    }

    // wrap conditions
    fn or<F>(mut self, mut f: F) -> Self where F: FnMut(&mut Self) -> &mut Self {
        self.where_statement().make_temp();
        f(&mut self); // closure will add to temp conditions
        let conditions = self.where_statement().take_temp();
        self.where_statement().add(Condition::Or(conditions));
        self
    }

    fn and_eq<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.and_eq_ref(c, v);
        self
    }

    fn and_eq_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.where_statement().add(Condition::AndEq(c));
        self
    }

    fn or_eq<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.or_eq_ref(c, v);
        self
    }

    fn or_eq_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.where_statement().add(Condition::OrEq(c));
        self
    }

    // more condition methods

}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::sql::select::SelectStatement;
    use crate::sqlx::Sqlite;

    use super::*;

    #[test]
    fn test() {
        // let s = Select { _a: Default::default(), statement: SelectStatement::from("user") };
        // let id = 1;
        // let name = "the user name";
        // let v = s
        //     .and(|s| s.and_eq_ref("id", id).and_eq_ref("name", name))
        //     .or_eq("age", 15);

    }
}