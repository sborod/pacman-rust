extern crate time;

const FRAME_PER_SECOND: i64 = 120;

fn timestamp() -> i64 {
    let current_time = time::get_time();
    current_time.sec*1000 + i64::from( current_time.nsec / 1_000_000 )
}

pub struct MainLoop {
    initial_time:        i64,
    current_time:        i64,
    current_frame:       i64,
    last_rendered_frame: i64
}

impl MainLoop {
    pub fn init() -> MainLoop {
        MainLoop {
            initial_time:        timestamp(),
            current_time:        0,
            current_frame:       0,
            last_rendered_frame: 0
        }
    }

    pub fn update( &mut self ) {
        self.current_time = timestamp() - self.initial_time;
        self.current_frame = self.current_time*FRAME_PER_SECOND / 1000;

        if self.current_frame > self.last_rendered_frame {
            // do calc
        }
    }

    pub fn render( &mut self, rendering_fn: &Fn() ) {
        if self.current_frame > self.last_rendered_frame {
            // //
            // println!( "Current time: {t}", t = ( self.current_time as f64 ) / 1000.0 );

            rendering_fn();

            // 
            self.last_rendered_frame = self.current_frame;
        }
    }
}