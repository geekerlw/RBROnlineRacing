use super::*;

#[derive(Default)]
pub struct LobyMenu {
    headstr: CString,
    roomtitle: CString,
    maptitle: CString,
    copyright: CString,
    op: MenuOp,
    rbr_menu: RBRMenu,
}

impl Menu for LobyMenu {
    fn init(&mut self) {
        self.headstr = CString::new("RBR LIVE BATTLE").expect("failed");
        self.roomtitle = CString::new("Joined Players:").expect("failed");
        self.maptitle = CString::new("Next Stage:").expect("failed");
        self.copyright = CString::new(format!("Copyright (c) 2023-2025 Lw_Ziye, Plugin Version: {}", std::env!("CARGO_PKG_VERSION"))).expect("failed");

        self.op.entries.push(MenuEntry { 
            text: CString::new("Join Race").expect("failed"),
            ..Default::default()
        });
        self.op.entries.push(MenuEntry { 
            text: CString::new("Select Car").expect("failed"),
            ..Default::default()
        });
        self.op.entries.push(MenuEntry { 
            text: CString::new("Car Tyre").expect("failed"),
            ..Default::default()
        });
        self.op.entries.push(MenuEntry { 
            text: CString::new("Car Setup").expect("failed"),
            ..Default::default() 
        });
        self.op.entries.push(MenuEntry { 
            text: CString::new("Exit Race").expect("failed"),
            ..Default::default()
        });
    }

    fn up(&mut self) {
        self.op.up();
    }

    fn down(&mut self) {
        self.op.down();
    }

    fn left(&mut self) {

    }

    fn right(&mut self) {
        
    }

    fn select(&mut self) {
        
    }

    fn draw(&self) {
        self.rbr_menu.draw_blackout(0.0, 600.0, 800.0, 0.0);
        self.rbr_menu.draw_selection(260.0, 49.0, 2.0, 400.0);
        self.rbr_menu.draw_selection(638.0, 49.0, 2.0, 400.0);
        self.rbr_menu.draw_selection(260.0, 210.0, 530.0, 2.0);

        self.rbr_menu.set_font_size(EFonts::FontBig.into());
        self.rbr_menu.set_menu_color(EMenuColors::MenuHeading.into());
        self.rbr_menu.draw_text(72.0, 49.0, self.headstr.as_ptr());
        self.rbr_menu.draw_text(270.0, 49.0, self.maptitle.as_ptr());

        // TODO: draw map and car image
        self.rbr_menu.draw_flatbox(270.0, 72.0, 160.0, 120.0);
        self.rbr_menu.draw_flatbox(432.0, 72.0, 36.0, 36.0);
        self.rbr_menu.draw_flatbox(432.0, 156.0, 36.0, 36.0);
        self.rbr_menu.draw_flatbox(470.0, 72.0, 160.0, 120.0);

        let mut y = self.op.draw() + 10.0;
        self.rbr_menu.draw_selection(0.0, y - 4.0, 260.0, 2.0);
        self.rbr_menu.set_menu_color(EMenuColors::MenuText.into());
        self.rbr_menu.draw_text(52.0, y, self.roomtitle.as_ptr());
        y += MENU_LINE_HEIGHT;
        self.rbr_menu.set_font_size(EFonts::FontSmall.into());
        for i in 0..8 {
            let rank = CString::new((i + 1).to_string()).expect("failed");
            self.rbr_menu.draw_text(72.0, y, rank.as_ptr());
            let name = CString::new("Lw_Ziye").expect("failed");
            self.rbr_menu.draw_text(90.0, y, name.as_ptr());
            let state = CString::new("Ready").expect("msg");
            self.rbr_menu.draw_text(200.0, y, state.as_ptr());
            y += MENU_LINE_HEIGHT;
        }

        self.rbr_menu.set_font_size(EFonts::FontSmall.into());
        self.rbr_menu.set_menu_color(EMenuColors::MenuText.into());
        self.rbr_menu.draw_text(72.0, 460.0, self.copyright.as_ptr());
    }
}