use std::rc::Rc;
use crate::cell::Cell;
use crate::world::World;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macro {
    // A macrocell
    pub tl: Rc<Cell>, pub tr: Rc<Cell>,
    pub bl: Rc<Cell>, pub br: Rc<Cell>,
    // The central subcell after n timesteps
    pub result: Rc<Cell>,
    // the size of the cell.
    // 0 is made of base cells
    pub size: usize,
    // whether the cell is just dead
    pub is_dead: bool,
}

impl Macro {
    pub fn horiz(left: &Macro, right: &Macro, world: &mut World) -> Macro {
        if let Cell::Macro(joined) = Cell::new(
            Rc::clone(&left.tr), Rc::clone(&right.tl),
            Rc::clone(&left.br), Rc::clone(&right.bl),
            world,
        ).as_ref() {
            return joined.clone();
        } else {
            unreachable!(
                "Joining two horizontal Macro Cells should result in one of the same size"
            );
        }
    }

    pub fn vert(top: &Macro, bottom: &Macro, world: &mut World) -> Macro {
        if let Cell::Macro(joined) = Cell::new(
            Rc::clone(&top.bl),    Rc::clone(&top.br),
            Rc::clone(&bottom.tl), Rc::clone(&bottom.tr),
            world,
        ).as_ref() {
            return joined.clone();
        } else {
            unreachable!(
                "Joining two vertical Macro Cells should result in one of the same size"
            )
        }
    }

    pub fn center(
        tl: &Macro, tr: &Macro,
        bl: &Macro, br: &Macro,
        world: &mut World
    ) -> Macro {
        if let Cell::Macro(joined) = Cell::new(
            Rc::clone(&tl.br), Rc::clone(&tr.bl),
            Rc::clone(&bl.tr), Rc::clone(&br.tl),
            world,
        ).as_ref() {
            return joined.clone();
        } else {
            unreachable!(
                "Taking the center of four Macro Cells should result in one of the same size"
            )
        }
    }

    pub fn consolidate(
        tl: &Macro, tr: &Macro, bl: &Macro, br: &Macro, world: &mut World,
    ) -> Macro {
        if let Cell::Macro(consolidated) = Cell::new(
            Rc::clone(&tl.result), Rc::clone(&tr.result),
            Rc::clone(&bl.result), Rc::clone(&br.result),
            world,
        ).as_ref() {
            return consolidated.clone();
        } else {
            unreachable!(
                "Taking the center of four Macro Cells should result in one of the same size"
            )
        }
    }
}
