//! Desktop coordinates in a consistent unit; independent of AppKit and window handles.
use serde::Serialize;
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Side {
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Esc reaches the app's key window, not the non-activating floating panel.
/// Treat it as a panel collapse only while an expanded panel is under the pointer.
pub(super) fn escape_collapses(
    visible: bool,
    frame: Rect,
    cursor: (f64, f64),
    dragging: bool,
) -> bool {
    // The ball is 56 points square and the edge handle is narrower.
    const BALL: f64 = 56.;
    visible
        && !dragging
        && (frame.width > BALL + 1. || frame.height > BALL + 1.)
        && cursor.0 >= frame.x
        && cursor.1 >= frame.y
        && cursor.0 < frame.x + frame.width
        && cursor.1 < frame.y + frame.height
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escape_collapses_only_an_expanded_panel_under_the_pointer() {
        let panel = Rect {
            x: 1380.,
            y: 456.,
            width: 300.,
            height: 540.,
        };
        assert!(escape_collapses(true, panel, (1500., 600.), false));
        // The pointer elsewhere means Esc belongs to the focused main window.
        assert!(!escape_collapses(true, panel, (700., 600.), false));
        assert!(
            !escape_collapses(true, panel, (1680., 600.), false),
            "right edge is exclusive"
        );
        assert!(!escape_collapses(false, panel, (1500., 600.), false));
        // An active drag keeps its own Esc cancel path.
        assert!(!escape_collapses(true, panel, (1500., 600.), true));
        for collapsed in [
            Rect {
                x: 1608.,
                y: 483.,
                width: 56.,
                height: 56.,
            },
            Rect {
                x: 1652.,
                y: 483.,
                width: 28.,
                height: 56.,
            },
        ] {
            assert!(!escape_collapses(
                true,
                collapsed,
                (collapsed.x + 10., collapsed.y + 10.),
                false
            ));
        }
    }
    fn area() -> Rect {
        Rect {
            x: 0.,
            y: 25.,
            width: 1680.,
            height: 980.,
        }
    }
    #[test]
    fn retina_pointer_is_normalized_before_screen_lookup() {
        assert_eq!(logical_point((2400., 1016.), 2.), (1200., 508.));
        assert_eq!(logical_point((-1920., 400.), 2.), (-960., 200.));
        assert_eq!(logical_point((1200., 508.), 1.), (1200., 508.));
    }
    #[test]
    fn visible_contact_scales_without_pointer_reaching_edge() {
        for scale in [1., 1.5, 2.] {
            let size = 56. * scale;
            let r = Rect {
                x: 1680. - size + 4. * scale,
                y: 300.,
                width: size,
                height: size,
            };
            let (next, side) = drag_frame(r, (0., 0.), (0., 0.), area(), true);
            assert_eq!(side, Some(Side::Right));
            assert_eq!(next.x, 1680. - size);
            let r = Rect {
                x: -4. * scale,
                y: 300.,
                width: size,
                height: size,
            };
            assert_eq!(
                drag_frame(r, (0., 0.), (0., 0.), area(), true).1,
                Some(Side::Left)
            );
        }
    }
    #[test]
    fn stable_grab_retreat_nearby_disabled_and_panel() {
        let r = Rect {
            x: 800.,
            y: 300.,
            width: 56.,
            height: 56.,
        };
        let start = (828., 328.);
        assert_eq!(
            drag_frame(r, start, (1656., 328.), area(), true).1,
            Some(Side::Right)
        );
        assert_eq!(drag_frame(r, start, (1655., 328.), area(), true).1, None);
        assert_eq!(drag_frame(r, start, (1100., 328.), area(), true).0.x, 1072.);
        assert_eq!(drag_frame(r, start, (1656., 328.), area(), false).1, None);
        let panel = Rect {
            width: 300.,
            height: 250.,
            ..r
        };
        assert_eq!(
            drag_frame(panel, start, (1680., 328.), area(), true).1,
            Some(Side::Right)
        );
    }
    #[test]
    fn negative_monitor_and_vertical_work_area_stay_visible() {
        let a = Rect {
            x: -1920.,
            y: -200.,
            width: 1920.,
            height: 1000.,
        };
        let r = Rect {
            x: -52.,
            y: 780.,
            width: 56.,
            height: 56.,
        };
        let (next, side) = drag_frame(r, (0., 0.), (0., 0.), a, true);
        assert_eq!(side, Some(Side::Right));
        assert_eq!(next.x, -56.);
        assert_eq!(next.y, 744.);
    }
}

pub(super) fn drag_frame(
    origin: Rect,
    start: (f64, f64),
    pointer: (f64, f64),
    area: Rect,
    enabled: bool,
) -> (Rect, Option<Side>) {
    let mut next = Rect {
        x: origin.x + pointer.0 - start.0,
        y: origin.y + pointer.1 - start.1,
        ..origin
    };
    let inset = if origin.width == origin.height {
        origin.width / 14.
    } else {
        0.
    };
    let side = if !enabled {
        None
    } else if next.x + inset <= area.x {
        Some(Side::Left)
    } else if next.x + next.width - inset >= area.x + area.width {
        Some(Side::Right)
    } else {
        None
    };
    next.x = match side {
        Some(Side::Left) => area.x,
        Some(Side::Right) => (area.x + area.width - next.width).max(area.x),
        None => next
            .x
            .clamp(area.x, (area.x + area.width - next.width).max(area.x)),
    };
    next.y = next
        .y
        .clamp(area.y, (area.y + area.height - next.height).max(area.y));
    (next, side)
}

// Tao cursor_position uses the primary display scale, while monitor_from_point
// takes logical desktop points on macOS. Normalize before selecting a screen.
pub(super) fn logical_point(point: (f64, f64), scale: f64) -> (f64, f64) {
    (point.0 / scale, point.1 / scale)
}
