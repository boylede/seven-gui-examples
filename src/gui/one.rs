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
        Anchor, RenderUi, UiBundle, UiLabelBuilder, UiLabel, UiButton, UiButtonBuilder, UiEvent, UiEventType, UiFinder, UiImage, UiText, Widgets,
    },
    utils::{
        application_root_dir,
        fps_counter::{FpsCounterBundle},
        scene::BasicScenePrefab,
    },
    winit::VirtualKeyCode,
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
struct Example {
    counter: Option<Entity>,
    button: Option<Entity>,
    quantity: u32,
}

type Labels = Widgets::<UiLabel, String>;
type Buttons = Widgets::<UiButton, String>;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        world.insert(Labels::default());
        world.insert(Buttons::default());

        let (_button_id, _label) =
            UiButtonBuilder::<(), String>::new("Increment".to_string())
                .with_id("button".to_string())
                .with_font_size(32.0)
                .with_position(0.0, 0.0)
                .with_size(64.0 * 6.0, 64.0)
                .with_anchor(Anchor::MiddleLeft)
                .with_image(UiImage::SolidColor([0.8, 0.6, 0.3, 1.0]))
                .with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]))
                .build_from_world(&world);


        init_output(&mut world);
        let (_i, _l) = UiLabelBuilder::<String>::new("0".to_string())
                .with_id("counter".to_string())
                .with_font_size(32.0)
                .with_position(0.0, 0.0)
                .with_size(64.0 * 6.0, 64.0)
                .with_anchor(Anchor::MiddleLeft)
                .build_from_world(&world);
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
                if let UiEvent{event_type: UiEventType::ClickStop, target, ..} = ui_event {
                    if let Some(b) = self.button {
                        if b == *target {
                            if let Some(e) = self.counter {
                                let mut texts = gd.world.write_storage::<UiText>();
                                let counter = texts.get_mut(e).unwrap();
                                self.quantity += 1;
                                counter.text = self.quantity.to_string();
                            }
                        }
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
        if self.counter.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("counter_label") {
                    self.counter = Some(entity);
                }
            });
        }
        if self.button.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("button_btn") {
                    self.button = Some(entity);
                }
            });
        }
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display_config.ron");
    let assets_dir = app_root.join("assets");

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
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, Example::default())?
        .build(game_data)?;
    game.run();
    Ok(())
}
