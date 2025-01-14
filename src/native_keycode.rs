use glutin::event::VirtualKeyCode;

pub fn translate_virtual_key(c: VirtualKeyCode) -> &'static str {
    match c {
        VirtualKeyCode::Key1 => "Digit1",
        VirtualKeyCode::Key2 => "Digit2",
        VirtualKeyCode::Key3 => "Digit3",
        VirtualKeyCode::Key4 => "Digit4",
        VirtualKeyCode::Key5 => "Digit5",
        VirtualKeyCode::Key6 => "Digit6",
        VirtualKeyCode::Key7 => "Digit7",
        VirtualKeyCode::Key8 => "Digit8",
        VirtualKeyCode::Key9 => "Digit9",
        VirtualKeyCode::Key0 => "Digit0",
        VirtualKeyCode::A => "KeyA",
        VirtualKeyCode::B => "KeyB",
        VirtualKeyCode::C => "KeyC",
        VirtualKeyCode::D => "KeyD",
        VirtualKeyCode::E => "KeyE",
        VirtualKeyCode::F => "KeyF",
        VirtualKeyCode::G => "KeyG",
        VirtualKeyCode::H => "KeyH",
        VirtualKeyCode::I => "KeyI",
        VirtualKeyCode::J => "KeyJ",
        VirtualKeyCode::K => "KeyK",
        VirtualKeyCode::L => "KeyL",
        VirtualKeyCode::M => "KeyM",
        VirtualKeyCode::N => "KeyN",
        VirtualKeyCode::O => "KeyO",
        VirtualKeyCode::P => "KeyP",
        VirtualKeyCode::Q => "KeyQ",
        VirtualKeyCode::R => "KeyR",
        VirtualKeyCode::S => "KeyS",
        VirtualKeyCode::T => "KeyT",
        VirtualKeyCode::U => "KeyU",
        VirtualKeyCode::V => "KeyV",
        VirtualKeyCode::W => "KeyW",
        VirtualKeyCode::X => "KeyX",
        VirtualKeyCode::Y => "KeyY",
        VirtualKeyCode::Z => "KeyZ",
        VirtualKeyCode::Escape => "Escape",
        VirtualKeyCode::F1 => "F1",
        VirtualKeyCode::F2 => "F2",
        VirtualKeyCode::F3 => "F3",
        VirtualKeyCode::F4 => "F4",
        VirtualKeyCode::F5 => "F5",
        VirtualKeyCode::F6 => "F6",
        VirtualKeyCode::F7 => "F7",
        VirtualKeyCode::F8 => "F8",
        VirtualKeyCode::F9 => "F9",
        VirtualKeyCode::F10 => "F10",
        VirtualKeyCode::F11 => "F11",
        VirtualKeyCode::F12 => "F12",
        VirtualKeyCode::F13 => "F13",
        VirtualKeyCode::F14 => "F14",
        VirtualKeyCode::F15 => "F15",
        VirtualKeyCode::F16 => "F16",
        VirtualKeyCode::F17 => "F17",
        VirtualKeyCode::F18 => "F18",
        VirtualKeyCode::F19 => "F19",
        VirtualKeyCode::F20 => "F20",
        VirtualKeyCode::F21 => "F21",
        VirtualKeyCode::F22 => "F22",
        VirtualKeyCode::F23 => "F23",
        VirtualKeyCode::F24 => "F24",
        VirtualKeyCode::Snapshot => "",
        VirtualKeyCode::Scroll => "ScrollLock",
        VirtualKeyCode::Pause => "Pause",
        VirtualKeyCode::Insert => "Insert",
        VirtualKeyCode::Home => "Home",
        VirtualKeyCode::Delete => "Delete",
        VirtualKeyCode::End => "End",
        VirtualKeyCode::PageDown => "PageDown",
        VirtualKeyCode::PageUp => "PageUp",
        VirtualKeyCode::Left => "ArrowLeft",
        VirtualKeyCode::Up => "ArrowUp",
        VirtualKeyCode::Right => "ArrowRight",
        VirtualKeyCode::Down => "ArrowDown",
        VirtualKeyCode::Back => "Backspace",
        VirtualKeyCode::Return => "Enter",
        VirtualKeyCode::Space => "Space",
        VirtualKeyCode::Compose => "",
        VirtualKeyCode::Numlock => "NumLock",
        VirtualKeyCode::Numpad0 => "Numpad0",
        VirtualKeyCode::Numpad1 => "Numpad1",
        VirtualKeyCode::Numpad2 => "Numpad2",
        VirtualKeyCode::Numpad3 => "Numpad3",
        VirtualKeyCode::Numpad4 => "Numpad4",
        VirtualKeyCode::Numpad5 => "Numpad5",
        VirtualKeyCode::Numpad6 => "Numpad6",
        VirtualKeyCode::Numpad7 => "Numpad7",
        VirtualKeyCode::Numpad8 => "Numpad8",
        VirtualKeyCode::Numpad9 => "Numpad9",
        VirtualKeyCode::AbntC1 => "",
        VirtualKeyCode::AbntC2 => "",
        VirtualKeyCode::NumpadAdd => "NumpadAdd",
        VirtualKeyCode::Apostrophe => "Apostrophe",
        VirtualKeyCode::Apps => "",
        VirtualKeyCode::Asterisk => "Star",
        VirtualKeyCode::At => "",
        VirtualKeyCode::Ax => "",
        VirtualKeyCode::Backslash => "Backslash",
        VirtualKeyCode::Calculator => "",
        VirtualKeyCode::Capital => "CapsLock",
        VirtualKeyCode::Colon => "",
        VirtualKeyCode::Comma => "Comma",
        VirtualKeyCode::Convert => "",
        VirtualKeyCode::NumpadDecimal => "NumpadDecimal",
        VirtualKeyCode::NumpadDivide => "NumpadDivide",
        VirtualKeyCode::Equals => "Equal",
        VirtualKeyCode::Grave => "Backquote",
        VirtualKeyCode::Kana => "",
        VirtualKeyCode::Kanji => "",
        VirtualKeyCode::LAlt => "AltLeft",
        VirtualKeyCode::LBracket => "BracketLeft",
        VirtualKeyCode::LControl => "ControlLeft",
        VirtualKeyCode::LShift => "ShiftLeft",
        VirtualKeyCode::LWin => "",
        VirtualKeyCode::Mail => "",
        VirtualKeyCode::MediaSelect => "",
        VirtualKeyCode::MediaStop => "",
        VirtualKeyCode::Minus => "Minus",
        VirtualKeyCode::NumpadMultiply => "NumpadMultiply",
        VirtualKeyCode::Mute => "",
        VirtualKeyCode::MyComputer => "",
        VirtualKeyCode::NavigateForward => "",
        VirtualKeyCode::NavigateBackward => "",
        VirtualKeyCode::NextTrack => "",
        VirtualKeyCode::NoConvert => "",
        VirtualKeyCode::NumpadComma => "NumpadComma",
        VirtualKeyCode::NumpadEnter => "NumpadEnter",
        VirtualKeyCode::NumpadEquals => "NumpadEqual",
        VirtualKeyCode::OEM102 => "",
        VirtualKeyCode::Period => "Period",
        VirtualKeyCode::PlayPause => "",
        VirtualKeyCode::Power => "",
        VirtualKeyCode::Plus => "Plus",
        VirtualKeyCode::PrevTrack => "",
        VirtualKeyCode::RAlt => "AltRight",
        VirtualKeyCode::RBracket => "BracketRight",
        VirtualKeyCode::RControl => "ControlRight",
        VirtualKeyCode::RShift => "ShiftRight",
        VirtualKeyCode::RWin => "",
        VirtualKeyCode::Semicolon => "Semicolon",
        VirtualKeyCode::Slash => "Slash",
        VirtualKeyCode::Sleep => "",
        VirtualKeyCode::Stop => "",
        VirtualKeyCode::NumpadSubtract => "NumpadSubtract",
        VirtualKeyCode::Sysrq => "",
        VirtualKeyCode::Tab => "Tab",
        VirtualKeyCode::Underline => "",
        VirtualKeyCode::Unlabeled => "",
        VirtualKeyCode::VolumeDown => "",
        VirtualKeyCode::VolumeUp => "",
        VirtualKeyCode::Wake => "",
        VirtualKeyCode::WebBack => "",
        VirtualKeyCode::WebFavorites => "",
        VirtualKeyCode::WebForward => "",
        VirtualKeyCode::WebHome => "",
        VirtualKeyCode::WebRefresh => "",
        VirtualKeyCode::WebSearch => "",
        VirtualKeyCode::WebStop => "",
        VirtualKeyCode::Yen => "",
        VirtualKeyCode::Caret => "Caret",
        VirtualKeyCode::Copy => "Copy",
        VirtualKeyCode::Paste => "Paste",
        VirtualKeyCode::Cut => "Cut",
    }
}
#[cfg(target_os = "macos")]
pub fn translate_scan_code(c: u32) -> &'static str {
    match c {
        0x12 => "Digit1",
        0x13 => "Digit2",
        0x14 => "Digit3",
        0x15 => "Digit4",
        0x17 => "Digit5",
        0x16 => "Digit6",
        0x1A => "Digit7",
        0x1C => "Digit8",
        0x19 => "Digit9",
        0x1D => "Digit0",
        0x00 => "KeyA",
        0x0B => "KeyB",
        0x08 => "KeyC",
        0x02 => "KeyD",
        0x0E => "KeyE",
        0x03 => "KeyF",
        0x05 => "KeyG",
        0x04 => "KeyH",
        0x22 => "KeyI",
        0x26 => "KeyJ",
        0x28 => "KeyK",
        0x25 => "KeyL",
        0x2E => "KeyM",
        0x2D => "KeyN",
        0x1F => "KeyO",
        0x23 => "KeyP",
        0x0C => "KeyQ",
        0x0F => "KeyR",
        0x01 => "KeyS",
        0x11 => "KeyT",
        0x20 => "KeyU",
        0x09 => "KeyV",
        0x0D => "KeyW",
        0x07 => "KeyX",
        0x10 => "KeyY",
        0x06 => "KeyZ",
        0x35 => "Escape",
        0x7A => "F1",
        0x78 => "F2",
        0x63 => "F3",
        0x76 => "F4",
        0x60 => "F5",
        0x61 => "F6",
        0x62 => "F7",
        0x64 => "F8",
        0x65 => "F9",
        0x6D => "F10",
        0x67 => "F11",
        0x6F => "F12",
        0x69 => "F13",
        0x6B => "F14",
        0x71 => "F15",
        // 0x37 => "Snapshot",
        // 0x46 => "ScrollLock",
        // Pause => "Pause",
        // 0x52 => "Insert",
        0x73 => "Home",
        0x75 => "Delete",
        0x77 => "End",
        0x79 => "PageDown",
        0x74 => "PageUp",
        0x7B => "ArrowLeft",
        0x7E => "ArrowUp",
        0x7C => "ArrowRight",
        0x7D => "ArrowDown",
        0x33 => "Backspace",
        0x24 => "Enter",
        0x31 => "Space",
        // Compose => "",
        0x47 => "NumLock",
        0x52 => "Numpad0",
        0x53 => "Numpad1",
        0x54 => "Numpad2",
        0x55 => "Numpad3",
        0x56 => "Numpad4",
        0x57 => "Numpad5",
        0x58 => "Numpad6",
        0x59 => "Numpad7",
        0x5B => "Numpad8",
        0x5C => "Numpad9",
        // AbntC1 => "",
        // AbntC2 => "",
        0x45 => "NumpadAdd",
        0x27 => "Apostrophe",
        // Apps => "",
        // At => "",
        // Ax => "",
        0x2A => "Backslash",
        // Calculator => "",
        0x39 => "CapsLock",
        0x29 => "Colon",
        0x2B => "Comma",
        // Convert => "",
        0x41 => "NumpadDecimal",
        0x4B => "NumpadDivide",
        0x18 => "Equal",
        0x32 => "Backquote",
        // Kana => "",
        // Kanji => "",
        // LAlt => "",
        0x21 => "BracketLeft",
        0x3B => "ControlLeft",
        // 0x38 => "AltLeft",
        0x38 => "ShiftLeft",
        0x37 => "Command",
        // 0x5B => "LeftWin",
        // Mail => "",
        // MediaSelect => "",
        // MediaStop => "",
        0x1B => "Minus",
        0x43 => "NumpadMultiply",
        // Mute => "",
        // MyComputer => "",
        // NavigateForward => "",
        // NavigateBackward => "",
        // NextTrack => "",
        // NoConvert => "",
        // NumpadComma => "NumpadComma",
        0x4C => "NumpadEnter",
        0x51 => "NumpadEqual",
        // OEM102 => "",
        0x2F => "Period",
        // PlayPause => "",
        // Power => "",
        // PrevTrack => "",
        // RAlt => "",
        0x1E => "BracketRight",
        // RControl => "ControlRight", // Same as ControlLeft
        // RMenu => "AltRight", // Same as ControlLeft
        // 0x36 => "ShiftRight",
        // RWin => "",
        // 0x27 => "Semicolon",
        0x2C => "Slash",
        // Sleep => "",
        // Stop => "",
        0x4E => "NumpadSubtract",
        // Sysrq => "",
        0x30 => "Tab",
        // Underline => "",
        // Unlabeled => "",
        // VolumeDown => "",
        // VolumeUp => "",
        // Wake => "",
        // WebBack => "",
        // WebFavorites => "",
        // WebForward => "",
        // WebHome => "",
        // WebRefresh => "",
        // WebSearch => "",
        // WebStop => "",
        // Yen => "",
        // Caret => "Caret",
        _ => "",
    }
}

#[cfg(target_os = "windows")]
pub fn translate_scan_code(c: u32) -> &'static str {
    match c {
        0x02 => "Digit1",
        0x03 => "Digit2",
        0x04 => "Digit3",
        0x05 => "Digit4",
        0x06 => "Digit5",
        0x07 => "Digit6",
        0x08 => "Digit7",
        0x09 => "Digit8",
        0x0A => "Digit9",
        0x0B => "Digit0",
        0x1E => "KeyA",
        0x30 => "KeyB",
        0x2E => "KeyC",
        0x20 => "KeyD",
        0x12 => "KeyE",
        0x21 => "KeyF",
        0x22 => "KeyG",
        0x23 => "KeyH",
        0x17 => "KeyI",
        0x24 => "KeyJ",
        0x25 => "KeyK",
        0x26 => "KeyL",
        0x32 => "KeyM",
        0x31 => "KeyN",
        0x18 => "KeyO",
        0x19 => "KeyP",
        0x10 => "KeyQ",
        0x13 => "KeyR",
        0x1F => "KeyS",
        0x14 => "KeyT",
        0x16 => "KeyU",
        0x2F => "KeyV",
        0x11 => "KeyW",
        0x2D => "KeyX",
        0x15 => "KeyY",
        0x2C => "KeyZ",
        0x01 => "Escape",
        0x3B => "F1",
        0x3C => "F2",
        0x3D => "F3",
        0x3E => "F4",
        0x3F => "F5",
        0x40 => "F6",
        0x41 => "F7",
        0x42 => "F8",
        0x43 => "F9",
        0x44 => "F10",
        0x85 => "F11",
        0x86 => "F12",
        // F13 => "F13",
        // F14 => "F14",
        // F15 => "F15",
        0x37 => "Snapshot",
        0x46 => "ScrollLock",
        // Pause => "Pause",
        0x52 => "Insert",
        0x47 => "Home",
        0x53 => "Delete",
        0x4F => "End",
        0x51 => "PageDown",
        0x49 => "PageUp",
        0x4B => "ArrowLeft",
        0x48 => "ArrowUp",
        0x4D => "ArrowRight",
        0x50 => "ArrowDown",
        0x0E => "Backspace",
        0x1C => "Enter",
        0x39 => "Space",
        // Compose => "",
        0x45 => "NumLock",
        // Numpad0 => "Numpad0", // Same as Insert
        // Numpad1 => "Numpad1", // Same as End
        // Numpad2 => "Numpad2", // Same as ArrowDown
        // Numpad3 => "Numpad3", // Same as PageDown
        // Numpad4 => "Numpad4", // Same as ArrowLeft
        0x4C => "Numpad5",
        // Numpad6 => "Numpad6", // Same as ArrowRight
        // Numpad7 => "Numpad7", // Same as Home
        // Numpad8 => "Numpad8", // Same as ArrowUp
        // Numpad9 => "Numpad9", // Same as PageUp
        // AbntC1 => "",
        // AbntC2 => "",
        0x4E => "NumpadAdd",
        // Apostrophe => "Apostrophe",
        // Apps => "",
        // At => "",
        // Ax => "",
        0x2B => "Backslash",
        // Calculator => "",
        0x3A => "CapsLock",
        0x27 => "Colon",
        0x33 => "Comma",
        // Convert => "",
        // Decimal => "NumpadDecimal", // Same as Delete
        0x35 => "NumpadDivide",
        0x0D => "Equal",
        0x29 => "Backquote",
        // Kana => "",
        // Kanji => "",
        // LAlt => "",
        0x1A => "BracketLeft",
        0x1D => "ControlLeft",
        0x38 => "AltLeft",
        0x2A => "ShiftLeft",
        0x5B => "LeftWin",
        // Mail => "",
        // MediaSelect => "",
        // MediaStop => "",
        0x0C => "Minus",
        // Multiply => "NumpadMultiply", // Same as Snapshot
        // Mute => "",
        // MyComputer => "",
        // NavigateForward => "",
        // NavigateBackward => "",
        // NextTrack => "",
        // NoConvert => "",
        // NumpadComma => "NumpadComma",
        // NumpadEnter => "NumpadEnter", // Same as Enter
        // NumpadEquals => "NumpadEqual",
        // OEM102 => "",
        0x34 => "Period",
        // PlayPause => "",
        // Power => "",
        // PrevTrack => "",
        // RAlt => "",
        0x1B => "BracketRight",
        // RControl => "ControlRight", // Same as ControlLeft
        // RMenu => "AltRight", // Same as ControlLeft
        0x36 => "ShiftRight",
        // RWin => "",
        // 0x27 => "Semicolon",
        // 0x35 => "Slash", // Same as NumpadDivide
        // Sleep => "",
        // Stop => "",
        0x4A => "NumpadSubtract",
        // Sysrq => "",
        0x0F => "Tab",
        // Underline => "",
        // Unlabeled => "",
        // VolumeDown => "",
        // VolumeUp => "",
        // Wake => "",
        // WebBack => "",
        // WebFavorites => "",
        // WebForward => "",
        // WebHome => "",
        // WebRefresh => "",
        // WebSearch => "",
        // WebStop => "",
        // Yen => "",
        // Caret => "Caret",
        _ => "",
    }
}

#[cfg(target_os = "linux")]
pub fn translate_scan_code(c: u32) -> &'static str {
    match c {
        0x02 => "Digit1",
        0x03 => "Digit2",
        0x04 => "Digit3",
        0x05 => "Digit4",
        0x06 => "Digit5",
        0x07 => "Digit6",
        0x08 => "Digit7",
        0x09 => "Digit8",
        0x0A => "Digit9",
        0x0B => "Digit0",
        0x1E => "KeyA",
        0x30 => "KeyB",
        0x2E => "KeyC",
        0x20 => "KeyD",
        0x12 => "KeyE",
        0x21 => "KeyF",
        0x22 => "KeyG",
        0x23 => "KeyH",
        0x17 => "KeyI",
        0x24 => "KeyJ",
        0x25 => "KeyK",
        0x26 => "KeyL",
        0x32 => "KeyM",
        0x31 => "KeyN",
        0x18 => "KeyO",
        0x19 => "KeyP",
        0x10 => "KeyQ",
        0x13 => "KeyR",
        0x1F => "KeyS",
        0x14 => "KeyT",
        0x16 => "KeyU",
        0x2F => "KeyV",
        0x11 => "KeyW",
        0x2D => "KeyX",
        0x15 => "KeyY",
        0x2C => "KeyZ",
        0x01 => "Escape",
        0x3B => "F1",
        0x3C => "F2",
        0x3D => "F3",
        0x3E => "F4",
        0x3F => "F5",
        0x40 => "F6",
        0x41 => "F7",
        0x42 => "F8",
        0x43 => "F9",
        0x44 => "F10",
        0x85 => "F11",
        0x86 => "F12",
        // F13 => "F13",
        // F14 => "F14",
        // F15 => "F15",
        0x37 => "Snapshot",
        0x46 => "ScrollLock",
        // Pause => "Pause",
        0x52 => "Insert",
        0x47 => "Home",
        0x53 => "Delete",
        0x4F => "End",
        0x6D => "PageDown",
        0x68 => "PageUp",
        0x69 => "ArrowLeft",
        0x67 => "ArrowUp",
        0x6a => "ArrowRight",
        0x6c => "ArrowDown",
        0x0E => "Backspace",
        0x1C => "Enter",
        0x39 => "Space",
        // Compose => "",
        0x45 => "NumLock",
        // Numpad0 => "Numpad0", // Same as Insert
        // Numpad1 => "Numpad1", // Same as End
        // Numpad2 => "Numpad2", // Same as ArrowDown
        // Numpad3 => "Numpad3", // Same as PageDown
        // Numpad4 => "Numpad4", // Same as ArrowLeft
        0x4C => "Numpad5",
        // Numpad6 => "Numpad6", // Same as ArrowRight
        // Numpad7 => "Numpad7", // Same as Home
        // Numpad8 => "Numpad8", // Same as ArrowUp
        // Numpad9 => "Numpad9", // Same as PageUp
        // AbntC1 => "",
        // AbntC2 => "",
        0x4E => "NumpadAdd",
        // Apostrophe => "Apostrophe",
        // Apps => "",
        // At => "",
        // Ax => "",
        0x2B => "Backslash",
        // Calculator => "",
        0x3A => "CapsLock",
        0x27 => "Colon",
        0x33 => "Comma",
        // Convert => "",
        // Decimal => "NumpadDecimal", // Same as Delete
        0x35 => "NumpadDivide",
        0x0D => "Equal",
        0x29 => "Backquote",
        // Kana => "",
        // Kanji => "",
        // LAlt => "",
        0x1A => "BracketLeft",
        0x1D => "ControlLeft",
        0x38 => "AltLeft",
        0x2A => "ShiftLeft",
        0x5B => "LeftWin",
        // Mail => "",
        // MediaSelect => "",
        // MediaStop => "",
        0x0C => "Minus",
        // Multiply => "NumpadMultiply", // Same as Snapshot
        // Mute => "",
        // MyComputer => "",
        // NavigateForward => "",
        // NavigateBackward => "",
        // NextTrack => "",
        // NoConvert => "",
        // NumpadComma => "NumpadComma",
        // NumpadEnter => "NumpadEnter", // Same as Enter
        // NumpadEquals => "NumpadEqual",
        // OEM102 => "",
        0x34 => "Period",
        // PlayPause => "",
        // Power => "",
        // PrevTrack => "",
        // RAlt => "",
        0x1B => "BracketRight",
        // RControl => "ControlRight", // Same as ControlLeft
        // RMenu => "AltRight", // Same as ControlLeft
        0x36 => "ShiftRight",
        // RWin => "",
        // 0x27 => "Semicolon",
        // 0x35 => "Slash", // Same as NumpadDivide
        // Sleep => "",
        // Stop => "",
        0x4A => "NumpadSubtract",
        // Sysrq => "",
        0x0F => "Tab",
        // Underline => "",
        // Unlabeled => "",
        // VolumeDown => "",
        // VolumeUp => "",
        // Wake => "",
        // WebBack => "",
        // WebFavorites => "",
        // WebForward => "",
        // WebHome => "",
        // WebRefresh => "",
        // WebSearch => "",
        // WebStop => "",
        // Yen => "",
        // Caret => "Caret",
        _ => "",
    }
}
