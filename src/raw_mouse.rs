use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCWSTR;

pub struct RawMouse {
    delta: Arc<Mutex<(i32, i32)>>,
}

impl RawMouse {
    pub fn new() -> Self {
        let delta = Arc::new(Mutex::new((0, 0)));
        let delta_clone = Arc::clone(&delta);

        std::thread::spawn(move || {
            unsafe {
                let class_name: Vec<u16> = "Static\0".encode_utf16().collect();
                let window_name: Vec<u16> = "RawMouseInput\0".encode_utf16().collect();
                
                let hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE(0),
                    PCWSTR(class_name.as_ptr()),
                    PCWSTR(window_name.as_ptr()),
                    WINDOW_STYLE(0),
                    0, 0, 0, 0,
                    HWND(std::ptr::null_mut()),
                    None,
                    None,
                    None,
                ).unwrap();

                let mut rid = RAWINPUTDEVICE {
                    usUsagePage: 0x01,
                    usUsage: 0x02,
                    dwFlags: RAWINPUTDEVICE_FLAGS(0),
                    hwndTarget: hwnd,
                };

                RegisterRawInputDevices(&[rid], std::mem::size_of::<RAWINPUTDEVICE>() as u32).unwrap();

                let mut msg = MSG::default();
                loop {
                    if GetMessageW(&mut msg, hwnd, 0, 0).0 > 0 {
                        if msg.message == WM_INPUT {
                            let mut size = 0u32;
                            GetRawInputData(
                                HRAWINPUT(msg.lParam.0 as *mut _),
                                RID_INPUT,
                                None,
                                &mut size,
                                std::mem::size_of::<RAWINPUTHEADER>() as u32,
                            );

                            let mut buffer = vec![0u8; size as usize];
                            GetRawInputData(
                                HRAWINPUT(msg.lParam.0 as *mut _),
                                RID_INPUT,
                                Some(buffer.as_mut_ptr() as *mut _),
                                &mut size,
                                std::mem::size_of::<RAWINPUTHEADER>() as u32,
                            );

                            let raw = &*(buffer.as_ptr() as *const RAWINPUT);
                            if raw.header.dwType == RIM_TYPEMOUSE.0 {
                                let mouse = raw.data.mouse;
                                if mouse.usFlags == MOUSE_STATE(MOUSE_MOVE_RELATIVE.0 as u16) {
                                    let mut d = delta_clone.lock().unwrap();
                                    d.0 += mouse.lLastX;
                                    d.1 += mouse.lLastY;
                                }
                            }
                        }
                    }
                }
            }
        });

        RawMouse { delta }
    }

    pub fn get_delta(&mut self) -> (i32, i32) {
        let mut d = self.delta.lock().unwrap();
        let result = *d;
        *d = (0, 0);
        result
    }
}
