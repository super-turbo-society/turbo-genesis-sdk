//--------------------------------------------------------------------------
// sprite!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__sprite__ as sprite;

#[doc(hidden)]
#[macro_export]
macro_rules! __sprite__ {
    (animation_key = $name:expr, default_sprite = $default_sprite:expr $(, $key:ident = $val:expr )* $(,)?) => {{
        // 1. Make a sprite with the given name
        let name = $name;
        let anim = $crate::canvas::animation::get(name);
        if anim.done() {
            anim.use_sprite($default_sprite);
        }
        let mut sprite = anim.sprite();
        // 2. For each key-value pair, call the corresponding method on the sprite.
        $(sprite = $crate::__sprite__!(@set sprite, $key, $val);)*
        // 3. Draw it!
        sprite.draw();
    }};
    (animation_key = $name:expr $(, $key:ident = $val:expr )* $(,)?) => {{
        // 1. Make a sprite with the given name
        let name = $name;
        let mut sprite = $crate::canvas::animation::get(name).sprite();
        // 2. For each key-value pair, call the corresponding method on the sprite.
        $(sprite = $crate::__sprite__!(@set sprite, $key, $val);)*
        // 3. Draw it!
        sprite.draw();
    }};
    (animation = $anim:expr $(, $key:ident = $val:expr ),* $(,)?) => {{
        // 1. Make a sprite with the given name
        let mut sprite = $anim.sprite();
        // 2. For each key-value pair, call the corresponding method on the sprite.
        $(sprite = $crate::__sprite__!(@set sprite, $key, $val);)*
        // 3. Draw it!
        sprite.draw();
    }};
    ($name:expr $(, $key:ident = $val:expr )* $(,)?) => {{
        // 1. Make a sprite with the given name
        let name = $name;
        let mut sprite = $crate::canvas::sprite(name);
        // 2. For each key-value pair, call the corresponding method on the sprite.
        $(sprite = $crate::__sprite__!(@set sprite, $key, $val);)*
        // 3. Draw it!
        sprite.draw();
    }};
    (@set $sprite:ident, x, $val:expr) => { $sprite.position_x($val) };
    (@set $sprite:ident, y, $val:expr) => { $sprite.position_y($val) };
    (@set $sprite:ident, xy, $val:expr) => { $sprite.position_xy($val) };
    (@set $sprite:ident, position, $val:expr) => { $sprite.position_xy($val) };
    (@set $sprite:ident, tx, $val:expr) => { $sprite.tex_position_x($val) };
    (@set $sprite:ident, ty, $val:expr) => { $sprite.tex_position_y($val) };
    (@set $sprite:ident, w, $val:expr) => { $sprite.width($val) };
    (@set $sprite:ident, h, $val:expr) => { $sprite.height($val) };
    (@set $sprite:ident, wh, $val:expr) => { $sprite.size_wh($val) };
    (@set $sprite:ident, size, $val:expr) => { $sprite.size_wh($val) };
    (@set $sprite:ident, origin, $val:expr) => { $sprite.origin_xy($val) };
    (@set $sprite:ident, rotation, $val:expr) => { $sprite.rotation_deg($val) };
    (@set $sprite:ident, bounds, $val:expr) => { $sprite.position_xy($val.xy()).size_wh($val.wh()) };
    (@set $sprite:ident, $key:ident, $val:expr) => { $sprite.$key($val) };
}

//--------------------------------------------------------------------------
// nine_slice!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__nine_slice__ as nine_slice;

#[doc(hidden)]
#[macro_export]
macro_rules! __nine_slice__ {
    ($name:expr, margins = $margins:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a nine-slice with the given name
        let mut nine_slice = $crate::canvas::nine_slice($name, $margins);
        // 2. For each key-value pair, call the corresponding method on the nine_slice.
        $(nine_slice = $crate::__nine_slice__!(@set nine_slice, $key, $val);)*
        // 3. Draw it!
        nine_slice.draw();
    }};
    (@set $nine_slice:ident, x, $val:expr) => { $nine_slice.position_x($val) };
    (@set $nine_slice:ident, y, $val:expr) => { $nine_slice.position_y($val) };
    (@set $nine_slice:ident, xy, $val:expr) => { $nine_slice.position_xy($val) };
    (@set $nine_slice:ident, position, $val:expr) => { $nine_slice.position_xy($val) };
    (@set $nine_slice:ident, tx, $val:expr) => { $nine_slice.tex_position_x($val) };
    (@set $nine_slice:ident, ty, $val:expr) => { $nine_slice.tex_position_y($val) };
    (@set $nine_slice:ident, w, $val:expr) => { $nine_slice.width($val) };
    (@set $nine_slice:ident, h, $val:expr) => { $nine_slice.height($val) };
    (@set $nine_slice:ident, wh, $val:expr) => { $nine_slice.size_wh($val) };
    (@set $nine_slice:ident, size, $val:expr) => { $nine_slice.size_wh($val) };
    (@set $nine_slice:ident, origin, $val:expr) => { $nine_slice.origin_xy($val) };
    (@set $nine_slice:ident, rotation, $val:expr) => { $nine_slice.rotation_deg($val) };
    (@set $nine_slice:ident, bounds, $val:expr) => { $nine_slice.position_xy($val.xy()).size_wh($val.wh()) };
    (@set $nine_slice:ident, $key:ident, $val:expr) => { $nine_slice.$key($val) };
}

//--------------------------------------------------------------------------
// rect!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__rect__ as rect;

#[doc(hidden)]
#[macro_export]
macro_rules! __rect__ {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a rect
        let mut rect = $crate::canvas::rect::Rectangle::new();
        // 2. For each key-value pair, call the corresponding method on the rect.
        $(rect = $crate::__rect__!(@set rect, $key, $val);)*
        // 3. Draw it!
        rect.draw();
    }};
    (@set $rect:ident, x, $val:expr) => { $rect.position_x($val) };
    (@set $rect:ident, y, $val:expr) => { $rect.position_y($val) };
    (@set $rect:ident, xy, $val:expr) => { $rect.position_xy($val) };
    (@set $rect:ident, position, $val:expr) => { $rect.position_xy($val) };
    (@set $rect:ident, w, $val:expr) => { $rect.width($val) };
    (@set $rect:ident, h, $val:expr) => { $rect.height($val) };
    (@set $rect:ident, wh, $val:expr) => { $rect.size_wh($val) };
    (@set $rect:ident, size, $val:expr) => { $rect.size_wh($val) };
    (@set $rect:ident, origin, $val:expr) => { $rect.origin_xy($val) };
    (@set $rect:ident, rotation, $val:expr) => { $rect.rotation_deg($val) };
    (@set $rect:ident, bounds, $val:expr) => { $rect.position_xy($val.xy()).size_wh($val.wh()) };
    (@set $rect:ident, $key:ident, $val:expr) => { $rect.$key($val) };
}

//--------------------------------------------------------------------------
// ellipse!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__ellipse__ as ellipse;

#[doc(hidden)]
#[macro_export]
macro_rules! __ellipse__ {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a rect
        let mut ellipse = $crate::canvas::ellipse::Ellipse::new();
        // 2. For each key-value pair, call the corresponding method on the ellipse.
        $(ellipse = $crate::__ellipse__!(@set ellipse, $key, $val);)*
        // 3. Draw it!
        ellipse.draw();
    }};
    (@set $ellipse:ident, x, $val:expr) => { $ellipse.position_x($val) };
    (@set $ellipse:ident, y, $val:expr) => { $ellipse.position_y($val) };
    (@set $ellipse:ident, xy, $val:expr) => { $ellipse.position_xy($val) };
    (@set $ellipse:ident, position, $val:expr) => { $ellipse.position_xy($val) };
    (@set $ellipse:ident, w, $val:expr) => { $ellipse.width($val) };
    (@set $ellipse:ident, h, $val:expr) => { $ellipse.height($val) };
    (@set $ellipse:ident, wh, $val:expr) => { $ellipse.size_wh($val) };
    (@set $ellipse:ident, size, $val:expr) => { $ellipse.size_wh($val) };
    (@set $ellipse:ident, origin, $val:expr) => { $ellipse.origin_xy($val) };
    (@set $ellipse:ident, rotation, $val:expr) => { $ellipse.rotation_deg($val) };
    (@set $ellipse:ident, bounds, $val:expr) => { $ellipse.position_xy($val.xy()).size_wh($val.wh()) };
    (@set $ellipse:ident, $key:ident, $val:expr) => { $ellipse.$key($val) };
}

//--------------------------------------------------------------------------
// circ!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__circ__ as circ;

#[doc(hidden)]
#[macro_export]
macro_rules! __circ__ {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a circ
        let mut circ = $crate::canvas::circ::Circle::new();
        // 2. For each key-value pair, call the corresponding method on the circ.
        $(circ = $crate::__circ__!(@set circ, $key, $val);)*
        // 3. Draw it!
        circ.draw();
    }};
    (@set $circ:ident, x, $val:expr) => { $circ.position_x($val) };
    (@set $circ:ident, y, $val:expr) => { $circ.position_y($val) };
    (@set $circ:ident, xy, $val:expr) => { $circ.position_xy($val) };
    (@set $circ:ident, position, $val:expr) => { $circ.position_xy($val) };
    (@set $circ:ident, d, $val:expr) => { $circ.diameter($val) };
    (@set $circ:ident, origin, $val:expr) => { $circ.origin_xy($val) };
    (@set $circ:ident, bounds, $val:expr) => { $circ.position_xy($val.xy()).diameter($val.w().max($val.h())) };
    (@set $circ:ident, $key:ident, $val:expr) => { $circ.$key($val) };
}

//--------------------------------------------------------------------------
// line!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__path__ as path;

#[doc(hidden)]
#[macro_export]
macro_rules! __path__ {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a line
        let mut path = $crate::canvas::path::Path::new();
        // 2. For each key-value pair, call the corresponding method on the line.
        $(path = $crate::__path__!(@set path, $key, $val);)*
        // 3. Draw it!
        path.draw();
    }};
    (@set $path:ident, start_x, $val:expr) => { $path.start_position_x($val) };
    (@set $path:ident, start_y, $val:expr) => { $path.start_position_y($val) };
    (@set $path:ident, start, $val:expr) => { $path.start_position_xy($val) };
    (@set $path:ident, end_x, $val:expr) => { $path.end_position_x($val) };
    (@set $path:ident, end_y, $val:expr) => { $path.end_position_y($val) };
    (@set $path:ident, end, $val:expr) => { $path.end_position_xy($val) };
    (@set $path:ident, w, $val:expr) => { $path };
    (@set $path:ident, origin, $val:expr) => { $path.origin_xy($val) };
    (@set $path:ident, rotation, $val:expr) => { $path.rotation_deg($val) };
    (@set $path:ident, $key:ident, $val:expr) => { $path.$key($val) };
}

//--------------------------------------------------------------------------
// text!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__text__ as text;

#[doc(hidden)]
#[macro_export]
macro_rules! __text__ {
    // Interpolation + key-value pairs.
    ($string:expr, $( $arg:expr ),* ; $( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a text
        let string = format!($string, $($arg),*);
        let mut text = $crate::canvas::text::Text::new(&string);
        // 2. For each key-value pair, call the corresponding method on the text.
        $(text = $crate::__text__!(@set text, $key, $val);)*
        // 3. Draw it!
        text.draw();
    }};
    // No interpolation + key-value pairs.
    ($string:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
        // 1. Make a text
        let mut text = $crate::canvas::text::Text::new($string);
        // 2. For each key-value pair, call the corresponding method on the text.
        $(text = $crate::__text__!(@set text, $key, $val);)*
        // 3. Draw it!
        text.draw();
    }};
    // Interpolation + no key-value pairs.
    ($string:expr, $( $arg:expr ),*$(,)*) => {{
        // 1. Make a text
        let string = format!($string, $($arg),*);
        let mut text = $crate::canvas::text::Text::new(&string);
        // 2. Draw it!
        text.draw();
    }};
    // No interpolation + no key-value pairs.
    ($string:expr) => {{
        // 1. Make a text
        let mut text = $crate::canvas::text::Text::new($string);
        // 2. Draw it!
        text.draw();
    }};
    (@set $text:ident, x, $val:expr) => { $text.position_x($val) };
    (@set $text:ident, y, $val:expr) => { $text.position_y($val) };
    (@set $text:ident, xy, $val:expr) => { $text.position_xy($val) };
    (@set $text:ident, position, $val:expr) => { $text.position_xy($val) };
    (@set $text:ident, rotation, $val:expr) => { $text.rotation_deg($val) };
    (@set $text:ident, $key:ident, $val:expr) => { $text.$key($val) };
}

//--------------------------------------------------------------------------
// text_box!
//--------------------------------------------------------------------------

#[doc(inline)]
pub use crate::__text_box__ as text_box;

#[doc(hidden)]
#[macro_export]
macro_rules! __text_box__ {
    ($text:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
        let mut tb = $crate::canvas::text_box::TextBox::new($text);
        $(tb = $crate::__text_box__!(@set tb, $key, $val);)*
        tb.draw();
    }};
    ($text:expr) => {{
        let mut tb = $crate::canvas::text_box::TextBox::new($text);
        tb.draw();
    }};
    (@set $tb:ident, align, $val:expr) => {{
        use $crate::canvas::text_box::Align;
        $tb.align(Align::from_str($val).unwrap_or(Align::Left))
    }};
    (@set $tb:ident, x, $val:expr) => { $tb.position_x($val) };
    (@set $tb:ident, y, $val:expr) => { $tb.position_y($val) };
    (@set $tb:ident, xy, $val:expr) => { $tb.position_xy($val) };
    (@set $tb:ident, position, $val:expr) => { $tb.position_xy($val) };
    (@set $tb:ident, w, $val:expr) => { $tb.width($val) };
    (@set $tb:ident, h, $val:expr) => { $tb.height($val) };
    (@set $tb:ident, wh, $val:expr) => { $tb.size_wh($val) };
    (@set $tb:ident, size, $val:expr) => { $tb.size_wh($val) };
    (@set $tb:ident, rotation, $val:expr) => { $tb.rotation_deg($val) };
    (@set $tb:ident, bounds, $val:expr) => { $tb.position_xy($val.xy()).size_wh($val.wh()) };
    (@set $tb:ident, $key:ident, $val:expr) => { $tb.$key($val) };
}
