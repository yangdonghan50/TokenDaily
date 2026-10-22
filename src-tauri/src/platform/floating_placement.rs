//! Initial placement only; subsequent user dragging and edge docking are unchanged.
use tauri::{LogicalSize, PhysicalPosition, WebviewWindow};

fn initial_position(x: i32, y: i32, width: u32, height: u32, scale: f64) -> PhysicalPosition<i32> {
    let ball = (56.0 * scale).round() as u32;
    let gap = (16.0 * scale).round() as u32;
    PhysicalPosition::new(
        x.saturating_add(width.saturating_sub(ball.saturating_add(gap)) as i32),
        y.saturating_add((height.saturating_sub(ball) / 2) as i32),
    )
}

pub(crate) fn initialize(window: &WebviewWindow) -> tauri::Result<()> {
    if let Some(monitor) = window.primary_monitor()?.or(window.current_monitor()?) {
        let area = monitor.work_area();
        window.set_size(LogicalSize::new(56.0, 56.0))?;
        window.set_position(initial_position(
            area.position.x,
            area.position.y,
            area.size.width,
            area.size.height,
            monitor.scale_factor(),
        ))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn right_middle_respects_dpi_and_work_area() {
        assert_eq!(
            initial_position(0, 0, 1920, 1040, 1.25),
            PhysicalPosition::new(1830, 485)
        );
        assert_eq!(
            initial_position(0, 40, 1920, 1000, 1.0),
            PhysicalPosition::new(1848, 512)
        );
        assert_eq!(
            initial_position(-1920, -100, 1920, 1040, 1.25),
            PhysicalPosition::new(-90, 385)
        );
        assert_eq!(
            initial_position(20, 30, 40, 40, 2.0),
            PhysicalPosition::new(20, 30)
        );
    }
}
