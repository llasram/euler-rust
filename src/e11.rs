use std::io::File;
use std::ops::Index;
use std::ops::Add;

#[derive(Copy)]
#[derive(Show)]
pub struct GridPos(usize, usize);

#[derive(Show)]
pub struct Grid {
    pub ncols: usize,
    pub nrows: usize,
    data: Vec<usize>,
}

#[allow(unstable)]
impl Grid {
    pub fn load(ncols: usize, nrows: usize, path: &Path) -> Grid {
        let data: Vec<usize> =
            File::open(path).
            read_to_string().unwrap().words().
            map(|w: &str| w.parse::<usize>().unwrap()).
            collect();
        Grid { ncols: ncols, nrows: nrows, data: data }
    }

    pub fn is_valid_pos(&self, pos: GridPos) -> bool {
        let GridPos(row, col) = pos;
        (row < self.nrows && col < self.ncols)
    }

    pub fn fold_segment<F>(
        &self, start: GridPos, step: GridPos, count: usize,
        init: usize, f: F) -> Option<usize>
        where F: Fn(usize, usize) -> usize,
    {
        let mut pos = start;
        let mut acc = init;
        for _ in (0..count) {
            if !self.is_valid_pos(pos) { return None; }
            acc = f(acc, self[pos]);
            pos = pos + step;
        }
        return Some(acc)
    }
}

#[allow(unstable)]
impl Index<usize> for Grid {
    type Output = usize;

    fn index<'a>(&'a self, index: &usize) -> &'a usize {
        &self.data[*index]
    }
}

#[allow(unstable)]
impl Index<GridPos> for Grid {
    type Output = usize;

    fn index<'a>(&'a self, pos: &GridPos) -> &'a usize {
        let &GridPos(row, col) = pos;
        &self.data[(row * self.ncols) + col]
    }
}

impl Add for GridPos {
    type Output = GridPos;

    fn add(self, rhs: GridPos) -> GridPos {
        let GridPos(row1, col1) = self;
        let GridPos(row2, col2) = rhs;
        GridPos(row1 + row2, col1 + col2)
    }
}


#[allow(dead_code)]
#[allow(unstable)]
pub fn e11(n: usize) -> usize {
    let path = Path::new("data/e11.txt");
    let grid = Grid::load(20, 20, &path);
    let steps = [GridPos(0, 1), GridPos(1, 0),
                 GridPos(1, 1), GridPos(-1, 1)];
    (0..grid.nrows).flat_map(|r| {
        (0..grid.ncols).map(move |c| GridPos(r, c))
    }).flat_map(|start| {
        steps.iter().map(move |step| (start, *step))
    }).filter_map(|(start, step)| {
        grid.fold_segment(start, step, n, 1, |a, x| a * x)
    }).max().unwrap()
}
