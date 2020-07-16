use std::rc::Rc;
use crate::world::{Quarters, World};
use crate::macro_::Macro;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Macro(Macro),
    Base([bool;16]),
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

                // create final result cell
                let result = Macro::consolidate(&rtl, &rtr, &rbl, &rbr, world);
                Cell::try_wrap(
                    Macro {
                        tl: Rc::clone(&tl), tr: Rc::clone(&tr),
                        bl: Rc::clone(&bl), br: Rc::clone(&br),
                        result: Cell::try_wrap(result, world),
                    },
                    world
                )
            },
            (
                Cell::Base(btl), Cell::Base(btr),
                Cell::Base(bbl), Cell::Base(bbr),
            ) => {
                // create combined grid
                let mut grid = [[false;8];8];
                let cells = [
                    [btl, btr],
                    [bbl, bbr],
                ];

                // map smaller grids to combined grid
                for (x, row) in grid.iter_mut().enumerate() {
                    for (y, item) in row.iter_mut().enumerate() {
                        let cell = cells[if x < 4 { 0 } else { 1 }]
                                        [if y < 4 { 0 } else { 1 }]
                                        [(x % 4) * 4 + (y % 4)];
                        *item = cell;
                    }
                }

                // build the first step
                let mut step_1 = [[false;6];6];
                for x in 1..7 {
                    for y in 1..7 {
                        // calculate alive and neighbors
                        let alive = grid[x][y];
                        let mut neighbors = 0;
                        for nx in 0..3 { for ny in 0..3 {
                            if grid[x + nx - 1][y + ny - 1] { neighbors += 1; }
                        }}

                        // apply the GOL rule
                        if neighbors == 3 || alive && neighbors == 2 {
                            step_1[x-1][y-1] = true;
                        }
                    }
                }

                // build the second step
                let mut step_2 = [[false;4];4];
                for x in 1..5 {
                    for y in 1..5 {
                        let alive = step_1[x][y];
                        let mut neighbors = 0;
                        for nx in 0..3 { for ny in 0..3 {
                            if step_2[x + nx - 1][y + ny - 1] { neighbors += 1; }
                        }}

                        // apply the GOL rule
                        if neighbors == 3 || alive && neighbors == 2 {
                            step_2[x-1][y-1] = true;
                        }
                    }
                }

                // flatten the second step
                let mut result = [false;16];
                for x in 0..4 { for y in 0..4 {
                    result[x * 4 + y] = step_2[x][y];
                }}

                Cell::try_wrap(
                    Macro {
                        tl: Rc::clone(&tl), tr: Rc::clone(&tr),
                        bl: Rc::clone(&bl), br: Rc::clone(&br),
                        result: Rc::new(Cell::Base(result)),
                    },
                    world
                )
            },
            _ => unreachable!(
                "Cell should not contain sub-cells of different depths"
            )
        };
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
