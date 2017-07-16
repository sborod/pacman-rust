extern crate time;

const FRAME_PER_SECOND: i64 = 120;

fn timestamp() -> i64 {
    let current_time = time::get_time();
    current_time.sec*1000 + ( ( current_time.nsec / 1_000_000 ) as i64 )
}

pub struct MainLoop {
    initial_time:        i64,
    time:                i64,
    frame:               i64,
    last_rendered_frame: i64
}

impl MainLoop {
    pub fn init( rendering_fn: &Fn() ) -> MainLoop {
        let mut tmp_MainLoop = MainLoop {
            initial_time:        timestamp(),
            time:                0,
            frame:               0,
            last_rendered_frame: 0
        };

        loop {
            tmp_MainLoop.update( &rendering_fn );
        }

        tmp_MainLoop
    }

    fn update( &mut self, rendering_fn: &Fn() ) {
        self.time = timestamp() - self.initial_time;
        self.frame = self.time*FRAME_PER_SECOND / 1000;

        if self.frame > self.last_rendered_frame {
            self.render( &rendering_fn );
            self.last_rendered_frame = self.frame;
        }
    }

    fn render( &mut self, rendering_fn: &Fn() ) {
        rendering_fn();
    }
}