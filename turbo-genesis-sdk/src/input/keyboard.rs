use borsh::BorshDeserialize;
use std::ops::Deref;
use turbo_genesis_abi::{TurboButton, TurboKeyboard};

pub use turbo_genesis_abi::TurboKeyCode;

/// A physical or logical key code, such as "A", "Enter", or "ArrowLeft".
#[derive(Debug)]
pub struct KeyCode(pub TurboKeyCode);

impl KeyCode {
    /// A list of all KeyCodes
    pub const ALL: &[Self] = &[
        KeyCode(TurboKeyCode::Backquote),
        KeyCode(TurboKeyCode::Backslash),
        KeyCode(TurboKeyCode::BracketLeft),
        KeyCode(TurboKeyCode::BracketRight),
        KeyCode(TurboKeyCode::Comma),
        KeyCode(TurboKeyCode::Digit0),
        KeyCode(TurboKeyCode::Digit1),
        KeyCode(TurboKeyCode::Digit2),
        KeyCode(TurboKeyCode::Digit3),
        KeyCode(TurboKeyCode::Digit4),
        KeyCode(TurboKeyCode::Digit5),
        KeyCode(TurboKeyCode::Digit6),
        KeyCode(TurboKeyCode::Digit7),
        KeyCode(TurboKeyCode::Digit8),
        KeyCode(TurboKeyCode::Digit9),
        KeyCode(TurboKeyCode::Equal),
        KeyCode(TurboKeyCode::IntlBackslash),
        KeyCode(TurboKeyCode::IntlRo),
        KeyCode(TurboKeyCode::IntlYen),
        KeyCode(TurboKeyCode::KeyA),
        KeyCode(TurboKeyCode::KeyB),
        KeyCode(TurboKeyCode::KeyC),
        KeyCode(TurboKeyCode::KeyD),
        KeyCode(TurboKeyCode::KeyE),
        KeyCode(TurboKeyCode::KeyF),
        KeyCode(TurboKeyCode::KeyG),
        KeyCode(TurboKeyCode::KeyH),
        KeyCode(TurboKeyCode::KeyI),
        KeyCode(TurboKeyCode::KeyJ),
        KeyCode(TurboKeyCode::KeyK),
        KeyCode(TurboKeyCode::KeyL),
        KeyCode(TurboKeyCode::KeyM),
        KeyCode(TurboKeyCode::KeyN),
        KeyCode(TurboKeyCode::KeyO),
        KeyCode(TurboKeyCode::KeyP),
        KeyCode(TurboKeyCode::KeyQ),
        KeyCode(TurboKeyCode::KeyR),
        KeyCode(TurboKeyCode::KeyS),
        KeyCode(TurboKeyCode::KeyT),
        KeyCode(TurboKeyCode::KeyU),
        KeyCode(TurboKeyCode::KeyV),
        KeyCode(TurboKeyCode::KeyW),
        KeyCode(TurboKeyCode::KeyX),
        KeyCode(TurboKeyCode::KeyY),
        KeyCode(TurboKeyCode::KeyZ),
        KeyCode(TurboKeyCode::Minus),
        KeyCode(TurboKeyCode::Period),
        KeyCode(TurboKeyCode::Quote),
        KeyCode(TurboKeyCode::Semicolon),
        KeyCode(TurboKeyCode::Slash),
        KeyCode(TurboKeyCode::AltLeft),
        KeyCode(TurboKeyCode::AltRight),
        KeyCode(TurboKeyCode::Backspace),
        KeyCode(TurboKeyCode::CapsLock),
        KeyCode(TurboKeyCode::ContextMenu),
        KeyCode(TurboKeyCode::ControlLeft),
        KeyCode(TurboKeyCode::ControlRight),
        KeyCode(TurboKeyCode::Enter),
        KeyCode(TurboKeyCode::SuperLeft),
        KeyCode(TurboKeyCode::SuperRight),
        KeyCode(TurboKeyCode::ShiftLeft),
        KeyCode(TurboKeyCode::ShiftRight),
        KeyCode(TurboKeyCode::Space),
        KeyCode(TurboKeyCode::Tab),
        KeyCode(TurboKeyCode::Convert),
        KeyCode(TurboKeyCode::KanaMode),
        KeyCode(TurboKeyCode::Lang1),
        KeyCode(TurboKeyCode::Lang2),
        KeyCode(TurboKeyCode::Lang3),
        KeyCode(TurboKeyCode::Lang4),
        KeyCode(TurboKeyCode::Lang5),
        KeyCode(TurboKeyCode::NonConvert),
        KeyCode(TurboKeyCode::Delete),
        KeyCode(TurboKeyCode::End),
        KeyCode(TurboKeyCode::Help),
        KeyCode(TurboKeyCode::Home),
        KeyCode(TurboKeyCode::Insert),
        KeyCode(TurboKeyCode::PageDown),
        KeyCode(TurboKeyCode::PageUp),
        KeyCode(TurboKeyCode::ArrowDown),
        KeyCode(TurboKeyCode::ArrowLeft),
        KeyCode(TurboKeyCode::ArrowRight),
        KeyCode(TurboKeyCode::ArrowUp),
        KeyCode(TurboKeyCode::NumLock),
        KeyCode(TurboKeyCode::Numpad0),
        KeyCode(TurboKeyCode::Numpad1),
        KeyCode(TurboKeyCode::Numpad2),
        KeyCode(TurboKeyCode::Numpad3),
        KeyCode(TurboKeyCode::Numpad4),
        KeyCode(TurboKeyCode::Numpad5),
        KeyCode(TurboKeyCode::Numpad6),
        KeyCode(TurboKeyCode::Numpad7),
        KeyCode(TurboKeyCode::Numpad8),
        KeyCode(TurboKeyCode::Numpad9),
        KeyCode(TurboKeyCode::NumpadAdd),
        KeyCode(TurboKeyCode::NumpadBackspace),
        KeyCode(TurboKeyCode::NumpadClear),
        KeyCode(TurboKeyCode::NumpadClearEntry),
        KeyCode(TurboKeyCode::NumpadComma),
        KeyCode(TurboKeyCode::NumpadDecimal),
        KeyCode(TurboKeyCode::NumpadDivide),
        KeyCode(TurboKeyCode::NumpadEnter),
        KeyCode(TurboKeyCode::NumpadEqual),
        KeyCode(TurboKeyCode::NumpadHash),
        KeyCode(TurboKeyCode::NumpadMemoryAdd),
        KeyCode(TurboKeyCode::NumpadMemoryClear),
        KeyCode(TurboKeyCode::NumpadMemoryRecall),
        KeyCode(TurboKeyCode::NumpadMemoryStore),
        KeyCode(TurboKeyCode::NumpadMemorySubtract),
        KeyCode(TurboKeyCode::NumpadMultiply),
        KeyCode(TurboKeyCode::NumpadParenLeft),
        KeyCode(TurboKeyCode::NumpadParenRight),
        KeyCode(TurboKeyCode::NumpadStar),
        KeyCode(TurboKeyCode::NumpadSubtract),
        KeyCode(TurboKeyCode::Escape),
        KeyCode(TurboKeyCode::Fn),
        KeyCode(TurboKeyCode::FnLock),
        KeyCode(TurboKeyCode::PrintScreen),
        KeyCode(TurboKeyCode::ScrollLock),
        KeyCode(TurboKeyCode::Pause),
        KeyCode(TurboKeyCode::BrowserBack),
        KeyCode(TurboKeyCode::BrowserFavorites),
        KeyCode(TurboKeyCode::BrowserForward),
        KeyCode(TurboKeyCode::BrowserHome),
        KeyCode(TurboKeyCode::BrowserRefresh),
        KeyCode(TurboKeyCode::BrowserSearch),
        KeyCode(TurboKeyCode::BrowserStop),
        KeyCode(TurboKeyCode::Eject),
        KeyCode(TurboKeyCode::LaunchApp1),
        KeyCode(TurboKeyCode::LaunchApp2),
        KeyCode(TurboKeyCode::LaunchMail),
        KeyCode(TurboKeyCode::MediaPlayPause),
        KeyCode(TurboKeyCode::MediaSelect),
        KeyCode(TurboKeyCode::MediaStop),
        KeyCode(TurboKeyCode::MediaTrackNext),
        KeyCode(TurboKeyCode::MediaTrackPrevious),
        KeyCode(TurboKeyCode::Power),
        KeyCode(TurboKeyCode::Sleep),
        KeyCode(TurboKeyCode::AudioVolumeDown),
        KeyCode(TurboKeyCode::AudioVolumeMute),
        KeyCode(TurboKeyCode::AudioVolumeUp),
        KeyCode(TurboKeyCode::WakeUp),
        KeyCode(TurboKeyCode::Meta),
        KeyCode(TurboKeyCode::Hyper),
        KeyCode(TurboKeyCode::Turbo),
        KeyCode(TurboKeyCode::Abort),
        KeyCode(TurboKeyCode::Resume),
        KeyCode(TurboKeyCode::Suspend),
        KeyCode(TurboKeyCode::Again),
        KeyCode(TurboKeyCode::Copy),
        KeyCode(TurboKeyCode::Cut),
        KeyCode(TurboKeyCode::Find),
        KeyCode(TurboKeyCode::Open),
        KeyCode(TurboKeyCode::Paste),
        KeyCode(TurboKeyCode::Props),
        KeyCode(TurboKeyCode::Select),
        KeyCode(TurboKeyCode::Undo),
        KeyCode(TurboKeyCode::Hiragana),
        KeyCode(TurboKeyCode::Katakana),
        KeyCode(TurboKeyCode::F1),
        KeyCode(TurboKeyCode::F2),
        KeyCode(TurboKeyCode::F3),
        KeyCode(TurboKeyCode::F4),
        KeyCode(TurboKeyCode::F5),
        KeyCode(TurboKeyCode::F6),
        KeyCode(TurboKeyCode::F7),
        KeyCode(TurboKeyCode::F8),
        KeyCode(TurboKeyCode::F9),
        KeyCode(TurboKeyCode::F10),
        KeyCode(TurboKeyCode::F11),
        KeyCode(TurboKeyCode::F12),
        KeyCode(TurboKeyCode::F13),
        KeyCode(TurboKeyCode::F14),
        KeyCode(TurboKeyCode::F15),
        KeyCode(TurboKeyCode::F16),
        KeyCode(TurboKeyCode::F17),
        KeyCode(TurboKeyCode::F18),
        KeyCode(TurboKeyCode::F19),
        KeyCode(TurboKeyCode::F20),
        KeyCode(TurboKeyCode::F21),
        KeyCode(TurboKeyCode::F22),
        KeyCode(TurboKeyCode::F23),
        KeyCode(TurboKeyCode::F24),
        KeyCode(TurboKeyCode::F25),
        KeyCode(TurboKeyCode::F26),
        KeyCode(TurboKeyCode::F27),
        KeyCode(TurboKeyCode::F28),
        KeyCode(TurboKeyCode::F29),
        KeyCode(TurboKeyCode::F30),
        KeyCode(TurboKeyCode::F31),
        KeyCode(TurboKeyCode::F32),
        KeyCode(TurboKeyCode::F33),
        KeyCode(TurboKeyCode::F34),
        KeyCode(TurboKeyCode::F35),
    ];

    /// Returns a slice of all possible `KeyCode`s
    pub fn all() -> &'static [Self] {
        Self::ALL
    }

    // Converts a `KeyCode` into a `char` if possible
    pub fn as_char(&self, shift: bool) -> Option<char> {
        use TurboKeyCode::*;
        match (self.0, shift) {
            // Letters
            (KeyA, false) => Some('a'),
            (KeyA, true) => Some('A'),
            (KeyB, false) => Some('b'),
            (KeyB, true) => Some('B'),
            (KeyC, false) => Some('c'),
            (KeyC, true) => Some('C'),
            (KeyD, false) => Some('d'),
            (KeyD, true) => Some('D'),
            (KeyE, false) => Some('e'),
            (KeyE, true) => Some('E'),
            (KeyF, false) => Some('f'),
            (KeyF, true) => Some('F'),
            (KeyG, false) => Some('g'),
            (KeyG, true) => Some('G'),
            (KeyH, false) => Some('h'),
            (KeyH, true) => Some('H'),
            (KeyI, false) => Some('i'),
            (KeyI, true) => Some('I'),
            (KeyJ, false) => Some('j'),
            (KeyJ, true) => Some('J'),
            (KeyK, false) => Some('k'),
            (KeyK, true) => Some('K'),
            (KeyL, false) => Some('l'),
            (KeyL, true) => Some('L'),
            (KeyM, false) => Some('m'),
            (KeyM, true) => Some('M'),
            (KeyN, false) => Some('n'),
            (KeyN, true) => Some('N'),
            (KeyO, false) => Some('o'),
            (KeyO, true) => Some('O'),
            (KeyP, false) => Some('p'),
            (KeyP, true) => Some('P'),
            (KeyQ, false) => Some('q'),
            (KeyQ, true) => Some('Q'),
            (KeyR, false) => Some('r'),
            (KeyR, true) => Some('R'),
            (KeyS, false) => Some('s'),
            (KeyS, true) => Some('S'),
            (KeyT, false) => Some('t'),
            (KeyT, true) => Some('T'),
            (KeyU, false) => Some('u'),
            (KeyU, true) => Some('U'),
            (KeyV, false) => Some('v'),
            (KeyV, true) => Some('V'),
            (KeyW, false) => Some('w'),
            (KeyW, true) => Some('W'),
            (KeyX, false) => Some('x'),
            (KeyX, true) => Some('X'),
            (KeyY, false) => Some('y'),
            (KeyY, true) => Some('Y'),
            (KeyZ, false) => Some('z'),
            (KeyZ, true) => Some('Z'),

            // Digits
            (Digit0, false) => Some('0'),
            (Digit0, true) => Some(')'),
            (Digit1, false) => Some('1'),
            (Digit1, true) => Some('!'),
            (Digit2, false) => Some('2'),
            (Digit2, true) => Some('@'),
            (Digit3, false) => Some('3'),
            (Digit3, true) => Some('#'),
            (Digit4, false) => Some('4'),
            (Digit4, true) => Some('$'),
            (Digit5, false) => Some('5'),
            (Digit5, true) => Some('%'),
            (Digit6, false) => Some('6'),
            (Digit6, true) => Some('^'),
            (Digit7, false) => Some('7'),
            (Digit7, true) => Some('&'),
            (Digit8, false) => Some('8'),
            (Digit8, true) => Some('*'),
            (Digit9, false) => Some('9'),
            (Digit9, true) => Some('('),

            // Symbols
            (Space, _) => Some(' '),
            (Enter, _) => Some('\n'),
            (Tab, _) => Some('\t'),
            (Backquote, false) => Some('`'),
            (Backquote, true) => Some('~'),
            (Minus, false) => Some('-'),
            (Minus, true) => Some('_'),
            (Equal, false) => Some('='),
            (Equal, true) => Some('+'),
            (BracketLeft, false) => Some('['),
            (BracketLeft, true) => Some('{'),
            (BracketRight, false) => Some(']'),
            (BracketRight, true) => Some('}'),
            (Backslash, false) => Some('\\'),
            (Backslash, true) => Some('|'),
            (Semicolon, false) => Some(';'),
            (Semicolon, true) => Some(':'),
            (Quote, false) => Some('\''),
            (Quote, true) => Some('"'),
            (Comma, false) => Some(','),
            (Comma, true) => Some('<'),
            (Period, false) => Some('.'),
            (Period, true) => Some('>'),
            (Slash, false) => Some('/'),
            (Slash, true) => Some('?'),

            // Numpad (basic)
            (Numpad0, _) => Some('0'),
            (Numpad1, _) => Some('1'),
            (Numpad2, _) => Some('2'),
            (Numpad3, _) => Some('3'),
            (Numpad4, _) => Some('4'),
            (Numpad5, _) => Some('5'),
            (Numpad6, _) => Some('6'),
            (Numpad7, _) => Some('7'),
            (Numpad8, _) => Some('8'),
            (Numpad9, _) => Some('9'),
            (NumpadAdd, _) => Some('+'),
            (NumpadSubtract, _) => Some('-'),
            (NumpadMultiply, _) => Some('*'),
            (NumpadDivide, _) => Some('/'),
            (NumpadDecimal, _) => Some('.'),
            (NumpadEqual, _) => Some('='),

            _ => None,
        }
    }
}

/// The full keyboard state, containing all tracked keys and their button states.
#[derive(Debug)]
pub struct Keyboard(TurboKeyboard);

impl Keyboard {
    /// Returns a vector of all characters just pressed this frame.
    pub fn chars(&self) -> Vec<char> {
        let shift = self.shift_left().pressed() || self.shift_right().pressed();
        self.0
            .keys
            .iter()
            .filter_map(|(key, btn)| {
                if btn.just_pressed() {
                    KeyCode(*key).as_char(shift)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns a string of all characters just pressed this frame.
    pub fn text(&self) -> String {
        self.chars().iter().collect()
    }

    /// Gets the `TurboButton` for backquote
    pub fn backquote(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Backquote)
    }

    /// Gets the `TurboButton` for backslash
    pub fn backslash(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Backslash)
    }

    /// Gets the `TurboButton` for bracket_left
    pub fn bracket_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BracketLeft)
    }

    /// Gets the `TurboButton` for bracket_right
    pub fn bracket_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BracketRight)
    }

    /// Gets the `TurboButton` for comma
    pub fn comma(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Comma)
    }

    /// Gets the `TurboButton` for digit0
    pub fn digit0(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit0)
    }

    /// Gets the `TurboButton` for digit1
    pub fn digit1(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit1)
    }

    /// Gets the `TurboButton` for digit2
    pub fn digit2(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit2)
    }

    /// Gets the `TurboButton` for digit3
    pub fn digit3(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit3)
    }

    /// Gets the `TurboButton` for digit4
    pub fn digit4(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit4)
    }

    /// Gets the `TurboButton` for digit5
    pub fn digit5(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit5)
    }

    /// Gets the `TurboButton` for digit6
    pub fn digit6(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit6)
    }

    /// Gets the `TurboButton` for digit7
    pub fn digit7(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit7)
    }

    /// Gets the `TurboButton` for digit8
    pub fn digit8(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit8)
    }

    /// Gets the `TurboButton` for digit9
    pub fn digit9(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Digit9)
    }

    /// Gets the `TurboButton` for equal
    pub fn equal(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Equal)
    }

    /// Gets the `TurboButton` for intl_backslash
    pub fn intl_backslash(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::IntlBackslash)
    }

    /// Gets the `TurboButton` for intl_ro
    pub fn intl_ro(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::IntlRo)
    }

    /// Gets the `TurboButton` for intl_yen
    pub fn intl_yen(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::IntlYen)
    }

    /// Gets the `TurboButton` for key_a
    pub fn key_a(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyA)
    }

    /// Gets the `TurboButton` for key_b
    pub fn key_b(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyB)
    }

    /// Gets the `TurboButton` for key_c
    pub fn key_c(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyC)
    }

    /// Gets the `TurboButton` for key_d
    pub fn key_d(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyD)
    }

    /// Gets the `TurboButton` for key_e
    pub fn key_e(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyE)
    }

    /// Gets the `TurboButton` for key_f
    pub fn key_f(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyF)
    }

    /// Gets the `TurboButton` for key_g
    pub fn key_g(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyG)
    }

    /// Gets the `TurboButton` for key_h
    pub fn key_h(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyH)
    }

    /// Gets the `TurboButton` for key_i
    pub fn key_i(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyI)
    }

    /// Gets the `TurboButton` for key_j
    pub fn key_j(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyJ)
    }

    /// Gets the `TurboButton` for key_k
    pub fn key_k(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyK)
    }

    /// Gets the `TurboButton` for key_l
    pub fn key_l(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyL)
    }

    /// Gets the `TurboButton` for key_m
    pub fn key_m(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyM)
    }

    /// Gets the `TurboButton` for key_n
    pub fn key_n(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyN)
    }

    /// Gets the `TurboButton` for key_o
    pub fn key_o(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyO)
    }

    /// Gets the `TurboButton` for key_p
    pub fn key_p(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyP)
    }

    /// Gets the `TurboButton` for key_q
    pub fn key_q(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyQ)
    }

    /// Gets the `TurboButton` for key_r
    pub fn key_r(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyR)
    }

    /// Gets the `TurboButton` for key_s
    pub fn key_s(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyS)
    }

    /// Gets the `TurboButton` for key_t
    pub fn key_t(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyT)
    }

    /// Gets the `TurboButton` for key_u
    pub fn key_u(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyU)
    }

    /// Gets the `TurboButton` for key_v
    pub fn key_v(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyV)
    }

    /// Gets the `TurboButton` for key_w
    pub fn key_w(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyW)
    }

    /// Gets the `TurboButton` for key_x
    pub fn key_x(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyX)
    }

    /// Gets the `TurboButton` for key_y
    pub fn key_y(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyY)
    }

    /// Gets the `TurboButton` for key_z
    pub fn key_z(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KeyZ)
    }

    /// Gets the `TurboButton` for minus
    pub fn minus(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Minus)
    }

    /// Gets the `TurboButton` for period
    pub fn period(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Period)
    }

    /// Gets the `TurboButton` for quote
    pub fn quote(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Quote)
    }

    /// Gets the `TurboButton` for semicolon
    pub fn semicolon(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Semicolon)
    }

    /// Gets the `TurboButton` for slash
    pub fn slash(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Slash)
    }

    /// Gets the `TurboButton` for alt_left
    pub fn alt_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::AltLeft)
    }

    /// Gets the `TurboButton` for alt_right
    pub fn alt_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::AltRight)
    }

    /// Gets the `TurboButton` for backspace
    pub fn backspace(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Backspace)
    }

    /// Gets the `TurboButton` for caps_lock
    pub fn caps_lock(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::CapsLock)
    }

    /// Gets the `TurboButton` for context_menu
    pub fn context_menu(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ContextMenu)
    }

    /// Gets the `TurboButton` for control_left
    pub fn control_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ControlLeft)
    }

    /// Gets the `TurboButton` for control_right
    pub fn control_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ControlRight)
    }

    /// Gets the `TurboButton` for enter
    pub fn enter(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Enter)
    }

    /// Gets the `TurboButton` for super_left
    pub fn super_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::SuperLeft)
    }

    /// Gets the `TurboButton` for super_right
    pub fn super_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::SuperRight)
    }

    /// Gets the `TurboButton` for shift_left
    pub fn shift_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ShiftLeft)
    }

    /// Gets the `TurboButton` for shift_right
    pub fn shift_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ShiftRight)
    }

    /// Gets the `TurboButton` for space
    pub fn space(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Space)
    }

    /// Gets the `TurboButton` for tab
    pub fn tab(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Tab)
    }

    /// Gets the `TurboButton` for convert
    pub fn convert(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Convert)
    }

    /// Gets the `TurboButton` for kana_mode
    pub fn kana_mode(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::KanaMode)
    }

    /// Gets the `TurboButton` for lang1
    pub fn lang1(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Lang1)
    }

    /// Gets the `TurboButton` for lang2
    pub fn lang2(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Lang2)
    }

    /// Gets the `TurboButton` for lang3
    pub fn lang3(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Lang3)
    }

    /// Gets the `TurboButton` for lang4
    pub fn lang4(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Lang4)
    }

    /// Gets the `TurboButton` for lang5
    pub fn lang5(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Lang5)
    }

    /// Gets the `TurboButton` for non_convert
    pub fn non_convert(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NonConvert)
    }

    /// Gets the `TurboButton` for delete
    pub fn delete(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Delete)
    }

    /// Gets the `TurboButton` for end
    pub fn end(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::End)
    }

    /// Gets the `TurboButton` for help
    pub fn help(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Help)
    }

    /// Gets the `TurboButton` for home
    pub fn home(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Home)
    }

    /// Gets the `TurboButton` for insert
    pub fn insert(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Insert)
    }

    /// Gets the `TurboButton` for page_down
    pub fn page_down(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::PageDown)
    }

    /// Gets the `TurboButton` for page_up
    pub fn page_up(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::PageUp)
    }

    /// Gets the `TurboButton` for arrow_down
    pub fn arrow_down(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ArrowDown)
    }

    /// Gets the `TurboButton` for arrow_left
    pub fn arrow_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ArrowLeft)
    }

    /// Gets the `TurboButton` for arrow_right
    pub fn arrow_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ArrowRight)
    }

    /// Gets the `TurboButton` for arrow_up
    pub fn arrow_up(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ArrowUp)
    }

    /// Gets the `TurboButton` for num_lock
    pub fn num_lock(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumLock)
    }

    /// Gets the `TurboButton` for numpad0
    pub fn numpad0(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad0)
    }

    /// Gets the `TurboButton` for numpad1
    pub fn numpad1(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad1)
    }

    /// Gets the `TurboButton` for numpad2
    pub fn numpad2(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad2)
    }

    /// Gets the `TurboButton` for numpad3
    pub fn numpad3(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad3)
    }

    /// Gets the `TurboButton` for numpad4
    pub fn numpad4(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad4)
    }

    /// Gets the `TurboButton` for numpad5
    pub fn numpad5(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad5)
    }

    /// Gets the `TurboButton` for numpad6
    pub fn numpad6(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad6)
    }

    /// Gets the `TurboButton` for numpad7
    pub fn numpad7(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad7)
    }

    /// Gets the `TurboButton` for numpad8
    pub fn numpad8(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad8)
    }

    /// Gets the `TurboButton` for numpad9
    pub fn numpad9(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Numpad9)
    }

    /// Gets the `TurboButton` for numpad_add
    pub fn numpad_add(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadAdd)
    }

    /// Gets the `TurboButton` for numpad_backspace
    pub fn numpad_backspace(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadBackspace)
    }

    /// Gets the `TurboButton` for numpad_clear
    pub fn numpad_clear(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadClear)
    }

    /// Gets the `TurboButton` for numpad_clear_entry
    pub fn numpad_clear_entry(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadClearEntry)
    }

    /// Gets the `TurboButton` for numpad_comma
    pub fn numpad_comma(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadComma)
    }

    /// Gets the `TurboButton` for numpad_decimal
    pub fn numpad_decimal(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadDecimal)
    }

    /// Gets the `TurboButton` for numpad_divide
    pub fn numpad_divide(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadDivide)
    }

    /// Gets the `TurboButton` for numpad_enter
    pub fn numpad_enter(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadEnter)
    }

    /// Gets the `TurboButton` for numpad_equal
    pub fn numpad_equal(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadEqual)
    }

    /// Gets the `TurboButton` for numpad_hash
    pub fn numpad_hash(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadHash)
    }

    /// Gets the `TurboButton` for numpad_memory_add
    pub fn numpad_memory_add(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadMemoryAdd)
    }

    /// Gets the `TurboButton` for numpad_memory_clear
    pub fn numpad_memory_clear(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadMemoryClear)
    }

    /// Gets the `TurboButton` for numpad_memory_recall
    pub fn numpad_memory_recall(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadMemoryRecall)
    }

    /// Gets the `TurboButton` for numpad_memory_store
    pub fn numpad_memory_store(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadMemoryStore)
    }

    /// Gets the `TurboButton` for numpad_memory_subtract
    pub fn numpad_memory_subtract(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadMemorySubtract)
    }

    /// Gets the `TurboButton` for numpad_multiply
    pub fn numpad_multiply(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadMultiply)
    }

    /// Gets the `TurboButton` for numpad_paren_left
    pub fn numpad_paren_left(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadParenLeft)
    }

    /// Gets the `TurboButton` for numpad_paren_right
    pub fn numpad_paren_right(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadParenRight)
    }

    /// Gets the `TurboButton` for numpad_star
    pub fn numpad_star(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadStar)
    }

    /// Gets the `TurboButton` for numpad_subtract
    pub fn numpad_subtract(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::NumpadSubtract)
    }

    /// Gets the `TurboButton` for escape
    pub fn escape(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Escape)
    }

    /// Gets the `TurboButton` for fn_
    pub fn fn_(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Fn)
    }

    /// Gets the `TurboButton` for fn_lock
    pub fn fn_lock(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::FnLock)
    }

    /// Gets the `TurboButton` for print_screen
    pub fn print_screen(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::PrintScreen)
    }

    /// Gets the `TurboButton` for scroll_lock
    pub fn scroll_lock(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::ScrollLock)
    }

    /// Gets the `TurboButton` for pause
    pub fn pause(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Pause)
    }

    /// Gets the `TurboButton` for browser_back
    pub fn browser_back(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserBack)
    }

    /// Gets the `TurboButton` for browser_favorites
    pub fn browser_favorites(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserFavorites)
    }

    /// Gets the `TurboButton` for browser_forward
    pub fn browser_forward(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserForward)
    }

    /// Gets the `TurboButton` for browser_home
    pub fn browser_home(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserHome)
    }

    /// Gets the `TurboButton` for browser_refresh
    pub fn browser_refresh(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserRefresh)
    }

    /// Gets the `TurboButton` for browser_search
    pub fn browser_search(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserSearch)
    }

    /// Gets the `TurboButton` for browser_stop
    pub fn browser_stop(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::BrowserStop)
    }

    /// Gets the `TurboButton` for eject
    pub fn eject(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Eject)
    }

    /// Gets the `TurboButton` for launch_app1
    pub fn launch_app1(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::LaunchApp1)
    }

    /// Gets the `TurboButton` for launch_app2
    pub fn launch_app2(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::LaunchApp2)
    }

    /// Gets the `TurboButton` for launch_mail
    pub fn launch_mail(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::LaunchMail)
    }

    /// Gets the `TurboButton` for media_play_pause
    pub fn media_play_pause(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::MediaPlayPause)
    }

    /// Gets the `TurboButton` for media_select
    pub fn media_select(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::MediaSelect)
    }

    /// Gets the `TurboButton` for media_stop
    pub fn media_stop(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::MediaStop)
    }

    /// Gets the `TurboButton` for media_track_next
    pub fn media_track_next(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::MediaTrackNext)
    }

    /// Gets the `TurboButton` for media_track_previous
    pub fn media_track_previous(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::MediaTrackPrevious)
    }

    /// Gets the `TurboButton` for power
    pub fn power(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Power)
    }

    /// Gets the `TurboButton` for sleep
    pub fn sleep(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Sleep)
    }

    /// Gets the `TurboButton` for audio_volume_down
    pub fn audio_volume_down(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::AudioVolumeDown)
    }

    /// Gets the `TurboButton` for audio_volume_mute
    pub fn audio_volume_mute(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::AudioVolumeMute)
    }

    /// Gets the `TurboButton` for audio_volume_up
    pub fn audio_volume_up(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::AudioVolumeUp)
    }

    /// Gets the `TurboButton` for wake_up
    pub fn wake_up(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::WakeUp)
    }

    /// Gets the `TurboButton` for meta
    pub fn meta(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Meta)
    }

    /// Gets the `TurboButton` for hyper
    pub fn hyper(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Hyper)
    }

    /// Gets the `TurboButton` for turbo
    pub fn turbo(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Turbo)
    }

    /// Gets the `TurboButton` for abort
    pub fn abort(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Abort)
    }

    /// Gets the `TurboButton` for resume
    pub fn resume(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Resume)
    }

    /// Gets the `TurboButton` for suspend
    pub fn suspend(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Suspend)
    }

    /// Gets the `TurboButton` for again
    pub fn again(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Again)
    }

    /// Gets the `TurboButton` for copy
    pub fn copy(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Copy)
    }

    /// Gets the `TurboButton` for cut
    pub fn cut(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Cut)
    }

    /// Gets the `TurboButton` for find
    pub fn find(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Find)
    }

    /// Gets the `TurboButton` for open
    pub fn open(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Open)
    }

    /// Gets the `TurboButton` for paste
    pub fn paste(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Paste)
    }

    /// Gets the `TurboButton` for props
    pub fn props(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Props)
    }

    /// Gets the `TurboButton` for select
    pub fn select(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Select)
    }

    /// Gets the `TurboButton` for undo
    pub fn undo(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Undo)
    }

    /// Gets the `TurboButton` for hiragana
    pub fn hiragana(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Hiragana)
    }

    /// Gets the `TurboButton` for katakana
    pub fn katakana(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::Katakana)
    }

    /// Gets the `TurboButton` for f1
    pub fn f1(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F1)
    }

    /// Gets the `TurboButton` for f2
    pub fn f2(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F2)
    }

    /// Gets the `TurboButton` for f3
    pub fn f3(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F3)
    }

    /// Gets the `TurboButton` for f4
    pub fn f4(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F4)
    }

    /// Gets the `TurboButton` for f5
    pub fn f5(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F5)
    }

    /// Gets the `TurboButton` for f6
    pub fn f6(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F6)
    }

    /// Gets the `TurboButton` for f7
    pub fn f7(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F7)
    }

    /// Gets the `TurboButton` for f8
    pub fn f8(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F8)
    }

    /// Gets the `TurboButton` for f9
    pub fn f9(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F9)
    }

    /// Gets the `TurboButton` for f10
    pub fn f10(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F10)
    }

    /// Gets the `TurboButton` for f11
    pub fn f11(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F11)
    }

    /// Gets the `TurboButton` for f12
    pub fn f12(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F12)
    }

    /// Gets the `TurboButton` for f13
    pub fn f13(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F13)
    }

    /// Gets the `TurboButton` for f14
    pub fn f14(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F14)
    }

    /// Gets the `TurboButton` for f15
    pub fn f15(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F15)
    }

    /// Gets the `TurboButton` for f16
    pub fn f16(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F16)
    }

    /// Gets the `TurboButton` for f17
    pub fn f17(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F17)
    }

    /// Gets the `TurboButton` for f18
    pub fn f18(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F18)
    }

    /// Gets the `TurboButton` for f19
    pub fn f19(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F19)
    }

    /// Gets the `TurboButton` for f20
    pub fn f20(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F20)
    }

    /// Gets the `TurboButton` for f21
    pub fn f21(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F21)
    }

    /// Gets the `TurboButton` for f22
    pub fn f22(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F22)
    }

    /// Gets the `TurboButton` for f23
    pub fn f23(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F23)
    }

    /// Gets the `TurboButton` for f24
    pub fn f24(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F24)
    }

    /// Gets the `TurboButton` for f25
    pub fn f25(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F25)
    }

    /// Gets the `TurboButton` for f26
    pub fn f26(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F26)
    }

    /// Gets the `TurboButton` for f27
    pub fn f27(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F27)
    }

    /// Gets the `TurboButton` for f28
    pub fn f28(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F28)
    }

    /// Gets the `TurboButton` for f29
    pub fn f29(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F29)
    }

    /// Gets the `TurboButton` for f30
    pub fn f30(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F30)
    }

    /// Gets the `TurboButton` for f31
    pub fn f31(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F31)
    }

    /// Gets the `TurboButton` for f32
    pub fn f32(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F32)
    }

    /// Gets the `TurboButton` for f33
    pub fn f33(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F33)
    }

    /// Gets the `TurboButton` for f34
    pub fn f34(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F34)
    }

    /// Gets the `TurboButton` for f35
    pub fn f35(&self) -> TurboButton {
        self.0.get(&TurboKeyCode::F35)
    }
}

/// Retrieves the current keyboard state from the FFI layer and deserializes it into a `Keyboard`.
pub fn get() -> Keyboard {
    // Preallocate a buffer for serialized keyboard data.
    let data = &mut vec![0; 1024];

    // Prepare a mutable length pointer for the FFI call to populate.
    let mut len = 0;
    let len_ptr = &mut len;

    // Call into FFI to fill `data` with serialized keyboard bytes and update `len`.
    turbo_genesis_ffi::input::keyboard(data.as_mut_ptr(), len_ptr);

    // Deserialize the ABI bytes into a `TurboKeyboard`.
    let inner = TurboKeyboard::try_from_slice(&data[..len as usize])
        .expect("Could not deserialize Keyboard");

    // Wrap in local `Keyboard` type for ergonomic use.
    Keyboard(inner)
}
