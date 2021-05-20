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

#[derive(Debug)]
pub struct World(HashMap<Quarters, Rc<Cell>>);

impl World {
    pub fn blank() -> World {
        World(HashMap::new())
    }

    pub fn to_grid(cell: &Rc<Cell>) -> Vec<Vec<bool>> {
        match cell.as_ref() {
            Cell::Base(n) => vec![
                vec![n[0], n[1]],
                vec![n[2], n[3]],
            ],
            Cell::Macro(m) => {
                let tl = World::to_grid(&m.tl);
                let tr = World::to_grid(&m.tr);
                let bl = World::to_grid(&m.bl);
                let br = World::to_grid(&m.br);

                let mut grid = vec![];
                for (mut l, mut r) in tl.into_iter().zip(tr.into_iter()) {
                    l.append(&mut r); grid.push(l);
                }
                for (mut l, mut r) in bl.into_iter().zip(br.into_iter()) {
                    l.append(&mut r); grid.push(l);
                }
                grid
            }
        }
    }

    pub fn to_string(cell: &Rc<Cell>) -> String {
        let grid = World::to_grid(cell);

        let mut result = "".to_string();
        for row in grid {
            for alive in row {
                result.push(if alive { '#' } else { '_' })
            }
            result.push('\n');
        }

        return result;
    }

    fn build_up(&mut self, grid: Vec<Vec<Rc<Cell>>>) -> Vec<Vec<Rc<Cell>>> {
        let mut new_grid = vec![];
        for i in (0..grid.len()).step_by(2) {
            let mut new_row = vec![];
            for j in (0..grid.len()).step_by(2) {
                new_row.push(Cell::new(
                    grid[i  ][j].clone(), grid[i  ][j+1].clone(),
                    grid[i+1][j].clone(), grid[i+1][j+1].clone(),
                    self
                ))
            }
            new_grid.push(new_row);
        }

        return new_grid;
    }

    pub fn from_grid(grid: Vec<Vec<bool>>) -> (World, Rc<Cell>) {
        // find the longest dimension
        let height = grid.len()
            .next_power_of_two(); // TODO: *2
        let width = grid.iter().fold(0, |p, n| p.max(n.len()))
            .next_power_of_two();
        let square = height.max(width);

        // build a square grid, where the side length is a power of two
        // replace missing cells with false, i.e. dead.
        let mut new_grid = vec![];
        for i in (0..(square / 2)).step_by(2) {
            let mut new_row = vec![];
            for j in (0..(square / 2)).step_by(2) {
                new_row.push(Rc::new(Cell::Base([
                    *grid.get(i  ).and_then(|r| r.get(j  )).unwrap_or(&false),
                    *grid.get(i  ).and_then(|r| r.get(j+1)).unwrap_or(&false),
                    *grid.get(i+1).and_then(|r| r.get(j  )).unwrap_or(&false),
                    *grid.get(i+1).and_then(|r| r.get(j+1)).unwrap_or(&false),
                ])));
            }
            new_grid.push(new_row);
        }

        // starting from the base macrocells, build up a quadtree
        let mut world = World::blank();
        while new_grid.len() > 1 {
            new_grid = world.build_up(new_grid)
        }

        return (world, new_grid[0][0].clone());
    }

    pub fn from_file(path: &std::path::Path) -> (World, Rc<Cell>) {
        let raw = std::fs::read_to_string(path).unwrap();

        let mut grid = vec![];
        for line in raw.lines() {
            let mut row = vec![];
            for byte in line.bytes() {
                row.push(byte == b'#')
            }
            grid.push(row);
        }

        return World::from_grid(grid);
    }

    pub fn blueprint(&mut self, quarters: Quarters, whole: Rc<Cell>) {
        self.0.insert(quarters, whole);
    }

    pub fn try_build(&self, quarters: &Quarters) -> Option<&Rc<Cell>> {
        self.0.get(quarters)
    }
}
