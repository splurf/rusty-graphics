use {
    super::{
        error::{Error, Result},
        polygon::*,
    },
    crate::util::event::EventType,
    sdl2::{
        init,
        pixels::Color,
        render::WindowCanvas,
        video::FullscreenType::{Desktop, Off},
        EventPump, Sdl,
    },
};

pub struct Engine {
    canvas: WindowCanvas,
    running: bool,
    ctx: Sdl,
}

impl Engine {
    /**
     * Initialize a new `Engine` with specified window title
     */
    pub fn init<T: AsRef<str>>(title: T) -> Result<Self> {
        let ctx = init().map_err(Error::from)?;
        let video = ctx.video()?;
        let (width, height) = video.display_bounds(0)?.size();

        let mut canvas = video
            .window(title.as_ref(), width / 2, height / 2)
            .build()?
            .into_canvas()
            .build()?;
        canvas.set_draw_color(Color::WHITE);

        let running = false;

        let mut engine = Self {
            canvas,
            running,
            ctx,
        };
        engine.clear();
        engine.flush();
        Ok(engine)
    }

    /**
     * Return a reference of the canvas
     */
    pub fn canvas(&self) -> &WindowCanvas {
        &self.canvas
    }

    /**
     * Return `true` if the engine is *running*
     */
    pub fn running(&self) -> bool {
        self.running
    }

    /**
     * Return a reference of the context
     */
    pub fn ctx(&self) -> &Sdl {
        &self.ctx
    }

    /**
     * Draw a singular line from one `Point` to another
     */
    pub fn draw_line<T: Into<Point>>(&mut self, start: T, end: T) -> Result<()> {
        self.canvas.draw_line(start, end).map_err(Into::into)
    }

    /**
     * Draw a singular line from one `Point` to another
     */
    pub fn draw_lines<A: Into<Point>, T: IntoIterator<Item = A>>(&mut self, iter: T) -> Result<()> {
        self.canvas
            .draw_lines(
                iter.into_iter()
                    .map(Into::into)
                    .collect::<Vec<Point>>()
                    .as_slice(),
            )
            .map_err(Into::into)
    }

    /**
     * Draw a potential `Polygon` onto the current viewport
     */
    pub fn draw_polygon(&mut self, polygon: &Polygon) -> Result<()> {
        self.draw_lines(polygon)
    }

    /**
     * Fill the area of the location of the provided polygon using the scan line method
     */
    pub fn fill_polygon(&mut self, polygon: &Polygon, color: Color, density: usize) -> Result<()> {
        let min = polygon.y_min();
        let max = polygon.y_max();

        let prev = self.canvas.draw_color();
        self.canvas.set_draw_color(color);
        (min..=max)
            .step_by(density)
            .map(|n| {
                polygon
                    .intersections_at_y(n)
                    .chunks_exact(2)
                    .map(|chunk| self.draw_line(chunk[0], chunk[1]))
                    .collect::<Result<()>>()
            })
            .collect::<Result<()>>()?;
        Ok(self.canvas.set_draw_color(prev))
    }

    /**
     * Clear the current viewport
     */
    pub fn clear(&mut self) {
        let prev = self.canvas.draw_color();
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(prev)
    }

    /**
     * Flush the current viewport
     */
    pub fn flush(&mut self) {
        self.canvas.present()
    }

    /**
     * Toggle fullscreen
     */
    pub fn toggle_fullscreen(&mut self) -> Result<()> {
        let window = self.canvas.window_mut();

        window
            .set_fullscreen(if let Off = window.fullscreen_state() {
                Desktop
            } else {
                Off
            })
            .map_err(Error::from)?;

        self.clear();
        Ok(self.flush())
    }

    /**
     * Start handling events with the specified `event_handler` closure
     */
    pub fn start(
        &mut self,
        custom_event_pump: Option<EventPump>,
        mut event_handler: impl FnMut(&mut Self, EventType, fn(&mut Self)) -> Result<()>,
    ) -> Result<()> {
        self.running = true;

        let mut event_pump = if let Some(event_pump) = custom_event_pump {
            event_pump
        } else {
            self.ctx.event_pump()?
        };

        for event in event_pump
            .wait_iter()
            .map(EventType::try_from)
            .filter_map(Result::ok)
        {
            event_handler(self, event, Self::stop)?;
            if !self.running {
                break;
            }
        }
        Ok(())
    }

    /**
     * Stop the engine from handling events
     */
    fn stop(&mut self) {
        self.running = false
    }
}
