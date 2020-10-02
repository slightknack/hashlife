use std::rc::Rc;
use std::collections::HashMap;
use std::convert::From;
use crate::cell::Cell;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Quarters((Rc<Cell>, Rc<Cell>, Rc<Cell>, Rc<Cell>));

impl Quarters {
    pub fn new(
        tl: &Rc<Cell>,
        tr: &Rc<Cell>,
        bl: &Rc<Cell>,
        br: &Rc<Cell>,
    ) -> Quarters {
        Quarters((
            Rc::clone(tl),
            Rc::clone(tr),
            Rc::clone(bl),
            Rc::clone(br),
        ))
    }
}

// impl From<Rc<Cell>> for Quarters {
//     fn from(cell: Rc<Cell>) -> Quarters {
//         Quarters((cell.tl, cell.tr, cell.bl, cell.br))
//     }
// }

pub struct World(HashMap<Quarters, Rc<Cell>>);

impl World {
    pub fn blank() -> World {
        World(HashMap::new())
    }

    pub fn blueprint(&mut self, quarters: Quarters, whole: Rc<Cell>) {
        self.0.insert(quarters, whole);
    }

    pub fn try_build(&self, quarters: &Quarters) -> Option<&Rc<Cell>> {
        self.0.get(quarters)
    }
}
