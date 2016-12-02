use specs;
use world as w;
use rand;
use sys;
use std::sync::Arc;
use radiant_rs::{Color, Layer};
use assets::AssetManager;


#[derive(Clone)]
pub struct System<'a> {
    pub layer: &'a Layer,
    pub assets: &'a AssetManager<'a>
}


impl<'a> specs::System<super::Delta> for System<'a>
{
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;
        let (body, sprited) = arg.fetch(|w| {
            (w.read::<w::Body>(), w.read::<w::Sprited>())
        });

        // update entities
        self.layer.clear();
        for (b, s) in (&body, &sprited).iter() {
            let frame_id = 0;
            s.sprite.draw_transformed(&self.layer, frame_id, 320.0, 200.0, Color::white(), b.rotation, 1, 1);
        }
    }
}
