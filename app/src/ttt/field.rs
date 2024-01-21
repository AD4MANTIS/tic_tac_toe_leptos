use leptos::*;

use super::symbol::PlayerSymbol;

#[derive(Clone, Copy)]
pub struct Fields(pub [RwSignal<Option<PlayerSymbol>>; 9]);

impl Default for Fields {
    fn default() -> Self {
        Self(
            std::iter::repeat(None::<PlayerSymbol>)
                .take(9)
                .map(create_rw_signal)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Must have 9 Fields"),
        )
    }
}

const WIDTH: usize = 3;
const HEIGHT: usize = 3;

macro_rules! line {
    ($f: expr, $($index: expr),+) => {
        [$($f.0[$index]()),+]
    };
}

impl Fields {
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Option<PlayerSymbol>> + '_> {
        (0..HEIGHT).map(|row_id| self.row(row_id))
    }

    pub fn row(&self, row_id: usize) -> impl Iterator<Item = Option<PlayerSymbol>> + '_ {
        self.0
            .iter()
            .skip(row_id * WIDTH)
            .take(WIDTH)
            .map(|field| field())
    }

    pub fn cols(&self) -> [[Option<PlayerSymbol>; HEIGHT]; WIDTH] {
        (0..WIDTH)
            .map(|col_id| self.col(col_id))
            .collect::<Vec<_>>()
            .try_into()
            .expect("")
    }

    pub fn col(&self, col_id: usize) -> [Option<PlayerSymbol>; HEIGHT] {
        line!(self, col_id, col_id + WIDTH, col_id + (WIDTH * 2))
    }

    pub fn diags(&self) -> [[Option<PlayerSymbol>; 3]; 2] {
        [line!(self, 0, 4, 8), line!(self, 2, 4, 6)]
    }

    pub fn all_win_lines(&self) -> Vec<Vec<Option<PlayerSymbol>>> {
        let mut all = self.rows().map(|x| x.collect()).collect::<Vec<Vec<_>>>();

        all.extend(self.cols().map(|x| x.to_vec()));
        all.extend(self.diags().map(|x| x.to_vec()));

        all
    }
}
