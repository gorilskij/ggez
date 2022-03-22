use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event;
use ggez::graphics;
use ggez::input;
use ggez::{Context, GameResult};

use glam::*;

use std::env;
use std::path;
use std::time::Duration;

struct MainState {
    sound: audio::Source,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sound = audio::Source::new(&ctx.fs, &ctx.audio, "/sound.ogg")?;
        let s = MainState { sound };
        Ok(s)
    }

    // To test: play, play_later, play_detached(),
    // set_repeat, set_fade_in, set_pitch,
    // basically every method on Source, actually,
    // then the same ones for `SpatialSource`.

    /// Plays the sound multiple times
    fn play_detached(&mut self, ctx: &mut Context) {
        // "detached" sounds keep playing even after they are dropped
        let _ = self.sound.play_detached(&ctx.audio);
    }

    /// Waits until the sound is done playing before playing again.
    fn play_later(&mut self, _ctx: &mut Context) {
        let _ = self.sound.play_later();
    }

    /// Fades the sound in over a second
    /// Which isn't really ideal 'cause the sound is barely a second long, but still.
    fn play_fadein(&mut self, ctx: &mut Context) {
        let mut sound = audio::Source::new(&ctx.fs, &ctx.audio, "/sound.ogg").unwrap();
        sound.set_fade_in(Duration::from_millis(1000));
        sound.play_detached(&ctx.audio).unwrap();
    }

    fn play_highpitch(&mut self, ctx: &mut Context) {
        let mut sound = audio::Source::new(&ctx.fs, &ctx.audio, "/sound.ogg").unwrap();
        sound.set_pitch(2.0);
        sound.play_detached(&ctx.audio).unwrap();
    }
    fn play_lowpitch(&mut self, ctx: &mut Context) {
        let mut sound = audio::Source::new(&ctx.fs, &ctx.audio, "/sound.ogg").unwrap();
        sound.set_pitch(0.5);
        sound.play_detached(&ctx.audio).unwrap();
    }

    /// Plays the sound and prints out stats until it's done.
    fn play_stats(&mut self, ctx: &mut Context) {
        let _ = self.sound.play(&ctx.audio);
        while self.sound.playing() {
            println!("Elapsed time: {:?}", self.sound.elapsed())
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(&ctx.gfx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw_text(
            &[graphics::Text::new()
                .text("Press number keys 1-6 to play a sound, or escape to quit.")
                .font("LiberationMono")],
            Vec2::new(100.0, 100.0),
            0.0,
            graphics::TextLayout::tl_single_line(),
            0,
        );

        canvas.finish(&mut ctx.gfx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: input::keyboard::KeyCode,
        _keymod: input::keyboard::KeyMods,
        _repeat: bool,
    ) -> GameResult {
        match keycode {
            input::keyboard::KeyCode::Key1 => self.play_detached(ctx),
            input::keyboard::KeyCode::Key2 => self.play_later(ctx),
            input::keyboard::KeyCode::Key3 => self.play_fadein(ctx),
            input::keyboard::KeyCode::Key4 => self.play_highpitch(ctx),
            input::keyboard::KeyCode::Key5 => self.play_lowpitch(ctx),
            input::keyboard::KeyCode::Key6 => self.play_stats(ctx),
            input::keyboard::KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("sounds", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
