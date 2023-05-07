use device_query::{DeviceState, Keycode::*};

pub(crate) fn write_char(reg: u32) {
    if let Some(c) = char::from_u32(reg) {
        print!("{c}");
    }
}

pub(crate) fn input_a() -> u32 {
    let keys = DeviceState::new().query_keymap();
    let mut result = 0u32;
    for k in keys {
        match k {
            Space => result |= 1 << 0,
            A => result |= 1 << 1,
            B => result |= 1 << 2,
            C => result |= 1 << 3,
            D => result |= 1 << 4,
            E => result |= 1 << 5,
            F => result |= 1 << 6,
            G => result |= 1 << 7,
            H => result |= 1 << 8,
            I => result |= 1 << 9,
            J => result |= 1 << 10,
            K => result |= 1 << 11,
            L => result |= 1 << 12,
            M => result |= 1 << 13,
            N => result |= 1 << 14,
            O => result |= 1 << 15,
            P => result |= 1 << 16,
            Q => result |= 1 << 17,
            R => result |= 1 << 18,
            S => result |= 1 << 19,
            T => result |= 1 << 20,
            U => result |= 1 << 21,
            V => result |= 1 << 22,
            W => result |= 1 << 23,
            X => result |= 1 << 24,
            Y => result |= 1 << 25,
            Z => result |= 1 << 26,
            Enter => result |= 1 << 27,
            Escape => result |= 1 << 28,
            Backspace => result |= 1 << 29,
            Delete => result |= 1 << 30,
            Tab => result |= 1 << 31,
            _ => (),
        }
    }
    result
}

pub(crate) fn input_b() -> u32 {
    let keys = DeviceState::new().query_keymap();
    let mut result = 0u32;
    for k in keys {
        match k {
            Key0 => result |= 1 << 0,
            Key1 => result |= 1 << 1,
            Key2 => result |= 1 << 2,
            Key3 => result |= 1 << 3,
            Key4 => result |= 1 << 4,
            Key5 => result |= 1 << 5,
            Key6 => result |= 1 << 6,
            Key7 => result |= 1 << 7,
            Key8 => result |= 1 << 8,
            Key9 => result |= 1 << 9,
            Up => result |= 1 << 10,
            Down => result |= 1 << 11,
            Left => result |= 1 << 12,
            Right => result |= 1 << 13,
            PageUp => result |= 1 << 14,
            PageDown => result |= 1 << 15,
            Numpad0 => result |= 1 << 16,
            Numpad1 => result |= 1 << 17,
            Numpad2 => result |= 1 << 18,
            Numpad3 => result |= 1 << 19,
            Numpad4 => result |= 1 << 20,
            Numpad5 => result |= 1 << 21,
            Numpad6 => result |= 1 << 22,
            Numpad7 => result |= 1 << 23,
            Numpad8 => result |= 1 << 24,
            Numpad9 => result |= 1 << 25,
            LShift => result |= 1 << 26,
            RShift => result |= 1 << 27,
            LControl => result |= 1 << 28,
            RControl => result |= 1 << 29,
            LAlt => result |= 1 << 30,
            RAlt => result |= 1 << 31,
            _ => (),
        }
    }
    result
}

pub(crate) fn input_c() -> u32 {
    let keys = DeviceState::new().query_keymap();
    let mut result = 0u32;
    for k in keys {
        match k {
            CapsLock => result |= 1 << 0,
            F1 => result |= 1 << 1,
            F2 => result |= 1 << 2,
            F3 => result |= 1 << 3,
            F4 => result |= 1 << 4,
            F5 => result |= 1 << 5,
            F6 => result |= 1 << 6,
            F7 => result |= 1 << 7,
            F8 => result |= 1 << 8,
            F9 => result |= 1 << 9,
            F10 => result |= 1 << 10,
            F11 => result |= 1 << 11,
            F12 => result |= 1 << 12,
            Meta => result |= 1 << 13,
            Home => result |= 1 << 14,
            End => result |= 1 << 15,
            Insert => result |= 1 << 16,
            Grave => result |= 1 << 17,
            Minus => result |= 1 << 18,
            Equal => result |= 1 << 19,
            LeftBracket => result |= 1 << 20,
            RightBracket => result |= 1 << 21,
            BackSlash => result |= 1 << 22,
            Semicolon => result |= 1 << 23,
            Apostrophe => result |= 1 << 24,
            Comma => result |= 1 << 25,
            Dot => result |= 1 << 26,
            Slash => result |= 1 << 27,
            NumpadSubtract => result |= 1 << 28,
            NumpadAdd => result |= 1 << 29,
            NumpadDivide => result |= 1 << 30,
            NumpadMultiply => result |= 1 << 31,
            _ => (),
        }
    }
    result
}
