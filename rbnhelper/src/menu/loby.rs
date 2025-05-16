use super::*;
use log::info;

#[derive(Default)]
pub struct LobyMenu {
    count: u32,
    op: MenuOp,
}

impl Menu for LobyMenu {
    fn init(&mut self) {
        self.op.entries.push(MenuEntry { text: CString::new("Online").expect("failed"), ..Default::default() });
        self.op.entries.push(MenuEntry { text: CString::new("Practice").expect("failed"), selectable: true, ..Default::default() });
        self.op.entries.push(MenuEntry { text: CString::new("Challege").expect("failed"), ..Default::default() });
        self.op.entries.push(MenuEntry { text: CString::new("About").expect("failed"), ..Default::default() });
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