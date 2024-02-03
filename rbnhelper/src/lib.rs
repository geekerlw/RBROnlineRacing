use std::os::raw::c_void;
use egui::Context;
use rbnhelper::RBNHelper;
use log::info;
use simplelog::WriteLogger;
use windows::core::Interface;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use windows::Win32::Foundation::{HWND, NOERROR};
use std::sync::Once;
use egui_d3d9::EguiDx9;

pub mod rbnhelper;

#[link(name = "RBRHacker", kind = "static")]
extern "C" {
    fn RBR_InitPlugin(arg: *mut c_void) -> *mut c_void;
    fn RBR_InitRuntime();
    fn RBR_GetD3dDevice9() -> *mut c_void;
    fn RBR_GetD3dWindow() -> *mut c_void;
    fn RBR_SetInitialize(func: extern "C" fn());
    fn RBR_SetOnBeginScene(func: extern "C" fn());
    fn RBR_SetOnEndScene(func: extern "C" fn());
}

static mut APP: Option<EguiDx9<i32>> = None;

#[no_mangle]
extern fn rbn_init() {
    info!("call rbn plugin init");
    unsafe {
        RBR_InitRuntime();
        
        let device = IDirect3DDevice9::from_raw(RBR_GetD3dDevice9());
        let hwnd = HWND(RBR_GetD3dWindow() as isize);
        APP = Some(EguiDx9::init(&device, hwnd, rbn_ui, 0, true));

        info!("init egui app finish with: {:?}, wnd: {:?}", device, hwnd);
        RBR_SetOnBeginScene(rbn_on_begin_frame);
        RBR_SetOnEndScene(rbn_on_end_frame);
    }
}

fn rbn_ui(ctx: &Context, _i: &mut i32) {
    egui::Window::new("test window").fixed_pos(egui::pos2(350.0, 200.0)).fixed_size([200.0, 80.0])
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

extern fn rbn_on_end_frame() {
    unsafe {
        let pd3d = RBR_GetD3dDevice9();
        let device = IDirect3DDevice9::from_raw_borrowed(&pd3d);
        if let Some(device) = device {
            APP.as_mut().unwrap().present(device);
        }
    }
}

// extern fn rbn_on_end_frame() {
//     let mut imgui = imgui::Context::create();
//     imgui.set_ini_filename(None);
//     let io = imgui.io_mut();
//     io.display_size = [300.0, 200.0];

//     let font_config = imgui::FontConfig::default();
    


//     let ui = imgui.frame();
//     ui.window("hello world")
//     .size([300.0, 200.0], imgui::Condition::Always)
//     .build(|| {
//         ui.text("Hello world!");
//         ui.text("This is showed by imgui-rs");
//     });
//     ui.show_demo_window(&mut true);

//     unsafe {
//         let mut renderer = Renderer::new_raw(&mut imgui, RBR_GetD3dDevice9()).unwrap();
//         renderer.render(imgui.render()).unwrap();
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