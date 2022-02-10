
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, Background, Color, Element, Hasher, Layout, Length,
    Point, Rectangle, Size, Widget,
};

pub struct ScrollableH {
    radius: f32,
}

impl ScrollableH {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for ScrollableH
where
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer<B>,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        self.radius.to_bits().hash(state);
    }

    fn draw(
        &self,
        _renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::BLACK),
                border_radius: self.radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for ScrollableH
where
    B: Backend,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}
