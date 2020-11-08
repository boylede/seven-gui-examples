use amethyst::{
    assets::{PrefabLoaderSystemDesc, Processor},
    audio::{output::init_output, Source},
    core::{ transform::TransformBundle},
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{
        RenderUi, UiBundle, UiEvent, UiEventType, UiFinder, UiText, UiCreator,
    },
    utils::{
        application_root_dir,
        fps_counter::{FpsCounterBundle},
        scene::BasicScenePrefab,
    },
    winit::VirtualKeyCode,
};

use log::info;


fn to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9. / 5.) + 32.
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.) * 5. / 9.
}

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
struct Example {
    fahreheit: Option<Entity>,
    celsius: Option<Entity>,
    temperature: f32,
}

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        init_output(&mut world);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("two/temp.ron", ());
        });
        self.temperature = 0.;
    }

    fn handle_event(
        &mut self,
        gd: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                match ui_event {
                    UiEvent{event_type: UiEventType::Focus, target, ..} => {
                        // todo: select entire field
                    }
                    UiEvent{event_type: UiEventType::Blur, target, ..} => {
                        // ?? is any cleanup required here for selections etc..
                    }
                    UiEvent{event_type: UiEventType::ValueChange, target, ..} | UiEvent{event_type: UiEventType::ValueCommit, target, ..} => {
                        if let Some(f) = self.fahreheit {
                            if let Some(c) = self.celsius {
                                match target {
                                    _ if *target == c => {
                                        info!("celsius valuechange");
                                        let mut texts = gd.world.write_storage::<UiText>();
                                        let c_temp = texts.get_mut(c).unwrap().text.parse().unwrap_or(0.0);
                                        self.temperature = c_temp;
                                        let f_temp = to_fahrenheit(c_temp);
                                        
                                        let f_text = texts.get_mut(f).unwrap();
                                        f_text.text = f_temp.to_string();
                                        
                                    }
                                    _ if *target == f => {
                                        info!("fahrenheit valuechange");
                                        let mut texts = gd.world.write_storage::<UiText>();
                                        let f_temp = texts.get_mut(f).unwrap().text.parse().unwrap_or(32.0);
                                        // self.temperature = f_text.text.parse().unwrap_or(32.0);
                                        self.temperature = to_celsius(f_temp);
                                        // c_text.text = self.temperature.to_string();
                                        // self.quantity += 1;
                                        let c_text = texts.get_mut(c).unwrap();
                                        c_text.text = self.temperature.to_string();
                                    }
                                    _ => {
                                        info!("match fail");
                                    }
                                }
                            }
                        } else {
                            info!{"what?"};
                        }
                    }
                    _ => {
                        //
                    }
                }

                Trans::None
            }
            StateEvent::Input(_input) => {
                Trans::None
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        if self.fahreheit.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("fahrenheit") {
                    self.fahreheit = Some(entity);
                }
            });
        }
        if self.celsius.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("celsius") {
                    self.celsius = Some(entity);
                }
            });
        }
        Trans::None
    }
}

pub fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    
    let assets_dir = app_root.join("assets");
    let display_config_path = assets_dir.join("two/display.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.1, 0.1, 0.1, 0.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, Example::default())?
        .build(game_data)?;
    game.run();
    Ok(())
}
