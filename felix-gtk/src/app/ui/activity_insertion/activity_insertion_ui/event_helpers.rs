/// If true, increase the duration.
/// If fasle, decrease the duration.
/// If None, do nothing.
#[must_use]
pub(super) fn increase_duration_on_scroll(event: &gdk::EventScroll) -> Option<bool> {
    match event.get_direction() {
        gdk::ScrollDirection::Up => Some(true),
        gdk::ScrollDirection::Down => Some(false),
        gdk::ScrollDirection::Smooth => {
            // Use x and y to deduce scroll direction
            let (dx, dy) = event.get_delta();
            if dx.abs() > dy.abs() {
                // Horizontal scroll
                None
            } else {
                Some(dy < 0.0)
            }
        }
        // Horizontal scroll
        _ => None,
    }
}
