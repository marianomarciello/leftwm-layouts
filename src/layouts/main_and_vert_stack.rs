use crate::{Layout, geometry::Rect, LayoutModifiers};


#[derive(Debug)]
pub struct MainAndVertStack;

impl Layout for MainAndVertStack {
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Option<Rect>> {
        if window_count == 0 {
            return vec![];
        }

        // QUESTION: where should the width limiter be implemented?
        //let column_count = match window_count {
        //    1 => 1,
        //    _ => 2,
        //};

        let master_width = match window_count {
            1 => modifiers.container_size.w,
            _ => (modifiers.container_size.w as f32 / 100.0 * modifiers.master_width_percentage) as i32
        };

        let mut master_x = modifiers.container_size.x;
        let stack_x = if modifiers.flipped_horizontal {
            master_x = match window_count {
                1 => master_x,
                _ => master_x + modifiers.container_size.w - master_width,
            };
            match window_count {
                1 => 0,
                _ => modifiers.container_size.x
            }
        } else {
            modifiers.container_size.x + master_width
        };


        // build the master window
        let mut vec: Vec<Option<Rect>> = Vec::new();
        vec.push(Some(Rect {
            x: master_x,
            y: modifiers.container_size.y,
            w: master_width,
            h: modifiers.container_size.h,
        }));

        // stack all the others
        let height_f = modifiers.container_size.h as f32 / (window_count - 1) as f32;
        let height = height_f.floor() as i32;
        let mut y = 0;
        for _ in 1..window_count {
            vec.push(Some(Rect {
                x: stack_x,
                y: modifiers.container_size.y + y,
                w: modifiers.container_size.w - master_width,
                h: height,
            }));
            y += height
        }

        vec
    }
}

mod tests {
    use crate::{Layout, LayoutModifiers, geometry::Rect};
    use crate::MainAndVertStack;

    #[test]
    fn monocle_returns_only_one_rect() {
        //let rects = Monocle.apply(3, &LayoutModifiers::default());
        //let present: Vec<Rect> = rects.into_iter().filter_map(|e| e).collect();
        //assert_eq!(present.len(), 1);
    }
}