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

pub trait Gamable {
    fn input_handling( &mut self );
    fn updating( &mut self );
    fn rendering( &self );
}


impl MainLoop {

    pub fn run( game: &mut Gamable ) {
        let mut tmp_main_loop = MainLoop {
            initial_time:        timestamp(),
            time:                0,
            frame:               0,
            last_rendered_frame: 0
        };

        loop {
            game.input_handling();
            tmp_main_loop.update( game );
        }
    }

    #[inline]
    fn update( &mut self, game: &mut Gamable ) {
        self.time = timestamp() - self.initial_time;
        self.frame = self.time*FRAME_PER_SECOND / 1000;

        if self.frame > self.last_rendered_frame {

            //
            game.updating();
            game.rendering();

            //
            self.last_rendered_frame = self.frame;
            
        }
    }

}