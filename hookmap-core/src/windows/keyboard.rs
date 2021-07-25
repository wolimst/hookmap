use super::DW_EXTRA_INFO;
use crate::common::{
    event::EventBlock,
    handler::{InputHandler, INPUT_HANDLER},
    keyboard::{EmulateKeyboardInput, InstallKeyboardHook, Key, KeyboardAction, KeyboardEvent},
};
use once_cell::sync::Lazy;
use std::{
    mem,
    sync::atomic::{AtomicPtr, Ordering},
    thread,
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::winuser::{
        self, INPUT, INPUT_KEYBOARD, KBDLLHOOKSTRUCT, KEYBDINPUT, KEYEVENTF_KEYUP, WH_KEYBOARD_LL,
    },
};

mod conversion;
use conversion::KeyCode;

static HHOOK_HANDLER: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

extern "system" fn hook_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event_info = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };
    if event_info.dwExtraInfo & DW_EXTRA_INFO != 0 {
        return call_next_hook(code, w_param, l_param);
    }
    let target = KeyCode(event_info.vkCode).into();
    let action = match event_info.flags >> 7 {
        0 => KeyboardAction::Press,
        _ => KeyboardAction::Release,
    };
    let event = KeyboardEvent::new(target, action);
    match INPUT_HANDLER.keyboard.lock().unwrap().emit(event) {
        EventBlock::Block => {
            set_keyboard_state(target, action);
            1
        }
        EventBlock::Unblock => call_next_hook(code, w_param, l_param),
    }
}

fn call_next_hook(n_code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        winuser::CallNextHookEx(
            HHOOK_HANDLER.load(Ordering::SeqCst),
            n_code,
            w_param,
            l_param,
        )
    }
}

fn set_keyboard_state(target: Key, action: KeyboardAction) {
    let mut buffer = [0; 256];
    let vk_code = <Key as Into<KeyCode>>::into(target).0 as usize;
    unsafe {
        winuser::GetKeyboardState(&buffer as *const _ as *mut _);
        match action {
            KeyboardAction::Press => buffer[vk_code] |= 1 << 7,
            KeyboardAction::Release => buffer[vk_code] &= !0u8 >> 1,
        }
        winuser::SetKeyboardState(&buffer as *const _ as *mut _);
        winuser::GetKeyboardState(&buffer as *const _ as *mut _);
    };
}

impl InstallKeyboardHook for InputHandler {
    fn install() {
        let handler = unsafe {
            winuser::SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), 0 as HINSTANCE, 0)
        };
        HHOOK_HANDLER.store(handler, Ordering::SeqCst);
    }
}

impl EmulateKeyboardInput for Key {
    fn press(&self) {
        send_key_input(self, 0);
    }
    fn release(&self) {
        send_key_input(self, KEYEVENTF_KEYUP);
    }

    fn is_pressed(&self) -> bool {
        get_key_state(self) & (1 << 15) != 0
    }
    fn is_toggled(&self) -> bool {
        get_key_state(self) & 1 != 0
    }
}

fn send_key_input(key: &Key, flags: u32) {
    let keybd_input = KEYBDINPUT {
        wVk: <Key as Into<KeyCode>>::into(*key).0 as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: DW_EXTRA_INFO,
    };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::transmute_copy(&keybd_input) },
    };

    thread::spawn(move || unsafe {
        winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);
    });
}

fn get_key_state(key: &Key) -> i16 {
    let key_code: KeyCode = (*key).into();
    unsafe { winuser::GetKeyState(key_code.0 as i32) as i16 }
}
