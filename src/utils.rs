use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Command,
};
pub struct DrawSquareAtRawCoord(pub u16, pub u16, pub Color);
impl Command for DrawSquareAtRawCoord {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let x = self.0;
        let y = self.1;
        MoveTo(x, y).write_ansi(f)?;
        DrawSquare(self.2).write_ansi(f)?;
        Ok(())
    }
}

pub struct DrawSquareAt(pub u16, pub u16, pub Color);
impl Command for DrawSquareAt {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let x = self.0;
        let y = self.1;
        MoveTo(x * 2, y).write_ansi(f)?;
        DrawSquare(self.2).write_ansi(f)?;
        Ok(())
    }
}

pub struct DrawSquare(pub Color);
impl Command for DrawSquare {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        SetBackgroundColor(self.0).write_ansi(f)?;
        Print("  ").write_ansi(f)?;
        ResetColor.write_ansi(f)?;
        Ok(())
    }
}

pub struct CreateTerminal;
impl Command for CreateTerminal {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        //terminal::enable_raw_mode().unwrap();
        EnterAlternateScreen.write_ansi(f)?;
        ResetColor.write_ansi(f)?;
        terminal::Clear(terminal::ClearType::All).write_ansi(f)?;
        Ok(())
    }
}

pub struct CloseTerminal;
impl Command for CloseTerminal {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        ResetColor.write_ansi(f)?;
        terminal::Clear(terminal::ClearType::All).write_ansi(f)?;
        LeaveAlternateScreen.write_ansi(f)?;
        terminal::disable_raw_mode().map_err(|_| std::fmt::Error)?;
        Ok(())
    }
}

pub struct ClearTerminal;
impl Command for ClearTerminal {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        ResetColor.write_ansi(f)?;
        terminal::Clear(terminal::ClearType::All).write_ansi(f)?;
        LeaveAlternateScreen.write_ansi(f)?;

        Ok(())
    }
}

pub struct PrintAt<T: std::fmt::Display>(pub u16, pub u16, pub T);
impl<T: std::fmt::Display> Command for PrintAt<T> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        MoveTo(self.0, self.1).write_ansi(f)?;
        Print(&self.2).write_ansi(f)?;
        Ok(())
    }
}
