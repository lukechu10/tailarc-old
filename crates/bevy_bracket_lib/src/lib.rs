//! A library for working with [Bevy](https://bevyengine.org) and [bracket-lib](https://crates.io/crates/bracket-lib).

use bevy_app::{App, Plugin};
use bracket_lib::prelude::*;

#[derive(Clone, Debug)]
pub struct BracketLibPlugin {
    bterm: BTerm,
}

impl BracketLibPlugin {
    pub fn new(bterm: BTerm) -> Self {
        BracketLibPlugin { bterm }
    }
}

impl Plugin for BracketLibPlugin {
    fn build(&self, app: &mut bevy_app::AppBuilder) {
        let bterm = self.bterm.clone();
        app.insert_resource(bterm.clone())
            .set_runner(move |app| bracket_lib_runner(app, bterm.clone()));
    }
}

struct State {
    bevy_app: App,
}

impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        self.bevy_app.world.insert_resource(bterm.clone());

        self.bevy_app.update();
    }
}

fn bracket_lib_runner(app: App, bterm: BTerm) {
    let gs = State { bevy_app: app };
    main_loop(bterm, gs).expect("could not start main loop");
}
