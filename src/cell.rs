use std::rc::Rc;
use crate::world::{Quarters, World};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    pub tl: Rc<Cell>,
    pub tr: Rc<Cell>,
    pub bl: Rc<Cell>,
    pub br: Rc<Cell>,
    pub result: Rc<Cell>,
}

impl Cell {
    pub fn new(
        tl: Rc<Cell>,
        tr: Rc<Cell>,
        bl: Rc<Cell>,
        br: Rc<Cell>,
        world: &mut World,
    ) -> Rc<Cell> {
        let quarters = Quarters::new(&tl, &tr, &bl, &br);
        if let Some(whole) = world.try_build(&quarters) {
            return Rc::clone(whole);
        }

        // create 5 supplemental cells
        let tm = Cell::horiz(&tl, &tr, world);
        let bm = Cell::horiz(&bl, &br, world);
        let ml = Cell::vert(&tl, &bl, world);
        let mr = Cell::vert(&tr, &br, world);
        let mm = Cell::vert(&tm, &bm, world);

        // create 4 result cells
        let rtl = Cell::consolidate(&tl, &tm, &ml, &mm, world);
        let rtr = Cell::consolidate(&tm, &tr, &mm, &mr, world);
        let rbl = Cell::consolidate(&tl, &tm, &ml, &mm, world);
        let rbr = Cell::consolidate(&tl, &tm, &ml, &mm, world);

        // create final result cell
        let result = Cell::consolidate(&rtl, &rtr, &rbl, &rbr, world);
        let cell   = Rc::new(Cell { tl, tr, bl, br, result });

        // cache construction & return
        world.blueprint(quarters, Rc::clone(&cell));
        return cell;
    }

    pub fn horiz(left: &Rc<Cell>, right: &Rc<Cell>, world: &mut World) -> Rc<Cell> {
        Cell::new(
            Rc::clone(&left.tr),
            Rc::clone(&right.tl),
            Rc::clone(&left.br),
            Rc::clone(&right.bl),
            world,
        )
    }

    pub fn vert(top: &Rc<Cell>, bottom: &Rc<Cell>, world: &mut World) -> Rc<Cell> {
        Cell::new(
            Rc::clone(&top.bl),
            Rc::clone(&top.br),
            Rc::clone(&bottom.tl),
            Rc::clone(&bottom.tr),
            world,
        )
    }

    pub fn consolidate(
        tl: &Rc<Cell>,
        tr: &Rc<Cell>,
        bl: &Rc<Cell>,
        br: &Rc<Cell>,
        world: &mut World,
    ) -> Rc<Cell> {
        // need to check world
        Cell::new(
            Rc::clone(&tl.result),
            Rc::clone(&tr.result),
            Rc::clone(&bl.result),
            Rc::clone(&br.result),
            world,
        )
    }
}
