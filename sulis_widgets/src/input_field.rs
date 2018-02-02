//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

use sulis_core::resource::ResourceSet;
use sulis_core::image::Image;
use sulis_core::ui::{LineRenderer, Widget, WidgetKind};
use sulis_core::io::{GraphicsRenderer};
use sulis_core::util::Point;
use Label;

const NAME: &str = "input_field";

pub struct InputField {
    pub text: String,
    label: Rc<RefCell<Label>>,
    carat: Option<Rc<Image>>,
    carat_width: f32,
    carat_height: f32,
    carat_offset: f32,
}

impl InputField {
    pub fn new(text: &str) -> Rc<RefCell<InputField>> {
        Rc::new(RefCell::new(InputField {
            text: text.to_string(),
            label: Label::new(text),
            carat: None,
            carat_width: 1.0,
            carat_height: 1.0,
            carat_offset: 0.0,
        }))
    }
}

impl WidgetKind for InputField {
    fn get_name(&self) -> &str { NAME }
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }

    fn wants_keyboard_focus(&self) -> bool { true }

    fn layout(&mut self, widget: &mut Widget) {
        if let Some(ref text) = self.label.borrow().text {
            widget.state.add_text_arg("0", text);
        }
        widget.do_base_layout();

        if let Some(ref font) = widget.state.font {
            widget.state.text_renderer = Some(Box::new(LineRenderer::new(font)));
        }

        if let Some(ref theme) = widget.theme {
            if let Some(ref image_id) = theme.custom.get("carat_image") {
                self.carat = ResourceSet::get_image(image_id);
            }

            self.carat_width = theme.get_custom_or_default("carat_width", 1.0);
            self.carat_height = theme.get_custom_or_default("carat_height", 1.0);
            self.carat_offset = theme.get_custom_or_default("carat_offset", 0.0);
        }
    }

    fn on_char_typed(&mut self, widget: &Rc<RefCell<Widget>>, c: char) -> bool {
        match c {
            '\u{8}' => {
                self.text.pop();
            }, _ => {
                if self.label.borrow().text_draw_end_x >
                    widget.borrow().state.inner_right() as f32 {
                    return true;
                }

                self.text.push(c);
            }
        }

        self.label.borrow_mut().text = Some(self.text.clone());
        widget.borrow_mut().invalidate_layout();
        Widget::fire_callback(widget, self);
        true
    }

    fn draw_graphics_mode(&mut self, renderer: &mut GraphicsRenderer, pixel_size: Point,
                          widget: &Widget, millis: u32) {
        self.label.borrow_mut().draw_graphics_mode(renderer, pixel_size, widget, millis);

        if !widget.state.has_keyboard_focus() { return; }

        if let Some(ref carat) = self.carat {
            let x = self.label.borrow().text_draw_end_x + self.carat_offset;
            let y = widget.state.inner_top() as f32 +
                (widget.state.inner_size.height as f32 - self.carat_height) / 2.0;
            let w = self.carat_width;
            let h = self.carat_height;
            carat.draw_graphics_mode(renderer, &widget.state.animation_state, x, y, w, h, millis);
        }
    }
}