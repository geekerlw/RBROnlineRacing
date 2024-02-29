use std::os::raw::c_void;
use egui::Context;
// use imgui::{FontConfig, FontSource};
// use imgui_dx9_renderer::Renderer;
use rbnhelper::RBNHelper;
use log::info;
use simplelog::WriteLogger;
// use windows::core::{Interface, Vtable};
use windows::core::Interface;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Foundation::{HWND, NOERROR};
use std::sync::Once;
use egui_d3d9::EguiDx9;

pub mod rbnhelper;
pub mod rbrdx9;

use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::ImguiRenderLoop;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use imgui::{Condition, Window};


pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            fmt::layer().event_format(
                fmt::format()
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true),
            ),
        )
        .with(EnvFilter::from_default_env())
        .init();
}

struct Dx11HookExample;

impl Dx11HookExample {
    fn new() -> Self {
        println!("Initializing");

        Dx11HookExample
    }
}

impl ImguiRenderLoop for Dx11HookExample {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("Hello world").size([300.0, 110.0], Condition::FirstUseEver)
        .position([200.0, 200.0], Condition::FirstUseEver)
        .no_nav()
        .resizable(false)
        .menu_bar(false)
        .title_bar(false)
        .focused(true)
        .movable(false)
        .draw_background(false)
        .build(|| {
            ui.text("Hello world!");
            ui.text("こんにちは世界！");
            ui.text("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
        });

        ui.window("leader board").size([300.0, 400.0], Condition::Always)
        .position([100.0, 100.0], Condition::Always)
        .no_nav().resizable(false).menu_bar(false).title_bar(false).draw_background(false)
        .build(|| {
            ui.set_window_font_scale(1.5);
            ui.text("1 | Lw_Ziye +00.00.00");
            ui.text("2 | Lw_Ziye +00.00.00");
            ui.text("3 | Lw_Ziye +00.00.00");
            ui.text("4 | Lw_Ziye +00.00.00");
            ui.text("5 | Lw_Ziye +00.00.00");
            ui.text("6 | Lw_Ziye +00.00.00");
            ui.text("7 | Lw_Ziye +00.00.00");
            ui.text("8 | Lw_Ziye +00.00.00");
        });

        ui.window("progress board").size([100.0, 400.0], Condition::Always)
        .position([60.0, 200.0], Condition::Always)
        .no_nav().resizable(false).menu_bar(false).title_bar(false).draw_background(false)
        .build(|| {
            let draw_list = ui.get_window_draw_list();
            draw_list.add_rect([70.0, 210.0], [100.0, 560.0], 0x7f7f7f7f).filled(true).build();
        });
    }
}


#[link(name = "RBRHacker", kind = "static")]
extern "C" {
    fn RBR_InitPlugin(arg: *mut c_void) -> *mut c_void;
    fn RBR_InitRuntime();
    fn RBR_GetD3dDevice9() -> *mut c_void;
    fn RBR_GetD3dWindow() -> *mut c_void;
    fn RBR_SetInitialize(func: extern "C" fn());
    fn RBR_SetOnBeginScene(func: extern "C" fn());
    fn RBR_SetOnEndScene(func: extern "C" fn());
    fn RBR_SetOnFrame(func: extern "C" fn());
    fn RBR_SetDrawFrontEndPage(func: extern "C" fn());
}

static mut APP: Option<EguiDx9<i32>> = None;

#[no_mangle]
extern fn rbn_init() {
    info!("call rbn plugin init");
    unsafe {
        RBR_InitRuntime();
        
        // let device = IDirect3DDevice9::from_raw(RBR_GetD3dDevice9());
        // let hwnd = HWND(RBR_GetD3dWindow() as isize);
        // APP = Some(EguiDx9::init(&device, hwnd, rbn_ui, 0, true));

        // info!("init egui app finish with: {:?}, wnd: {:?}", device, hwnd);
        // RBR_SetOnBeginScene(rbn_on_begin_frame);
        // RBR_SetOnFrame(rbn_on_end_frame);

        info!("steven: to hook after 3 seconds");
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(3));
            hudhook::Hudhook::builder()
                .with::<rbrdx9::ImguiRBRDx9Hooks>(Dx11HookExample::new())
                .build().apply().unwrap();
        });

        info!("steven: finish hook");
    }
}

fn rbn_ui(ctx: &Context, _i: &mut i32) {
    egui::Window::new("test window").fixed_pos(egui::pos2(0.0, 200.0)).fixed_size([100.0, 80.0])
    .title_bar(false)
    .show(ctx, |ui| {
        ui.label("Hello world !!!");
    });
}

extern fn rbn_on_begin_frame() {
    // unsafe {
    //     APP.as_mut().unwrap().pre_reset();
    // }
}

// extern fn rbn_on_end_frame() {
//     unsafe {
//         let pd3d = RBR_GetD3dDevice9();
//         let device = IDirect3DDevice9::from_raw_borrowed(&pd3d);
//         if let Some(device) = device {
//             APP.as_mut().unwrap().present(device);
//         }
//     }
// }


#[no_mangle]
extern "stdcall" fn DllMain(_hinst: usize, _reason: u32, _reserved: *mut ()) -> bool {
    true
}

#[no_mangle]
extern "cdecl" fn RBR_CreatePlugin(rbrgame: *mut c_void) -> *mut c_void {
    let log_file = std::env::current_dir().unwrap().join("rbnhelper.log");
    WriteLogger::init(log::LevelFilter::Info, 
        simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();

    info!("Create Plugin RBN Helper [{}] with arg: {:?}", std::env!("CARGO_PKG_VERSION"), rbrgame);

    unsafe {
        let plugin = RBR_InitPlugin(rbrgame);
        RBR_SetInitialize(rbn_init);

        return plugin;
    };
}