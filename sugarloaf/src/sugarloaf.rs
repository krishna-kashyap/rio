use crate::components::core::{image::Handle, shapes::Rectangle};
use crate::components::layer::{self, LayerBrush};
use crate::components::rect::{Rect, RectBrush};
use crate::components::text;

use crate::context::Context;
use crate::core::{ImageProperties, RepeatedSugar, Sugar, SugarStack};
use crate::font::fonts::{SugarloafFont, SugarloafFonts};
#[cfg(not(target_arch = "wasm32"))]
use crate::font::loader::Database;
use crate::layout::SugarloafLayout;
use core::fmt::{Debug, Formatter};
use cosmic_text::{Edit, Style, Weight};

#[cfg(target_arch = "wasm32")]
pub struct Database;

pub trait Renderable: 'static + Sized {
    fn init(context: &Context) -> Self;
    fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    );
    fn update(&mut self, event: winit::event::WindowEvent);
    fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        dimensions: (u32, u32),
        instances: &[Rect],
        context: &mut Context,
    );
}

pub struct Sugarloaf<'a> {
    pub ctx: Context,
    pub layout: SugarloafLayout,
    text_brush: text::TextRenderer,
    rect_brush: RectBrush,
    layer_brush: LayerBrush,
    rects: Vec<Rect>,
    fonts: SugarloafFonts,
    cache: cosmic_text::SwashCache,
    atlas: text::TextAtlas,
    spans: Vec<(String, text::Attrs<'a>)>,
    editor: cosmic_text::Editor,
    font_system: cosmic_text::FontSystem,
}

#[derive(Debug)]
pub struct SugarloafErrors {
    pub fonts_not_found: Vec<SugarloafFont>,
}

pub struct SugarloafWithErrors<'a> {
    pub instance: Sugarloaf<'a>,
    pub errors: SugarloafErrors,
}

impl Debug for SugarloafWithErrors<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.errors)
    }
}

impl<'a> Sugarloaf<'a> {
    pub async fn new(
        winit_window: &winit::window::Window,
        power_preference: wgpu::PowerPreference,
        fonts: SugarloafFonts,
        layout: SugarloafLayout,
        #[allow(unused)] db: Option<&'a Database>,
    ) -> Result<Sugarloaf<'a>, SugarloafWithErrors<'a>> {
        let ctx = Context::new(winit_window, power_preference).await;

        let mut font_system = cosmic_text::FontSystem::new();
        let cache = cosmic_text::SwashCache::new();
        let mut atlas = text::TextAtlas::new(&ctx);
        let text_brush = text::TextRenderer::new(&mut atlas, &ctx);
        let mut editor = cosmic_text::Editor::new(cosmic_text::Buffer::new_empty(
            cosmic_text::Metrics::new(20.0, 20.0).scale(ctx.scale),
        ));

        editor.borrow_with(&mut font_system);

        editor
            .buffer_mut()
            .set_size(&mut font_system, layout.width, layout.height);

        let rect_brush = RectBrush::init(&ctx);
        let layer_brush = LayerBrush::new(&ctx);

        let instance = Sugarloaf {
            layer_brush,
            fonts,
            ctx,
            rect_brush,
            rects: vec![],
            text_brush,
            cache,
            atlas,
            layout,
            editor,
            font_system,
            spans: vec![],
        };

        Ok(instance)
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        match self.ctx.surface.get_current_texture() {
            Ok(frame) => {
                let mut encoder = self.ctx.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None },
                );

                let view = &frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("sugarloaf::init -> Clear frame"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.layout.background_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                self.ctx.staging_belt.finish();
                self.ctx.queue.submit(Some(encoder.finish()));
                frame.present();
                self.ctx.staging_belt.recall();
            }
            Err(error) => {
                if error == wgpu::SurfaceError::OutOfMemory {
                    panic!("Swapchain error: {error}. Rendering cannot continue.")
                }
            }
        }
    }

    // #[inline]
    // pub fn update_font(
    //     &mut self,
    //     fonts: SugarloafFonts,
    //     #[allow(unused)] db: Option<&Database>,
    // ) -> Option<SugarloafErrors> {
    //     if self.fonts != fonts {
    //         log::info!("requested a font change");

    //         #[cfg(not(target_arch = "wasm32"))]
    //         let loaded_fonts = Font::new(fonts.to_owned(), db);
    //         #[cfg(target_arch = "wasm32")]
    //         let loaded_fonts = Font::new(fonts.to_owned());

    //         let fonts_not_found = loaded_fonts.1;
    //         if !fonts_not_found.is_empty() {
    //             return Some(SugarloafErrors { fonts_not_found });
    //         }

    //         let font = loaded_fonts.0;
    //         let is_monospace = font.text.is_monospace;

    //         // Clean font cache per instance
    //         self.sugar_cache = HashMap::new();

    //         let text_brush = text::GlyphBrushBuilder::using_fonts(vec![
    //             font.text.regular,
    //             font.text.italic,
    //             font.text.bold,
    //             font.text.bold_italic,
    //             font.symbol,
    //             font.emojis,
    //             font.unicode,
    //             font.icons,
    //             font.breadcrumbs,
    //         ])
    //         .build(&self.ctx.device, self.ctx.format);
    //         self.text_brush = text_brush;
    //         self.fonts = fonts;
    //         self.is_text_monospaced = is_monospace;
    //     }

    //     None
    // }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) -> &mut Self {
        self.ctx.resize(width, height);
        self.layout.resize(width, height).update();
        self
    }

    #[inline]
    pub fn rescale(&mut self, scale: f32) -> &mut Self {
        self.ctx.scale = scale;
        self.layout.rescale(scale).update();
        self
    }

    // #[inline]
    // pub fn stack(&mut self, stack: SugarStack) {
    //     let attrs = text::Attrs::new();
    //     let comic_attrs = attrs.family(cosmic_text::Family::Name("Fira Code"));

    //     self.spans = vec![
    //         ("-->> --- ", comic_attrs),
    //         ("B", attrs.weight(Weight::BOLD)),
    //         ("ol->d ", attrs),
    //         ("I", attrs.style(Style::Italic)),
    //     ];
    // }

    #[inline]
    pub fn stack(&mut self, stack: SugarStack) {
        // let mut x = 0.;
        let attrs = text::Attrs::new();
        let attr = attrs.family(cosmic_text::Family::Name("Fira Code"));

        let mut repeated = RepeatedSugar::new(0);
        let size = stack.len();
        for i in 0..size {
            // let mut sugar_char_width = 1.;
            // let rect_pos_x = self.layout.style.screen_position.0 + x;

            if i < size - 1
                && stack[i].content == stack[i + 1].content
                && stack[i].foreground_color == stack[i + 1].foreground_color
                && stack[i].background_color == stack[i + 1].background_color
                && stack[i].decoration.is_none()
                && stack[i + 1].decoration.is_none()
            {
                repeated.set(&stack[i]);
                // x += add_pos_x;
                continue;
            }

            repeated.set_reset_on_next();

            let mut quantity = 1;
            if repeated.count() > 0 {
                quantity += repeated.count();
            }

            let sugar_str = if quantity > 1 {
                repeated.content_str.to_owned()
            } else {
                stack[i].content.to_string()
            };

            let fg_color = if quantity > 1 {
                repeated.foreground_color
            } else {
                stack[i].foreground_color
            };

            // let bg_color = if quantity > 1 {
            //     repeated.background_color
            // } else {
            //     stack[i].background_color
            // };

            // attr.family(cosmic_text::Family::Name("Fira Code"));
            // let comic_attrs = attrs.family(Family::Name("Fira Code"));
            attr.color(cosmic_text::Color::rgb(1, 1, 1));

            if let Some(style) = &stack[i].style {
                if style.is_bold_italic {
                    self.spans.push((
                        sugar_str,
                        attr.weight(cosmic_text::Weight::BOLD).style(Style::Italic),
                    ));
                } else if style.is_bold {
                    self.spans
                        .push((sugar_str, attr.weight(cosmic_text::Weight::BOLD)));
                } else if style.is_italic {
                    self.spans.push((sugar_str, attr.style(Style::Italic)));
                }
            } else {
                self.spans.push((sugar_str, attr));
            }

            // let section_pos_x = if quantity > 1 {
            //     repeated.pos_x
            // } else {
            //     rect_pos_x
            // };

            self.rects.push(Rect {
                position: [0., 0.],
                // color: bg_color,
                color: stack[i].background_color,
                size: [10. * quantity as f32, self.layout.sugarheight],
            });

            if let Some(decoration) = &stack[i].decoration {
                // TODO:
                //  let dec_position_y = match decoration.position.1 {
                //     SugarDecorationPositionY::Bottom(pos_decoration_y) => {
                //         scaled_rect_pos_y + ((pos_decoration_y) * self.ctx.scale)
                //     }
                //     SugarDecorationPositionY::Top(pos_decoration_y) => {
                //         scaled_rect_pos_y + pos_decoration_y
                //     }
                //     SugarDecorationPositionY::Middle(pos_decoration_y) => {
                //         scaled_rect_pos_y + (self.layout.sugarheight / 2.0) + pos_decoration_y
                //     }
                // };

                // let dec_pos_y = (scaled_rect_pos_y)
                let dec_pos_y =
                    (10.) + (decoration.relative_position.1 * self.layout.line_height);
                // A decoration with is_content_positioned has the width and height based on font_size
                // and in this way is not affected by line_height (useful for decorations like Block and Beam)
                // if decoration.is_content_positioned {
                //     self.rects.push(Rect {
                //         position: [
                //             (scaled_rect_pos_x
                //                 + (add_pos_x * decoration.relative_position.0)
                //                     / self.ctx.scale),
                //             scaled_rect_pos_y,
                //         ],
                //         color: decoration.color,
                //         size: [
                //             (width_bound * decoration.size.0),
                //             (self.layout.font_size) + decoration.size.1,
                //         ],
                //     });
                // } else {
                self.rects.push(Rect {
                    position: [
                        10.,
                        // (scaled_rect_pos_x
                        //     + (add_pos_x * decoration.relative_position.0)
                        //         / self.ctx.scale),
                        dec_pos_y,
                    ],
                    color: decoration.color,
                    size: [
                        (10.),
                        // (width_bound * decoration.size.0),
                        (self.layout.sugarheight) * decoration.size.1,
                    ],
                });
                // }
            }

            if repeated.reset_on_next() {
                repeated.reset();
            }
        }
        self.spans.push(("\n".to_string(), attr));
        let spans: Vec<(&str, text::Attrs<'a>)> =
            self.spans.iter().map(|v| (v.0.as_str(), v.1)).collect();
        self.editor.buffer_mut().set_rich_text(
            &mut self.font_system,
            spans,
            cosmic_text::Shaping::Advanced,
        );
    }

    #[inline]
    pub fn get_context(&self) -> &Context {
        &self.ctx
    }

    #[inline]
    pub fn get_scale(&self) -> f32 {
        self.ctx.scale
    }

    #[inline]
    pub fn set_background_color(&mut self, color: wgpu::Color) -> &mut Self {
        self.layout.background_color = color;
        self
    }

    #[inline]
    pub fn set_background_image(&mut self, image: &ImageProperties) -> &mut Self {
        let handle = Handle::from_path(image.path.to_owned());
        self.layout.background_image = Some(layer::types::Image::Raster {
            handle,
            bounds: Rectangle {
                width: image.width,
                height: image.height,
                x: image.x,
                y: image.y,
            },
        });
        self
    }

    #[inline]
    pub fn pile_rects(&mut self, mut instances: Vec<Rect>) -> &mut Self {
        self.rects.append(&mut instances);
        self
    }

    // #[inline]
    // pub fn text(
    //     &mut self,
    //     pos: (f32, f32),
    //     text_str: String,
    //     font_id_usize: usize,
    //     scale: f32,
    //     color: [f32; 4],
    //     single_line: bool,
    // ) -> &mut Self {
    //     let font_id = FontId(font_id_usize);

    //     let text = crate::components::text::Text {
    //         text: &text_str,
    //         scale: PxScale::from(scale * self.ctx.scale),
    //         font_id,
    //         extra: crate::components::text::Extra { color, z: 0.0 },
    //     };

    //     let layout = if single_line {
    //         glyph_brush::Layout::default_single_line()
    //             .v_align(glyph_brush::VerticalAlign::Center)
    //             .h_align(glyph_brush::HorizontalAlign::Left)
    //     } else {
    //         glyph_brush::Layout::default()
    //             .v_align(glyph_brush::VerticalAlign::Center)
    //             .h_align(glyph_brush::HorizontalAlign::Left)
    //     };

    //     let section = &crate::components::text::Section {
    //         screen_position: (pos.0 * self.ctx.scale, pos.1 * self.ctx.scale),
    //         bounds: (self.layout.width, self.layout.height),
    //         text: vec![text],
    //         layout,
    //     };

    //     self.text_brush.queue(section);
    //     self
    // }

    #[inline]
    pub fn render(&mut self) {
        self.editor
            .buffer_mut()
            .shape_until_scroll(&mut self.font_system);
        self.text_brush
            .prepare(
                &self.ctx.device,
                &self.ctx.queue,
                &mut self.font_system,
                &mut self.atlas,
                text::Resolution {
                    width: self.ctx.size.width,
                    height: self.ctx.size.height,
                },
                [text::TextArea {
                    buffer: &self.editor.buffer(),
                    left: 10.0,
                    top: 10.0,
                    scale: self.ctx.scale,
                    bounds: None,
                    default_color: cosmic_text::Color::rgb(255, 255, 255),
                }],
                &mut self.cache,
            )
            .unwrap();

        match self.ctx.surface.get_current_texture() {
            Ok(frame) => {
                let mut encoder = self.ctx.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None },
                );

                let view = &frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("sugarloaf::render -> Clear frame"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.layout.background_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                if let Some(bg_image) = &self.layout.background_image {
                    self.layer_brush.prepare_ref(
                        &mut encoder,
                        &mut self.ctx,
                        &[bg_image],
                    );

                    self.layer_brush
                        .render_with_encoder(0, view, &mut encoder, None);
                }

                self.rect_brush.render(
                    &mut encoder,
                    view,
                    (self.ctx.size.width, self.ctx.size.height),
                    &self.rects,
                    &mut self.ctx,
                );

                self.rects = vec![];
                // self.text_brush.render(&self.atlas, &mut encoder, view);
                self.spans = vec![];

                self.ctx.queue.submit(Some(encoder.finish()));
                frame.present();
                self.atlas.trim();
            }
            Err(error) => {
                if error == wgpu::SurfaceError::OutOfMemory {
                    panic!("Swapchain error: {error}. Rendering cannot continue.")
                }
            }
        }
    }
}
