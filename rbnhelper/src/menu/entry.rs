use super::*;
use log::info;

#[derive(Default)]
pub struct EntryMenu {
    count: u32,
    op: MenuOp,
}

impl Menu for EntryMenu {
    fn init(&mut self) {
        self.op.entries.push(MenuEntry { text: "Online".to_string(), ..Default::default() });
        self.op.entries.push(MenuEntry { text: "Practice".to_string(), selectable: true, ..Default::default() });
        self.op.entries.push(MenuEntry { text: "Challege".to_string(), ..Default::default() });
        self.op.entries.push(MenuEntry { text: "About".to_string(), ..Default::default() });
    }

    fn up(&mut self) {
        self.op.up();
    }

    fn down(&mut self) {
        self.op.down();
    }

    fn left(&mut self, index: usize) {
        match index {
            _ => {
                self.count += 1;
            }
        }
    }

    fn select(&mut self, index: usize) {
        info!("steven: let's print index {} and count: {}", index, self.count);
    }

    fn draw(&self) {
        self.op.draw();
    }
}