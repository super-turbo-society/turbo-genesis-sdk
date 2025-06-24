use crate::TurboButton;
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;

#[derive(
    Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize,
)]
pub struct TurboKeyboard {
    pub keys: BTreeMap<TurboKeyCode, TurboButton>,
}
impl TurboKeyboard {
    pub fn main_events_cleared(&mut self) {
        for key in self.keys.values_mut() {
            key.main_events_cleared();
        }
    }
    pub fn get(&self, keycode: &TurboKeyCode) -> TurboButton {
        *self.keys.get(keycode).unwrap_or(&TurboButton::Released)
    }
    pub fn backquote(&self) -> TurboButton {
        self.get(&TurboKeyCode::Backquote)
    }
    pub fn backslash(&self) -> TurboButton {
        self.get(&TurboKeyCode::Backslash)
    }
    pub fn bracket_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::BracketLeft)
    }
    pub fn bracket_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::BracketRight)
    }
    pub fn comma(&self) -> TurboButton {
        self.get(&TurboKeyCode::Comma)
    }
    pub fn digit0(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit0)
    }
    pub fn digit1(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit1)
    }
    pub fn digit2(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit2)
    }
    pub fn digit3(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit3)
    }
    pub fn digit4(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit4)
    }
    pub fn digit5(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit5)
    }
    pub fn digit6(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit6)
    }
    pub fn digit7(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit7)
    }
    pub fn digit8(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit8)
    }
    pub fn digit9(&self) -> TurboButton {
        self.get(&TurboKeyCode::Digit9)
    }
    pub fn equal(&self) -> TurboButton {
        self.get(&TurboKeyCode::Equal)
    }
    pub fn intl_backslash(&self) -> TurboButton {
        self.get(&TurboKeyCode::IntlBackslash)
    }
    pub fn intl_ro(&self) -> TurboButton {
        self.get(&TurboKeyCode::IntlRo)
    }
    pub fn intl_yen(&self) -> TurboButton {
        self.get(&TurboKeyCode::IntlYen)
    }
    pub fn key_a(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyA)
    }
    pub fn key_b(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyB)
    }
    pub fn key_c(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyC)
    }
    pub fn key_d(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyD)
    }
    pub fn key_e(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyE)
    }
    pub fn key_f(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyF)
    }
    pub fn key_g(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyG)
    }
    pub fn key_h(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyH)
    }
    pub fn key_i(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyI)
    }
    pub fn key_j(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyJ)
    }
    pub fn key_k(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyK)
    }
    pub fn key_l(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyL)
    }
    pub fn key_m(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyM)
    }
    pub fn key_n(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyN)
    }
    pub fn key_o(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyO)
    }
    pub fn key_p(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyP)
    }
    pub fn key_q(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyQ)
    }
    pub fn key_r(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyR)
    }
    pub fn key_s(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyS)
    }
    pub fn key_t(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyT)
    }
    pub fn key_u(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyU)
    }
    pub fn key_v(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyV)
    }
    pub fn key_w(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyW)
    }
    pub fn key_x(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyX)
    }
    pub fn key_y(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyY)
    }
    pub fn key_z(&self) -> TurboButton {
        self.get(&TurboKeyCode::KeyZ)
    }
    pub fn minus(&self) -> TurboButton {
        self.get(&TurboKeyCode::Minus)
    }
    pub fn period(&self) -> TurboButton {
        self.get(&TurboKeyCode::Period)
    }
    pub fn quote(&self) -> TurboButton {
        self.get(&TurboKeyCode::Quote)
    }
    pub fn semicolon(&self) -> TurboButton {
        self.get(&TurboKeyCode::Semicolon)
    }
    pub fn slash(&self) -> TurboButton {
        self.get(&TurboKeyCode::Slash)
    }
    pub fn alt_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::AltLeft)
    }
    pub fn alt_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::AltRight)
    }
    pub fn backspace(&self) -> TurboButton {
        self.get(&TurboKeyCode::Backspace)
    }
    pub fn caps_lock(&self) -> TurboButton {
        self.get(&TurboKeyCode::CapsLock)
    }
    pub fn context_menu(&self) -> TurboButton {
        self.get(&TurboKeyCode::ContextMenu)
    }
    pub fn control_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::ControlLeft)
    }
    pub fn control_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::ControlRight)
    }
    pub fn enter(&self) -> TurboButton {
        self.get(&TurboKeyCode::Enter)
    }
    pub fn super_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::SuperLeft)
    }
    pub fn super_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::SuperRight)
    }
    pub fn shift_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::ShiftLeft)
    }
    pub fn shift_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::ShiftRight)
    }
    pub fn space(&self) -> TurboButton {
        self.get(&TurboKeyCode::Space)
    }
    pub fn tab(&self) -> TurboButton {
        self.get(&TurboKeyCode::Tab)
    }
    pub fn convert(&self) -> TurboButton {
        self.get(&TurboKeyCode::Convert)
    }
    pub fn kana_mode(&self) -> TurboButton {
        self.get(&TurboKeyCode::KanaMode)
    }
    pub fn lang1(&self) -> TurboButton {
        self.get(&TurboKeyCode::Lang1)
    }
    pub fn lang2(&self) -> TurboButton {
        self.get(&TurboKeyCode::Lang2)
    }
    pub fn lang3(&self) -> TurboButton {
        self.get(&TurboKeyCode::Lang3)
    }
    pub fn lang4(&self) -> TurboButton {
        self.get(&TurboKeyCode::Lang4)
    }
    pub fn lang5(&self) -> TurboButton {
        self.get(&TurboKeyCode::Lang5)
    }
    pub fn non_convert(&self) -> TurboButton {
        self.get(&TurboKeyCode::NonConvert)
    }
    pub fn delete(&self) -> TurboButton {
        self.get(&TurboKeyCode::Delete)
    }
    pub fn end(&self) -> TurboButton {
        self.get(&TurboKeyCode::End)
    }
    pub fn help(&self) -> TurboButton {
        self.get(&TurboKeyCode::Help)
    }
    pub fn home(&self) -> TurboButton {
        self.get(&TurboKeyCode::Home)
    }
    pub fn insert(&self) -> TurboButton {
        self.get(&TurboKeyCode::Insert)
    }
    pub fn page_down(&self) -> TurboButton {
        self.get(&TurboKeyCode::PageDown)
    }
    pub fn page_up(&self) -> TurboButton {
        self.get(&TurboKeyCode::PageUp)
    }
    pub fn arrow_down(&self) -> TurboButton {
        self.get(&TurboKeyCode::ArrowDown)
    }
    pub fn arrow_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::ArrowLeft)
    }
    pub fn arrow_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::ArrowRight)
    }
    pub fn arrow_up(&self) -> TurboButton {
        self.get(&TurboKeyCode::ArrowUp)
    }
    pub fn num_lock(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumLock)
    }
    pub fn numpad0(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad0)
    }
    pub fn numpad1(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad1)
    }
    pub fn numpad2(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad2)
    }
    pub fn numpad3(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad3)
    }
    pub fn numpad4(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad4)
    }
    pub fn numpad5(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad5)
    }
    pub fn numpad6(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad6)
    }
    pub fn numpad7(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad7)
    }
    pub fn numpad8(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad8)
    }
    pub fn numpad9(&self) -> TurboButton {
        self.get(&TurboKeyCode::Numpad9)
    }
    pub fn numpad_add(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadAdd)
    }
    pub fn numpad_backspace(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadBackspace)
    }
    pub fn numpad_clear(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadClear)
    }
    pub fn numpad_clear_entry(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadClearEntry)
    }
    pub fn numpad_comma(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadComma)
    }
    pub fn numpad_decimal(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadDecimal)
    }
    pub fn numpad_divide(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadDivide)
    }
    pub fn numpad_enter(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadEnter)
    }
    pub fn numpad_equal(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadEqual)
    }
    pub fn numpad_hash(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadHash)
    }
    pub fn numpad_memory_add(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadMemoryAdd)
    }
    pub fn numpad_memory_clear(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadMemoryClear)
    }
    pub fn numpad_memory_recall(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadMemoryRecall)
    }
    pub fn numpad_memory_store(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadMemoryStore)
    }
    pub fn numpad_memory_subtract(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadMemorySubtract)
    }
    pub fn numpad_multiply(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadMultiply)
    }
    pub fn numpad_paren_left(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadParenLeft)
    }
    pub fn numpad_paren_right(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadParenRight)
    }
    pub fn numpad_star(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadStar)
    }
    pub fn numpad_subtract(&self) -> TurboButton {
        self.get(&TurboKeyCode::NumpadSubtract)
    }
    pub fn escape(&self) -> TurboButton {
        self.get(&TurboKeyCode::Escape)
    }
    pub fn fn_(&self) -> TurboButton {
        self.get(&TurboKeyCode::Fn)
    }
    pub fn fn_lock(&self) -> TurboButton {
        self.get(&TurboKeyCode::FnLock)
    }
    pub fn print_screen(&self) -> TurboButton {
        self.get(&TurboKeyCode::PrintScreen)
    }
    pub fn scroll_lock(&self) -> TurboButton {
        self.get(&TurboKeyCode::ScrollLock)
    }
    pub fn pause(&self) -> TurboButton {
        self.get(&TurboKeyCode::Pause)
    }
    pub fn browser_back(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserBack)
    }
    pub fn browser_favorites(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserFavorites)
    }
    pub fn browser_forward(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserForward)
    }
    pub fn browser_home(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserHome)
    }
    pub fn browser_refresh(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserRefresh)
    }
    pub fn browser_search(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserSearch)
    }
    pub fn browser_stop(&self) -> TurboButton {
        self.get(&TurboKeyCode::BrowserStop)
    }
    pub fn eject(&self) -> TurboButton {
        self.get(&TurboKeyCode::Eject)
    }
    pub fn launch_app1(&self) -> TurboButton {
        self.get(&TurboKeyCode::LaunchApp1)
    }
    pub fn launch_app2(&self) -> TurboButton {
        self.get(&TurboKeyCode::LaunchApp2)
    }
    pub fn launch_mail(&self) -> TurboButton {
        self.get(&TurboKeyCode::LaunchMail)
    }
    pub fn media_play_pause(&self) -> TurboButton {
        self.get(&TurboKeyCode::MediaPlayPause)
    }
    pub fn media_select(&self) -> TurboButton {
        self.get(&TurboKeyCode::MediaSelect)
    }
    pub fn media_stop(&self) -> TurboButton {
        self.get(&TurboKeyCode::MediaStop)
    }
    pub fn media_track_next(&self) -> TurboButton {
        self.get(&TurboKeyCode::MediaTrackNext)
    }
    pub fn media_track_previous(&self) -> TurboButton {
        self.get(&TurboKeyCode::MediaTrackPrevious)
    }
    pub fn power(&self) -> TurboButton {
        self.get(&TurboKeyCode::Power)
    }
    pub fn sleep(&self) -> TurboButton {
        self.get(&TurboKeyCode::Sleep)
    }
    pub fn audio_volume_down(&self) -> TurboButton {
        self.get(&TurboKeyCode::AudioVolumeDown)
    }
    pub fn audio_volume_mute(&self) -> TurboButton {
        self.get(&TurboKeyCode::AudioVolumeMute)
    }
    pub fn audio_volume_up(&self) -> TurboButton {
        self.get(&TurboKeyCode::AudioVolumeUp)
    }
    pub fn wake_up(&self) -> TurboButton {
        self.get(&TurboKeyCode::WakeUp)
    }
    pub fn meta(&self) -> TurboButton {
        self.get(&TurboKeyCode::Meta)
    }
    pub fn hyper(&self) -> TurboButton {
        self.get(&TurboKeyCode::Hyper)
    }
    pub fn turbo(&self) -> TurboButton {
        self.get(&TurboKeyCode::Turbo)
    }
    pub fn abort(&self) -> TurboButton {
        self.get(&TurboKeyCode::Abort)
    }
    pub fn resume(&self) -> TurboButton {
        self.get(&TurboKeyCode::Resume)
    }
    pub fn suspend(&self) -> TurboButton {
        self.get(&TurboKeyCode::Suspend)
    }
    pub fn again(&self) -> TurboButton {
        self.get(&TurboKeyCode::Again)
    }
    pub fn copy(&self) -> TurboButton {
        self.get(&TurboKeyCode::Copy)
    }
    pub fn cut(&self) -> TurboButton {
        self.get(&TurboKeyCode::Cut)
    }
    pub fn find(&self) -> TurboButton {
        self.get(&TurboKeyCode::Find)
    }
    pub fn open(&self) -> TurboButton {
        self.get(&TurboKeyCode::Open)
    }
    pub fn paste(&self) -> TurboButton {
        self.get(&TurboKeyCode::Paste)
    }
    pub fn props(&self) -> TurboButton {
        self.get(&TurboKeyCode::Props)
    }
    pub fn select(&self) -> TurboButton {
        self.get(&TurboKeyCode::Select)
    }
    pub fn undo(&self) -> TurboButton {
        self.get(&TurboKeyCode::Undo)
    }
    pub fn hiragana(&self) -> TurboButton {
        self.get(&TurboKeyCode::Hiragana)
    }
    pub fn katakana(&self) -> TurboButton {
        self.get(&TurboKeyCode::Katakana)
    }
    pub fn f1(&self) -> TurboButton {
        self.get(&TurboKeyCode::F1)
    }
    pub fn f2(&self) -> TurboButton {
        self.get(&TurboKeyCode::F2)
    }
    pub fn f3(&self) -> TurboButton {
        self.get(&TurboKeyCode::F3)
    }
    pub fn f4(&self) -> TurboButton {
        self.get(&TurboKeyCode::F4)
    }
    pub fn f5(&self) -> TurboButton {
        self.get(&TurboKeyCode::F5)
    }
    pub fn f6(&self) -> TurboButton {
        self.get(&TurboKeyCode::F6)
    }
    pub fn f7(&self) -> TurboButton {
        self.get(&TurboKeyCode::F7)
    }
    pub fn f8(&self) -> TurboButton {
        self.get(&TurboKeyCode::F8)
    }
    pub fn f9(&self) -> TurboButton {
        self.get(&TurboKeyCode::F9)
    }
    pub fn f10(&self) -> TurboButton {
        self.get(&TurboKeyCode::F10)
    }
    pub fn f11(&self) -> TurboButton {
        self.get(&TurboKeyCode::F11)
    }
    pub fn f12(&self) -> TurboButton {
        self.get(&TurboKeyCode::F12)
    }
    pub fn f13(&self) -> TurboButton {
        self.get(&TurboKeyCode::F13)
    }
    pub fn f14(&self) -> TurboButton {
        self.get(&TurboKeyCode::F14)
    }
    pub fn f15(&self) -> TurboButton {
        self.get(&TurboKeyCode::F15)
    }
    pub fn f16(&self) -> TurboButton {
        self.get(&TurboKeyCode::F16)
    }
    pub fn f17(&self) -> TurboButton {
        self.get(&TurboKeyCode::F17)
    }
    pub fn f18(&self) -> TurboButton {
        self.get(&TurboKeyCode::F18)
    }
    pub fn f19(&self) -> TurboButton {
        self.get(&TurboKeyCode::F19)
    }
    pub fn f20(&self) -> TurboButton {
        self.get(&TurboKeyCode::F20)
    }
    pub fn f21(&self) -> TurboButton {
        self.get(&TurboKeyCode::F21)
    }
    pub fn f22(&self) -> TurboButton {
        self.get(&TurboKeyCode::F22)
    }
    pub fn f23(&self) -> TurboButton {
        self.get(&TurboKeyCode::F23)
    }
    pub fn f24(&self) -> TurboButton {
        self.get(&TurboKeyCode::F24)
    }
    pub fn f25(&self) -> TurboButton {
        self.get(&TurboKeyCode::F25)
    }
    pub fn f26(&self) -> TurboButton {
        self.get(&TurboKeyCode::F26)
    }
    pub fn f27(&self) -> TurboButton {
        self.get(&TurboKeyCode::F27)
    }
    pub fn f28(&self) -> TurboButton {
        self.get(&TurboKeyCode::F28)
    }
    pub fn f29(&self) -> TurboButton {
        self.get(&TurboKeyCode::F29)
    }
    pub fn f30(&self) -> TurboButton {
        self.get(&TurboKeyCode::F30)
    }
    pub fn f31(&self) -> TurboButton {
        self.get(&TurboKeyCode::F31)
    }
    pub fn f32(&self) -> TurboButton {
        self.get(&TurboKeyCode::F32)
    }
    pub fn f33(&self) -> TurboButton {
        self.get(&TurboKeyCode::F33)
    }
    pub fn f34(&self) -> TurboButton {
        self.get(&TurboKeyCode::F34)
    }
    pub fn f35(&self) -> TurboButton {
        self.get(&TurboKeyCode::F35)
    }
}

/// Based on winit KeyCode which is based on the w3c UI Events spec.
/// See [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub enum TurboKeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}
impl TurboKeyCode {
    pub const ALL: &[Self] = &[
        Self::Backquote,
        Self::Backslash,
        Self::BracketLeft,
        Self::BracketRight,
        Self::Comma,
        Self::Digit0,
        Self::Digit1,
        Self::Digit2,
        Self::Digit3,
        Self::Digit4,
        Self::Digit5,
        Self::Digit6,
        Self::Digit7,
        Self::Digit8,
        Self::Digit9,
        Self::Equal,
        Self::IntlBackslash,
        Self::IntlRo,
        Self::IntlYen,
        Self::KeyA,
        Self::KeyB,
        Self::KeyC,
        Self::KeyD,
        Self::KeyE,
        Self::KeyF,
        Self::KeyG,
        Self::KeyH,
        Self::KeyI,
        Self::KeyJ,
        Self::KeyK,
        Self::KeyL,
        Self::KeyM,
        Self::KeyN,
        Self::KeyO,
        Self::KeyP,
        Self::KeyQ,
        Self::KeyR,
        Self::KeyS,
        Self::KeyT,
        Self::KeyU,
        Self::KeyV,
        Self::KeyW,
        Self::KeyX,
        Self::KeyY,
        Self::KeyZ,
        Self::Minus,
        Self::Period,
        Self::Quote,
        Self::Semicolon,
        Self::Slash,
        Self::AltLeft,
        Self::AltRight,
        Self::Backspace,
        Self::CapsLock,
        Self::ContextMenu,
        Self::ControlLeft,
        Self::ControlRight,
        Self::Enter,
        Self::SuperLeft,
        Self::SuperRight,
        Self::ShiftLeft,
        Self::ShiftRight,
        Self::Space,
        Self::Tab,
        Self::Convert,
        Self::KanaMode,
        Self::Lang1,
        Self::Lang2,
        Self::Lang3,
        Self::Lang4,
        Self::Lang5,
        Self::NonConvert,
        Self::Delete,
        Self::End,
        Self::Help,
        Self::Home,
        Self::Insert,
        Self::PageDown,
        Self::PageUp,
        Self::ArrowDown,
        Self::ArrowLeft,
        Self::ArrowRight,
        Self::ArrowUp,
        Self::NumLock,
        Self::Numpad0,
        Self::Numpad1,
        Self::Numpad2,
        Self::Numpad3,
        Self::Numpad4,
        Self::Numpad5,
        Self::Numpad6,
        Self::Numpad7,
        Self::Numpad8,
        Self::Numpad9,
        Self::NumpadAdd,
        Self::NumpadBackspace,
        Self::NumpadClear,
        Self::NumpadClearEntry,
        Self::NumpadComma,
        Self::NumpadDecimal,
        Self::NumpadDivide,
        Self::NumpadEnter,
        Self::NumpadEqual,
        Self::NumpadHash,
        Self::NumpadMemoryAdd,
        Self::NumpadMemoryClear,
        Self::NumpadMemoryRecall,
        Self::NumpadMemoryStore,
        Self::NumpadMemorySubtract,
        Self::NumpadMultiply,
        Self::NumpadParenLeft,
        Self::NumpadParenRight,
        Self::NumpadStar,
        Self::NumpadSubtract,
        Self::Escape,
        Self::Fn,
        Self::FnLock,
        Self::PrintScreen,
        Self::ScrollLock,
        Self::Pause,
        Self::BrowserBack,
        Self::BrowserFavorites,
        Self::BrowserForward,
        Self::BrowserHome,
        Self::BrowserRefresh,
        Self::BrowserSearch,
        Self::BrowserStop,
        Self::Eject,
        Self::LaunchApp1,
        Self::LaunchApp2,
        Self::LaunchMail,
        Self::MediaPlayPause,
        Self::MediaSelect,
        Self::MediaStop,
        Self::MediaTrackNext,
        Self::MediaTrackPrevious,
        Self::Power,
        Self::Sleep,
        Self::AudioVolumeDown,
        Self::AudioVolumeMute,
        Self::AudioVolumeUp,
        Self::WakeUp,
        Self::Meta,
        Self::Hyper,
        Self::Turbo,
        Self::Abort,
        Self::Resume,
        Self::Suspend,
        Self::Again,
        Self::Copy,
        Self::Cut,
        Self::Find,
        Self::Open,
        Self::Paste,
        Self::Props,
        Self::Select,
        Self::Undo,
        Self::Hiragana,
        Self::Katakana,
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
        Self::F13,
        Self::F14,
        Self::F15,
        Self::F16,
        Self::F17,
        Self::F18,
        Self::F19,
        Self::F20,
        Self::F21,
        Self::F22,
        Self::F23,
        Self::F24,
        Self::F25,
        Self::F26,
        Self::F27,
        Self::F28,
        Self::F29,
        Self::F30,
        Self::F31,
        Self::F32,
        Self::F33,
        Self::F34,
        Self::F35,
    ];
    pub fn all() -> &'static [Self] {
        Self::ALL
    }
}
