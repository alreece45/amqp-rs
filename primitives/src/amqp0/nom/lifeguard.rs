
extern crate lifeguard;

use super::ParserPool;
use lifeguard::Pool;

struct SimpleParserPool {
    list_pool: Pool<List<'static>>,
    table_pool: Pool<Table<'static>>,
    vec_pool: Pool<Vec<Val<'static>>>,
}

impl ParserPool for SimpleParserPool {
    fn new_list(&mut self, cap: usize) -> List<'static> {
        if self.list_pool.size() == 0 {
            List::with_capacity(cap)
        }
        else {
            self.list_pool.new().detach()
        }
    }

    fn new_table(&mut self, cap: usize) -> Table<'static> {
        if self.table_pool.size() == 0 {
            Table::with_capacity(cap)
        }
        else {
            self.table_pool.new().detach()
        }
    }

    fn new_vec(&mut self, cap: usize) -> Vec<Val<'static>> {
        if self.vec_pool.size() == 0 {
            Vec::with_capacity(cap)
        }
        else {
            self.vec_pool.new().detach()
        }
    }
}