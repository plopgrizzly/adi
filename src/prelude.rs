//! Automatically import traits with `use cala::prelude::*;`.

#[cfg(feature = "draw")]
/// **feature:draw** -
pub use pix::{el::Pixel, ops::Blend as PixelBlend};

#[cfg(feature = "draw")]
/// **feature:draw** - Something that can be drawn on.
pub trait Canvas {
    /// Draw a group on the screen.
    fn draw(
        &mut self,
        shader: &crate::draw::Shader,
        group: &crate::draw::Group,
    );
    /// Set camera for shader.
    fn set_camera(&mut self, camera: crate::draw::Transform);
    /// Set tint for shader.
    fn set_tint<P: pix::el::Pixel>(
        &mut self,
        shader: &crate::draw::Shader,
        tint: P,
    ) where
        pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>;
    /// Draw a group with a texture on the screen.
    fn draw_graphic(
        &mut self,
        shader: &crate::draw::Shader,
        group: &crate::draw::Group,
        graphic: &crate::draw::Texture,
    );
    /// Returns the amount of time elapsed since the previous frame.
    fn elapsed(&self) -> std::time::Duration;
    /// Return the aspect ratio (`height / width`) of the `Canvas`.
    fn height(&self) -> f32;
    /// Returns true if the canvas has changed size since the last redraw.
    fn resized(&self) -> bool;
}

#[cfg(feature = "exec")]
/// **feature:exec** -
pub use pasts::{join, wait, exec, race};
