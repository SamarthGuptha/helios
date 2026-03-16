use windows::core::{Interface, s};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn run_ui_loop() {
    unsafe {
         let instance = GetModuleHandleA(None).unwrap();
         let window_class = s!("HeliosDashboardClass");

         let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() asu32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance.into(),
            lpszClassName: window_class,
            ..Default::default()
         };
         RegisterClassExA(&wc);
         let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Helios Dashboard"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1280,
            720,
            None,
            None,
            instance,
            None,
         ).expect("Failed to create Win32 Window");

         let factory: IDXGIFactory4 = CreateDXGIFactory2(0).unwrap();
         let adapter = factory.EnumAdapters1(0).unwrap();

         let mut device: Option<ID3D12Device> = None;
         D3D12CreateDevice(
            &adapter,
            windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0,
            &mut device,
         ).unwrap();
         let device = device.unwrap();

         let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            ..Default::default()
         };
         let command_queue: ID3D12CommandQueue = device.CreateCommandQueue(&queue_desc).unwrap();

         let swapchain_desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: 1280,
            Height: 720,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC{ Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            ..Default::default()

         };
         let _swapchain = factory.CreateSwapChainForHwnd(
            &command_queue,
            hwnd,
            &swapchain_desc,
            None,
            None,
         ).unwrap();

         let mut imgui_ctx = imgui::Context::create();
         imgui_ctx.set_ini_filename(None);

         println!("DX12: Initialization Complete. ImGui Context succesfully bound to process.");
         let mut msg = MSG::default();
         while GetMessageA(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
         }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM)-> LRESULT {
    match msg {
        WM_DESTROY => {
        PostQuitMessage(0);
        LRESULT(0)
        }
        - => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}