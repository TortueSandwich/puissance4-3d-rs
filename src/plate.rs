use crate::utils::DrawSquareAtRawCoord;
use crate::{
    coord::Coord, coord::NumberCoord, error::P4Error, pilier::Pillar, pilier::Plane,
    player::PlayerID,
};
use core::iter::zip;
use crossterm::{cursor::MoveTo, queue, style::Color, style::Print};
use itertools::Itertools;

use std::io::{self, Write};

use crossterm::cursor::{RestorePosition, SavePosition};
use std::cell::Cell;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum TypeOfDisplay {
    Arthur,
    Victor,
}
pub enum PreviewAction {
    Add,
    Remove,
}

#[derive(Clone)]
pub struct Plate {
    data: [Pillar; 16],
    pub type_of_display: Cell<TypeOfDisplay>,
}

impl TypeOfDisplay {
    fn switch(&self) -> TypeOfDisplay {
        match self {
            TypeOfDisplay::Arthur => TypeOfDisplay::Victor,
            TypeOfDisplay::Victor => TypeOfDisplay::Arthur,
        }
    }
}

impl Plate {
    pub fn switch_view(&self) {
        log::info!("Switching view . . .");
        self.type_of_display
            .set(self.type_of_display.get().switch());
    }
    fn get_plane(&self, n: u8) -> Plane {
        let mut res = [PlayerID::Empty; 16];
        for i in 0..16 {
            let temp = self.data[i].get_player(n.into()).clone();
            res[i] = temp;
        }
        Plane { data: res }
    }
    fn get_coord_mut(&mut self, nco: impl Coord) -> &mut Pillar {
        &mut self.data[nco.to_num() as usize]
    }
    fn get_coord(&self, nco: impl Coord) -> &Pillar {
        &self.data[nco.to_num() as usize]
    }
    fn get_pion(&self, nco: impl Coord, height: usize) -> PlayerID {
        self.get_coord(nco).get_player(height).clone()
    }

    pub fn playable(&self, nco: impl Coord + Clone) -> bool {
        !self.get_coord(nco.clone()).is_full()
    }

    pub fn add_player(&mut self, nco: impl Coord + Clone, pl: PlayerID) -> Result<(), P4Error> {
        log::debug!("Adding player {:?} at {:?} [before]", pl, nco);
        let pillar = self.get_coord_mut(nco.clone());
        if pillar.is_full() {
            let nco_lc = nco.as_letter_coord();
            log::warn!("Throwing OverFilledPillar at {:?} error", nco_lc.clone());
            return Err(P4Error::OverFilledPillar(Some(nco_lc.clone())));
        }
        match pillar.add_player(pl) {
            Err(P4Error::OverFilledPillar(None)) => {
                let nco_lc = nco.as_letter_coord();
                log::warn!("Throwing OverFilledPillar at {:?} error", nco_lc.clone());
                Err(P4Error::OverFilledPillar(Some(nco_lc.clone())))
            }
            t => t,
        }
    }
}

/// DISPLAYING
impl Plate {
    const PREVIEW_COLOR: Color = Color::AnsiValue(1);
    const ORIGIN: (u16, u16) = (0, 0);
    pub fn plot(&self) {
        match self.type_of_display.get() {
            TypeOfDisplay::Arthur => self.plot_arthur().unwrap(),
            TypeOfDisplay::Victor => self.plot_victor().unwrap(),
        };
    }

    pub fn clear_plot(&self) {
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, SavePosition).unwrap();
        for y in 0..(4 * 6) {
            queue!(stdout, MoveTo(0, y), Print("  ".repeat(7))).unwrap();
        }
        crossterm::execute!(stdout, RestorePosition).unwrap();
    }

    pub fn preview(
        &self,
        coords_preview: impl Coord + Clone,
        action: PreviewAction,
    ) -> Result<(), P4Error> {
        log::trace!("Updating preview");
        let height: u16 = self.get_coord(coords_preview.clone()).get_height().clone() as u16;
        // if the pillar is filled
        if height >= 4 {
            return Err(P4Error::OverFilledPillar(Some(
                coords_preview.clone().as_letter_coord(),
            )));
        }
        let shared_height = Arc::new(height);
        let coord_process_closure: Box<dyn FnOnce(u16, u16) -> (u16, u16)> = match self
            .type_of_display
            .get()
        {
            TypeOfDisplay::Arthur => {
                let shared_height_clone = shared_height.clone();
                Box::new(move |x, y| (x as u16 * 3, y as u16 * 6 + (3 - *shared_height_clone)))
            }
            TypeOfDisplay::Victor => {
                let shared_height_clone = shared_height.clone();
                Box::new(move |x, y| (x as u16 * 2 + 2, y as u16 + 6 * (3 - *shared_height_clone)))
            } // TypeOfDisplay::Both => {
              //     let shared_height_clone = shared_height.clone();
              //     Box::new(move |x, y| (x as u16 * 2 + 2, y as u16 + 6 * (3 - *shared_height_clone)))
              // }
        };

        let (org_x, org_y) = Self::ORIGIN;
        let (x_prev, y_prev) = coords_preview.to_xy_coord();
        let (x_prev, y_prev) = coord_process_closure(x_prev as u16, y_prev as u16);
        let x: u16 = org_x /* + 1 */ + x_prev as u16; // +1 is the offset of name of line
        let y: u16 = org_y + 1 + y_prev as u16;
        let color = match action {
            PreviewAction::Add => Self::PREVIEW_COLOR,
            PreviewAction::Remove => self.get_pion(coords_preview, height as usize).color(),
        };
        queue!(
            io::stdout(),
            SavePosition,
            DrawSquareAtRawCoord(x, y, color),
            RestorePosition
        )
        .map_err(|e| P4Error::OutputInterfaceError(e))?;
        io::stdout()
            .flush()
            .map_err(|e| P4Error::OutputInterfaceError(e))?;
        
        Ok(())
    }

    ///Par niveau
    pub fn plot_victor(&self) -> Result<(), P4Error> {
        log::trace!("Drawing Plate (Victor Version)");

        let mut w = io::stdout();
        crossterm::execute!(w, SavePosition).unwrap();
        let (org_x, org_y) = Self::ORIGIN;
        let plane_y_size = 6u16;

        for height in 0..4 {
            let plane = self.get_plane(height);
            let norm_origin = (0, plane_y_size * (3 - height) as u16);
            let plate_origin: (u16, u16) = (norm_origin.0 + org_x, norm_origin.1 + org_y);
            plane.draw(plate_origin)?;
        }
        crossterm::execute!(w, RestorePosition).unwrap();
        w.flush().unwrap();
        Ok(())
    }

    ///Par pillier
    pub fn plot_arthur(&self) -> Result<(), P4Error> {
        log::trace!("Drawing plate (Arthur Version)");
        let mut w = io::stdout();
        crossterm::execute!(w, SavePosition).unwrap();
        let (org_x, org_y) = Self::ORIGIN;
        let (pillar_width, pillar_height) = (1, 6);
        let separation_width = 2;

        for (i, pillar) in self.data.iter().enumerate() {
            let pillar_name = NumberCoord::new(i as u8).as_letter_coord();
            let (i_x, i_y) = pillar_name.to_xy_coord();
            let norm_origin = (i_x * (pillar_width + separation_width), i_y * pillar_height);
            let pillar_origin: (u16, u16) = (norm_origin.0 + org_x, norm_origin.1 + org_y);
            pillar.draw(pillar_origin, pillar_name)?;
        }
        crossterm::execute!(w, RestorePosition).unwrap();
        w.flush().unwrap();

        Ok(())
    }
}

// CHECKIING WIN
impl Plate {
    const WIN_COMBINATIONS: [[usize; 4]; 6] = {
        let diag1 = [0, 1, 2, 3];
        let diag2 = [3, 2, 1, 0];
        let h0 = [0, 0, 0, 0];
        let h1 = [1, 1, 1, 1];
        let h2 = [2, 2, 2, 2];
        let h3 = [3, 3, 3, 3];
        [diag1, diag2, h0, h1, h2, h3]
    };

    fn get_line_x_of_pillar<'a>(&'a self, x: usize) -> Box<impl Iterator<Item = &Pillar>> {
        Box::new(self.data.iter().skip(4 * x).take(4))
    }
    fn get_line_y_of_pillar(&self, y: usize) -> Box<impl Iterator<Item = &Pillar>> {
        Box::new(self.data.iter().skip(y).step_by(4))
    }
    fn get_first_diagonal(&self) -> Box<impl Iterator<Item = &Pillar>> {
        Box::new(self.data.iter().skip(0).step_by(5))
    }
    fn get_second_diagonal(&self) -> Box<impl Iterator<Item = &Pillar>> {
        Box::new(self.data.iter().skip(3).step_by(3))
    }

    fn check_4_origin<'a, F>(&'a self, get_pillar_line: F) -> bool
    where
        F: Fn(usize) -> Box<dyn Iterator<Item = &'a Pillar> + 'a>,
    {
        let mut res: bool = false;
        for i in 0..4 {
            for line_win in Self::WIN_COMBINATIONS {
                let line_win_iter = line_win.into_iter();
                res |= all_elements_identical(
                    zip(get_pillar_line(i), line_win_iter).map(|(p, h)| p.get_player(h)),
                );
            }
        }
        res
    }

    fn check_diag<'a, F>(&'a self, get_pillar_diag: F) -> bool
    where
        F: Fn() -> Box<dyn Iterator<Item = &'a Pillar> + 'a>,
    {
        let mut res: bool = false;
        for line_win in Self::WIN_COMBINATIONS {
            let line_win_iter = line_win.into_iter();
            res |= all_elements_identical(
                zip(get_pillar_diag(), line_win_iter).map(|(p, h)| p.get_player(h)),
            );
        }
        res
    }

    pub fn check_win(&self) -> bool {
        //log::debug!("{}", self.data.iter().any(|p| p.is_full()));
        //log::debug!("{:?}", all_elements_identical((0..4).map(|i| plate[i].get_player(0))));

        let plate = &self.data;
        if plate
            .iter()
            .any(|p| all_elements_identical(p.data.into_iter()))
        {
            return true;
        }

        if self.check_4_origin(|i| self.get_line_x_of_pillar(i))
            || self.check_4_origin(|i| self.get_line_y_of_pillar(i))
            || self.check_diag(|| self.get_first_diagonal())
            || self.check_diag(|| self.get_second_diagonal())
        {
            return true;
        }

        return false;
    }
}

impl std::fmt::Debug for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.type_of_display)?;
        for (i, p) in self.data.into_iter().enumerate() {
            if i % 4 == 0 && i != 0 {
                writeln!(f)?;
            }
            let coords = NumberCoord::new(i as u8).as_letter_coord();
            write!(f, "{:?} : {},", coords, p)?;
        }
        Ok(())
    }
}

impl Default for Plate {
    fn default() -> Self {
        log::trace!("Empty plate created");
        Plate {
            data: [Pillar::default(); 16],
            type_of_display: TypeOfDisplay::Victor.into(),
        }
    }
}

fn all_elements_identical<'a, I>(iter: I) -> bool
where
    I: Iterator<Item = PlayerID>,
{
    iter.tuple_windows::<(PlayerID, PlayerID)>()
        .map(|(one, two)| one == two && one != PlayerID::Empty)
        .all(|e| e)
}
