use std::{ffi::CString, sync::{Arc, RwLock}};
use rbrproxy::game::RBRMenu;

use crate::components::store::RacingStore;

pub mod loby;

const MENU_LINE_HEIGHT: f32 = 21.0f32;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub enum EFonts {
    FontSmall,
    #[default]
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

#[allow(dead_code)]
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
    font: EFonts,
    menu_color: EMenuColors,
    color: Option<[f32; 4]>,
    position: Option<[f32; 2]>,
    width: Option<f32>,
    left: Option<Box<dyn Fn() + Send + 'static>>,
    right: Option<Box<dyn Fn() + Send + 'static>>,
    select: Option<Box<dyn Fn() + Send + 'static>>,
}

pub trait Menu {
    fn init(&mut self, store: Arc<RwLock<RacingStore>>);

    fn up(&mut self);

    fn down(&mut self);

    fn left(&mut self);

    fn right(&mut self);

    fn select(&mut self);

    fn draw(&self);
}

#[derive(Default)]
pub struct MenuOp {
    select_index: usize,
    entries: Vec<MenuEntry>,
    rbr_menu: RBRMenu,
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

    fn left(&mut self) {
        let entry = &self.entries[self.select_index];
        if let Some(left_action) = &entry.left {
            left_action();
        }
    }

    fn right(&mut self) {
        let entry = &self.entries[self.select_index];
        if let Some(right_action) = &entry.right {
            right_action();
        }
    }

    fn select(&mut self) {
        let entry = &self.entries[self.select_index];
        if let Some(select_action) = &entry.select {
            select_action();
        }
    }

    fn draw(&self) -> f32 {
        let mut x = 72.0f32;
        let mut y = 72.0f32;
        for (line, entry) in self.entries.iter().enumerate() {
            self.rbr_menu.set_font_size(entry.font.clone().into());

            if let Some(color) = &entry.color {
                self.rbr_menu.set_color(color[0], color[1], color[2],color[3]);
            } else {
                self.rbr_menu.set_menu_color(entry.menu_color.clone().into());
            }

            if let Some(pos) = &entry.position {
                x = pos[0];
                y = pos[1];
            }
            
            if line == self.select_index {
                self.rbr_menu.draw_selection(0.0, y - 2.0, entry.width.unwrap_or(260.0), 21.0f32);
            }

            self.rbr_menu.draw_text(x, y, entry.text.as_ptr());

            y += MENU_LINE_HEIGHT;
        }
        y
    }
}