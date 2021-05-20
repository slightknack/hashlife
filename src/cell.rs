use std::rc::Rc;
use crate::world::{Quarters, World};
use crate::macro_::Macro;

const DEAD_BASE:    [bool;4] = [false, false, false, false];

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Macro(Macro),
    Base([bool;4]),
}

impl Cell {
    pub fn new(
        tl: Rc<Cell>, tr: Rc<Cell>,
        bl: Rc<Cell>, br: Rc<Cell>,
        world: &mut World,
    ) -> Rc<Cell> {
        let quarters = Quarters::new(&tl, &tr, &bl, &br);
        if let Some(whole) = world.try_build(&quarters) {
            return Rc::clone(whole);
        }

        return match (
            tl.as_ref(), tr.as_ref(),
            bl.as_ref(), br.as_ref(),
        ) {
            (
                Cell::Macro(mtl), Cell::Macro(mtr),
                Cell::Macro(mbl), Cell::Macro(mbr),
            ) => {
                // create 5 supplemental cells
                let tm = Macro::horiz(mtl, mtr, world);
                let bm = Macro::horiz(mbl, mbr, world);
                let ml = Macro::vert(mtl, mbl, world);
                let mr = Macro::vert(mtr, mbr, world);
                let mm = Macro::center(mtl, mtr, mbl, mbr, world);

                // create 4 result cells
                let rtl = Macro::consolidate(&mtl, &tm, &ml, &mm, world);
                let rtr = Macro::consolidate(&tm, &mtr, &mm, &mr, world);
                let rbl = Macro::consolidate(&ml, &mm, &mbl, &bm, world);
                let rbr = Macro::consolidate(&mm, &mr, &bm, &mbr, world);

                let is_all_dead = tl.is_all_dead()
                    && tr.is_all_dead()
                    && bl.is_all_dead()
                    && br.is_all_dead();

                // create final result cell
                let result = Macro::consolidate(&rtl, &rtr, &rbl, &rbr, world);
                Cell::try_wrap(
                    Macro {
                        tl: Rc::clone(&tl), tr: Rc::clone(&tr),
                        bl: Rc::clone(&bl), br: Rc::clone(&br),
                        result: Cell::try_wrap(result, world),
                        size: mtl.size + 1,
                        is_dead: is_all_dead,
                    },
                    world
                )
            },
            (
                Cell::Base(btl), Cell::Base(btr),
                Cell::Base(bbl), Cell::Base(bbr),
            ) => {
                // create combined grid
                let mut grid = [[false;4];4];
                let cells = [
                    [btl, btr],
                    [bbl, bbr],
                ];

                // map smaller grids to combined grid
                for (x, row) in grid.iter_mut().enumerate() {
                    for (y, item) in row.iter_mut().enumerate() {
                        let cell = cells[if x < 2 { 0 } else { 1 }]
                                        [if y < 2 { 0 } else { 1 }]
                                        [(x % 2) * 2 + (y % 2)];
                        *item = cell;
                    }
                }

                // apply the GOL rule
                let mut result = [false;4];
                for x in 0..2 {
                    for y in 0..2 {
                        // count the neighbors
                        let alive = grid[x + 1][y + 1];
                        let mut neighbors = if alive { -1 } else { 0 };

                        for nx in 0..3 {
                            for ny in 0..3 {
                                if grid[x + nx][y + ny] { neighbors += 1; }
                            }
                        }

                        // apply the GOL rule
                        if neighbors == 3 || alive && neighbors == 2 {
                            result[(x * 2) + y] = true;
                        }
                    }
                }

                let is_all_dead = tl.is_all_dead()
                    && tr.is_all_dead()
                    && bl.is_all_dead()
                    && br.is_all_dead();

                Cell::try_wrap(
                    Macro {
                        tl: Rc::clone(&tl), tr: Rc::clone(&tr),
                        bl: Rc::clone(&bl), br: Rc::clone(&br),
                        result: Rc::new(Cell::Base(result)),
                        size: 0,
                        is_dead: is_all_dead,
                    },
                    world
                )
            },
            _ => unreachable!(
                "Cell should not contain sub-cells of different depths"
            )
        };
    }

    pub fn trim(mut cell: &Rc<Cell>) -> Rc<Cell> {
        match cell.as_ref() {
            Cell::Base(_) => cell.clone(),
            Cell::Macro(Macro { tl, tr, bl, br, .. }) => {
                let tl_dead = tl.is_all_dead();
                let tr_dead = tr.is_all_dead();
                let bl_dead = bl.is_all_dead();
                let br_dead = br.is_all_dead();

                match (tl_dead, tr_dead, bl_dead, br_dead) {
                    (false, false, false, false) => Rc::new(Cell::Base(DEAD_BASE)),
                    (true,  false, false, false) => Cell::trim(tl),
                    (false, true,  false, false) => Cell::trim(tr),
                    (false, false, true,  false) => Cell::trim(bl),
                    (false, false, false, true ) => Cell::trim(br),
                    _ => cell.clone()
                }
            }
        }
    }

    pub fn jump(cell: Rc<Cell>, step: usize, world: &mut World) -> Rc<Cell> {
        // trim the cell as small as possible
        let mut cell = Cell::trim(&cell);
        if step == 0 { return cell; }

        if let Cell::Base(_) = cell.as_ref() {
            let dead_base = Rc::new(Cell::Base(DEAD_BASE));
            cell = Cell::new(
                cell,              dead_base.clone(),
                dead_base.clone(), dead_base.clone(),
                world,
            );
        }

        // cell must be a macro-cell at this point.
        let macro_cell = match cell.as_ref() {
            Cell::Macro(m) => m,
            Cell::Base(_) => unreachable!(),
        };

        let dp = if macro_cell.size == 0 {
            Rc::new(Cell::Base(DEAD_BASE))
        } else {
            Cell::dead_of_size(macro_cell.size - 1, world)
        };

        // need to pad *more*
        // need to figure out how much
        // this increases size by 1
        // increase size by 2?
        // TODO: refactor this out into a function.

        // make a ring of padding around the trimmed macrocell
        let padded_tl = Cell::new(dp.clone(), dp.clone(), dp.clone(), macro_cell.tl.clone(), world);
        let padded_tr = Cell::new(dp.clone(), dp.clone(), macro_cell.tr.clone(), dp.clone(), world);
        let padded_bl = Cell::new(dp.clone(), macro_cell.bl.clone(), dp.clone(), dp.clone(), world);
        let padded_br = Cell::new(macro_cell.br.clone(), dp.clone(), dp.clone(), dp.clone(), world);

        // this steps
        let padded_macro = Cell::new(
            padded_tl, padded_tl,
            padded_bl, padded_br,
            world,
        );

        // TODO: pad with dead macrocells
        // if the result step of the current macrocell is too big,
        // split and rejoin
        // if the step is too large,
        // join into a bigger macrocell and step
        // then step remaining

        if step < 2usize.pow(macro_cell.size as u32) {
            todo!()
        }

        todo!()
    }

    pub fn dead_of_size(size: usize, world: &mut World) -> Rc<Cell> {
        let smaller = if size == 0 {
            Rc::new(Cell::Base(DEAD_BASE))
        } else {
            Cell::dead_of_size(size - 1, world)
        };

        Cell::new(
            smaller.clone(), smaller.clone(),
            smaller.clone(), smaller.clone(),
            world,
        )
    }

    pub fn is_all_dead(&self) -> bool {
        match self {
            Cell::Base(b) if b == &DEAD_BASE => true,
            Cell::Macro(m) => m.is_dead,
            _ => false
        }
    }

    pub fn try_wrap(m: Macro, world: &mut World) -> Rc<Cell> {
        let quarters = Quarters::new(&m.tl, &m.tr, &m.bl, &m.br);
        if let Some(whole) = world.try_build(&quarters) {
            Rc::clone(whole)
        } else {
            let whole = Rc::new(Cell::Macro(m));
            world.blueprint(quarters, Rc::clone(&whole));
            whole
        }
    }
}
