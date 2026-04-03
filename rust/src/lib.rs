use godot::prelude::*;
use godot::classes::Engine;

mod classes;

use classes::tour_singleton::TourSingleton;


struct GodotTour;

#[gdextension]
unsafe impl ExtensionLibrary for GodotTour {
    fn editor_run_behavior() -> godot::init::EditorRunBehavior {
        godot::init::EditorRunBehavior::ToolClassesOnly
    }
    
    fn min_level() -> InitLevel {
        InitLevel::Scene
    }
    
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            // The StringName identifies your singleton and can be
            // used later to access it.
            Engine::singleton().register_singleton(
                "Tour",
                &TourSingleton::new_alloc().upcast::<Object>(),
            );
        }
    }
    
    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::Scene {
            // Get the `Engine` instance and `StringName` for your singleton.
            let mut engine = Engine::singleton();
            let singleton_name = StringName::from("Tour");

            // We need to retrieve the pointer to the singleton object,
            // as it has to be freed manually - unregistering singleton 
            // doesn't do it automatically.
            let singleton = engine
                .get_singleton(&singleton_name.clone())
                .expect("cannot retrieve the singleton");

            // Unregistering singleton and freeing the object itself is needed 
            // to avoid memory leaks and warnings, especially for hot reloading.
            engine.unregister_singleton(&singleton_name);
            singleton.free();
        }
    }
}
