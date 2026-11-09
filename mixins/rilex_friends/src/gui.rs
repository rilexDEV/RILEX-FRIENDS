pub const TITLE: &str = "RILEX FRIENDS";
const CLOSE_HINT: &str = "Right Shift / Esc: Close";

// All coordinates are GUI pixels. Native fill accepts ARGB; native text accepts RGB.
pub fn window_rect(viewport: [f32; 2]) -> Option<[f32; 4]> {
    let [screen_w, screen_h] = viewport;
    if !screen_w.is_finite() || !screen_h.is_finite() || screen_w < 180.0 || screen_h < 120.0 {
        return None;
    }
    let width = 400.0_f32.min(screen_w - 16.0);
    let height = 300.0_f32.min(screen_h - 16.0);
    Some([
        ((screen_w - width) / 2.0).floor(),
        ((screen_h - height) / 2.0).floor(),
        width,
        height,
    ])
}

pub fn draw<B>(
    batch: &mut B,
    viewport: [f32; 2],
    text_width: impl Fn(&str) -> f32,
    mut fill: impl FnMut(&mut B, [f32; 4], u32),
    mut text: impl FnMut(&mut B, &str, [f32; 2], u32),
) {
    let Some([x, y, width, height]) = window_rect(viewport) else {
        return;
    };
    // Dim the world, then draw the panel with a two-pixel dark gray border.
    fill(batch, [0.0, 0.0, viewport[0], viewport[1]], 0x7800_0000);
    fill(batch, [x + 3.0, y + 3.0, width, height], 0x8000_0000);
    fill(batch, [x, y, width, height], 0xFF55_5555);
    fill(
        batch,
        [x + 2.0, y + 2.0, width - 4.0, height - 4.0],
        0xFFC6_C6C6,
    );
    fill(batch, [x + 2.0, y + 2.0, width - 4.0, 2.0], 0xFFFF_FFFF);
    fill(batch, [x + 2.0, y + 4.0, 2.0, height - 6.0], 0xFFFF_FFFF);
    text(
        batch,
        TITLE,
        [(x + (width - text_width(TITLE)) / 2.0).floor(), y + 12.0],
        0x0030_3030,
    );
    fill(batch, [x + 12.0, y + 28.0, width - 24.0, 1.0], 0xFF85_8585);
    fill(batch, [x + 12.0, y + 29.0, width - 24.0, 1.0], 0xFFEE_EEEE);

    // Keyboard-operated close control; mouse interaction arrives with the UI states.
    let button_w = 180.0_f32.min(width - 24.0);
    let bx = (x + (width - button_w) / 2.0).floor();
    let by = y + height - 34.0;
    fill(batch, [bx, by, button_w, 22.0], 0xFF37_3737);
    fill(
        batch,
        [bx + 1.0, by + 1.0, button_w - 2.0, 20.0],
        0xFF8B_8B8B,
    );
    fill(
        batch,
        [bx + 1.0, by + 1.0, button_w - 2.0, 2.0],
        0xFFDD_DDDD,
    );
    fill(batch, [bx + 1.0, by + 3.0, 2.0, 16.0], 0xFFDD_DDDD);
    fill(
        batch,
        [bx + 2.0, by + 19.0, button_w - 3.0, 2.0],
        0xFF55_5555,
    );
    fill(
        batch,
        [bx + button_w - 3.0, by + 3.0, 2.0, 16.0],
        0xFF55_5555,
    );
    text(
        batch,
        CLOSE_HINT,
        [
            (bx + (button_w - text_width(CLOSE_HINT)) / 2.0).floor(),
            by + 7.0,
        ],
        0x00FF_FFFF,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centers_and_fits_small_gui_scales() {
        assert_eq!(
            window_rect([800.0, 600.0]),
            Some([200.0, 150.0, 400.0, 300.0])
        );
        assert_eq!(window_rect([320.0, 240.0]), Some([8.0, 8.0, 304.0, 224.0]));
        assert_eq!(window_rect([0.0, 0.0]), None);
        assert_eq!(window_rect([f32::NAN, 240.0]), None);
    }

    #[test]
    fn title_is_centered_and_panel_has_requested_palette() {
        let mut rectangles = Vec::new();
        let mut labels = Vec::new();
        draw(
            &mut rectangles,
            [800.0, 600.0],
            |_| 78.0,
            |rects, rect, color| rects.push((rect, color)),
            |_, label, at, _| labels.push((label.to_owned(), at)),
        );
        assert_eq!(rectangles[2], ([200.0, 150.0, 400.0, 300.0], 0xFF55_5555));
        assert_eq!(rectangles[3], ([202.0, 152.0, 396.0, 296.0], 0xFFC6_C6C6));
        assert_eq!(labels[0], ("RILEX FRIENDS".to_owned(), [361.0, 162.0]));
    }
}
