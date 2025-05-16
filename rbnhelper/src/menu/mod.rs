use std::ffi::CString;
use rbrproxy::RBRProxy;

pub mod loby;

const MENU_LINE_HEIGHT: f32 = 21.0f32;

#[derive(Default, Clone)]
pub enum EFonts {
    #[default]
    FontSmall,
    FontBig,
    FontDebug,
    FontHead,
}

impl Into<i32> for EFonts {
    fn into(self) -> i32 {
        match self {
            EFonts::FontSmall => 0,
            EFonts::FontBig => 1,
            EFonts::FontDebug => 2,
            EFonts::FontHead => 3,
        }
    }
}

#[derive(Default, Clone)]
pub enum EMenuColors {
    MenuBkground,
    MenuSelection,
    MenuIcon,
    #[default]
    MenuText,
    MenuHeading,
}

impl Into<i32> for EMenuColors {
    fn into(self) -> i32 {
        match self {
            EMenuColors::MenuBkground => 0,
            EMenuColors::MenuSelection => 1,
            EMenuColors::MenuIcon => 2,
            EMenuColors::MenuText => 3,
            EMenuColors::MenuHeading => 4,
        }
    }
}

#[derive(Default)]
pub struct MenuEntry {
    text: CString,
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
    rbrproxy: RBRProxy,
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
        menu.select(self.select_index);
    }

    fn draw(&self) {
        let mut x = 65.0f32;
        let mut y = 80.0f32;
        for (line, entry) in self.entries.iter().enumerate() {
            if let Some(font) = &entry.font {
                self.rbrproxy.set_font_size(font.clone().into());
            }

            if let Some(menu_color) = &entry.menu_color {
                self.rbrproxy.set_menu_color(menu_color.clone().into());
            }
            else if let Some(color) = &entry.color {
                self.rbrproxy.set_color(color[0], color[1], color[2],color[3]);
            }

            if let Some(pos) = &entry.position {
                x = pos[0];
                y = pos[1];
            }
            
            if line == self.select_index {
                self.rbrproxy.draw_selection(x, y - 10.0, 200f32);
            }

            self.rbrproxy.draw_text(x, y, entry.text.as_ptr());

            y += MENU_LINE_HEIGHT;
        }
    }
}