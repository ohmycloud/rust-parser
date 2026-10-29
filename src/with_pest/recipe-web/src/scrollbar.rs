/// The visible thumb stays compact regardless of the content/viewport ratio.
pub const THUMB_LENGTH: f64 = 20.0;

pub fn thumb_offset(position: f64, maximum: f64, track_length: f64) -> f64 {
    if maximum <= 0.0 {
        return 0.0;
    }
    (position / maximum).clamp(0.0, 1.0) * (track_length - THUMB_LENGTH).max(0.0)
}

pub fn scroll_position(pointer: f64, grab_offset: f64, track_length: f64, maximum: f64) -> f64 {
    let travel = track_length - THUMB_LENGTH;
    if travel <= 0.0 {
        return 0.0;
    }
    ((pointer - grab_offset) / travel).clamp(0.0, 1.0) * maximum.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumb_reaches_both_track_ends_and_midpoint() {
        assert_eq!(thumb_offset(0.0, 1000.0, 100.0), 0.0);
        assert_eq!(thumb_offset(500.0, 1000.0, 100.0), 40.0);
        assert_eq!(thumb_offset(1000.0, 1000.0, 100.0), 80.0);
    }

    #[test]
    fn dragging_preserves_the_grab_position_and_clamps_to_bounds() {
        assert_eq!(scroll_position(50.0, 10.0, 100.0, 1000.0), 500.0);
        assert_eq!(scroll_position(-20.0, 10.0, 100.0, 1000.0), 0.0);
        assert_eq!(scroll_position(200.0, 10.0, 100.0, 1000.0), 1000.0);
    }

    #[test]
    fn non_scrollable_or_tiny_tracks_are_safe() {
        assert_eq!(thumb_offset(0.0, 0.0, 100.0), 0.0);
        assert_eq!(scroll_position(10.0, 0.0, 10.0, 1000.0), 0.0);
        assert_eq!(scroll_position(10.0, 0.0, 100.0, 0.0), 0.0);
    }
}
