use floem::event::EventListener;
use floem::peniko::Color;
use floem::reactive::{create_rw_signal, RwSignal};
// use floem::{h_stack, Decorators, label, scroll, v_stack, virtual_list, VirtualDirection, VirtualItemSize, stack};
use floem::view::View;
use floem::views::{Decorators,label, scroll, stack, virtual_list, VirtualDirection, VirtualItemSize};
use floem::widgets::button;
use im::{Vector, vector};

pub fn app_view() -> impl View {
  pub const LIST_WIDTH: f64 = 300.0;

  let actions_list_signal: RwSignal<Vector<String>> = create_rw_signal(Vector::new());
  let actions_list_signal_ = actions_list_signal.clone();
  let actions_list_signal__ = actions_list_signal.clone();
  let view = stack((
    scroll({
      virtual_list(
        VirtualDirection::Vertical,
        VirtualItemSize::Fixed(Box::new(|| 22.0)),
        move || actions_list_signal_.get(),
        move |command| command.clone(),
        move |command| {
          label(move || command.clone()).style(move |s| {
            s.padding(10.0)
              .padding_top(3.0)
              .padding_bottom(3.0)
              .width(300.)
              .items_start()
              .border_bottom(1.0)
              .border_color(Color::rgb8(205, 205, 205))
          })
        },
      )
        .style(|s| s.flex_col().width(LIST_WIDTH - 1.0)
          .flex_grow(0.5)
        )
    })
      .style(|s| {
        s.width(200.0)
          .min_height(240.0)
          .border_left(1.0)
          .border_top(1.0)
          .border_bottom(1.0)
          .border_color(Color::DARK_GRAY)
      }),
    stack((
      button(|| "A").style(|s| s.padding(5.0).width_full())
        .on_click_stop(move |_| {
          actions_list_signal__.update(|actions| actions.push_back("A".to_string()));
        })
        .style(|s| s.padding(5.0)
          .width(20.0)
          .height(20.0)
        ),
      ))

  ));

  let id = view.id();
  view.on_event_stop(EventListener::KeyUp, move |e| {
    if let floem::event::Event::KeyUp(e) = e {
      if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::NumLock) {
        id.inspect();
      }
    }
  })

}

fn main() {
  floem::launch(app_view);
}
