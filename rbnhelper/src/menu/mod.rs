use log::info;

use crate::game::hacker::{RBR_DrawMenuText, RBR_DrawTextOverRsf};

pub mod entry;

const MENU_LINE_HEIGHT: i16 = 21;

#[derive(Default)]
pub enum EFonts {
    #[default]
    FontSmall,
    FontBig,
    FontDebug,
    FontHead,
}

#[derive(Default)]
pub enum EMenuColors {
    MenuBkground,
    MenuSelection,
    MenuIcon,
    #[default]
    MenuText,
    MenuHeading,
}

#[derive(Default)]
pub struct MenuEntry {
    text: String,
    selectable: bool,
    tips: Option<String>,
    font: Option<EFonts>,
    menu_color: Option<EMenuColors>,
    color: Option<[f32; 4]>,
    position: Option<[f32; 2]>,
}

pub trait Menu {
    fn init(&mut self);

    fn up(&mut self);

    fn down(&mut self);

    fn left(&mut self, _index: usize) {}

    fn right(&mut self, _index: usize) {}

    fn select(&mut self, _index: usize) {}

    fn draw(&self);
}

#[derive(Default)]
pub struct MenuOp {
    select_index: usize,
    entries: Vec<MenuEntry>,
}

impl MenuOp {
    fn up(&mut self) {
        if self.select_index == 0 {
            self.select_index = self.entries.len() - 1;
        }
        else {
            self.select_index -= 1;
        }
    }

    fn down(&mut self) {
        self.select_index = (self.select_index + 1) % self.entries.len();
    }

    fn left(&mut self, menu: &mut (dyn Menu + 'static)) {
        if let Some(entry) = self.entries.get_mut(self.select_index) {
            if entry.selectable {
                menu.left(self.select_index);
            }
        }
    }

    fn right(&mut self, menu: &mut (dyn Menu + 'static)) {
        if let Some(entry) = self.entries.get_mut(self.select_index) {
            if entry.selectable {
                menu.right(self.select_index);
            }
        }
    }

    fn select(&mut self, menu: &mut (dyn Menu + 'static)) {
        if let Some(entry) = self.entries.get_mut(self.select_index) {
            menu.select(self.select_index);
        }
    }

    fn draw(&self) {
        let mut x = 65i16;
        let mut y = 80i16;
        for (line, entry) in self.entries.iter().enumerate() {
            if let Some(font) = &entry.font {
                // game set font.
            }

            if let Some(color) = &entry.color {
                // game set color
            }

            if let Some(menu_color) = &entry.menu_color {
                // game set menu color
            }
            else if let Some(color) = &entry.color {
                // game set color
            }

            if let Some(pos) = &entry.position {
                x = pos[0] as i16;
                y = pos[1] as i16;
            }

            unsafe { RBR_DrawTextOverRsf(x, y, 0xFFFFFFFF, entry.text.as_ptr()) };

            y += MENU_LINE_HEIGHT;
            info!("steven: draw text in [{}, {}], {}", x, y, entry.text);
        }
    }
}