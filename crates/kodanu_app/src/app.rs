use crate::{AppConfig, AppRuntime};

use {
    kodanu_camera::{ActiveCamera, Camera},
    kodanu_ecs::{Read, World, Write},
    kodanu_editor::EditorView,
    kodanu_graphics::{RenderQueue, RendererConfig},
    kodanu_input::{Input, InputPlugin, KeyCode, WinitHandler},
    kodanu_log::LogConfig,
    kodanu_math::{DVec2, UVec2},
    kodanu_plugin::{Plugin, PluginRegistry},
    kodanu_scheduler::{Scheduler, Stage, System},
    kodanu_time::Time,
    kodanu_window::WindowConfig,
    tracing_subscriber::fmt,
};

use kodanu_editor::EditorViewPlugin;
use kodanu_time::TimePlugin;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

#[derive(Default)]
pub struct App {
    runtime: Option<AppRuntime>,
    world: World,
    scheduler: Scheduler,
    config: AppConfig,
}

impl App {
    pub fn run(mut self) {
        fmt().with_env_filter(self.config.log().env_filter()).init();

        let event_loop = EventLoop::new().expect("Failed to create event loop");

        event_loop.run_app(&mut self).expect("Failed to run app");
    }
}

impl App {
    pub fn add_plugin<P>(&mut self, plugin: P) -> &mut Self
    where
        P: Plugin,
    {
        plugin.build(&mut PluginRegistry::new(
            &mut self.scheduler,
            &mut self.world.cell(),
        ));

        self
    }
}

impl App {
    pub fn with_window_config(mut self, config: WindowConfig) -> Self {
        self.config.set_window_config(config);
        self
    }

    pub fn with_renderer_config(mut self, config: RendererConfig) -> Self {
        self.config.set_renderer_config(config);
        self
    }

    pub fn with_log_config(mut self, config: LogConfig) -> Self {
        self.config.set_log_config(config);
        self
    }
}

impl App {
    pub fn add_startup_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::Startup, system);
        self
    }

    pub fn add_pre_fixed_update_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::PreFixedUpdate, system);
        self
    }

    pub fn add_fixed_update_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::FixedUpdate, system);
        self
    }

    pub fn add_post_fixed_update_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::PostFixedUpdate, system);
        self
    }

    pub fn add_pre_update_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::PreUpdate, system);
        self
    }

    pub fn add_update_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::Update, system);
        self
    }

    pub fn add_late_update_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::LateUpdate, system);
        self
    }

    pub fn add_pre_render_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::PreRender, system);
        self
    }

    pub fn add_render_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::Render, system);
        self
    }

    pub fn add_post_render_system(mut self, system: System) -> Self {
        self.scheduler.add(Stage::PostRender, system);
        self
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.runtime.is_some() {
            return;
        }

        self.runtime =
            Some(AppRuntime::new(event_loop, &self.config).expect("Failed to create app"));

        self.add_plugin(InputPlugin);

        self.add_plugin(TimePlugin);

        self.world.insert_resource(RenderQueue::default());
        self.scheduler
            .add(Stage::Render, RenderQueue::update_queue_system);

        self.add_plugin(EditorViewPlugin);

        self.scheduler.run_startup(self.world.cell());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.handle_redraw(event_loop),
            WindowEvent::Resized(size) => self.handle_resize(size),
            _ => self.handle_input(event),
        }
    }
}

impl App {
    fn handle_redraw(&mut self, event_loop: &ActiveEventLoop) {
        let (world, runtime) = (self.world.cell(), unsafe {
            self.runtime.as_mut().unwrap_unchecked()
        });

        let (input, editor, queue, time) = (
            world.res::<Read<Input>>(),
            world.res::<Read<EditorView>>(),
            world.res::<Read<RenderQueue>>(),
            world.res::<Read<Time>>(),
        );

        if input.key_just_pressed(KeyCode::Escape) {
            event_loop.exit();
        }

        self.scheduler.run_fixed(world, time.delta());

        self.scheduler.run_update(world);
        self.scheduler.run_render(world);

        runtime.render(editor.view_projection(), queue.items());

        runtime.request_redraw();
    }

    fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        let (world, runtime) = (self.world.cell(), unsafe {
            self.runtime.as_mut().unwrap_unchecked()
        });

        runtime.surface_resize(UVec2::new(size.width, size.height));

        for (camera, _) in world.view::<(Write<Camera>, Read<ActiveCamera>)>() {
            camera.set_viewport_size(size.width, size.height);
        }
    }

    fn handle_input(&mut self, event: WindowEvent) {
        let input = self.world.res::<Write<Input>>();

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                WinitHandler::handle_keyboard_input(input, &event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                WinitHandler::handle_mouse_input(input, state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                WinitHandler::handle_cursor_move(input, DVec2::new(position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                WinitHandler::handle_mouse_wheel(input, delta);
            }
            _ => {}
        }
    }
}
