#[cfg(test)]
mod tests {
    fn setup_main_menu_build_and_cleanup_checking() -> App {
        let mut app = App::new();

        //We test this as a startup system because we cannot test states directly
        app.add_systems(Startup, spawn_main_menu);

        app
    }

    #[test]
    fn main_menu_build_and_cleanup_checking() {
        //No entities should exist at this point
        let mut app = setup_main_menu_build_and_cleanup_checking();
        let mut item_num = app.world.entities().len();
        assert_eq!(0, item_num);

        //Main Menu entities should now be loaded
        app.update();
        item_num = app.world.entities().len();
        assert!(item_num > 0);

        //Now we call our unload Main Menu function...
        app.add_systems(Update, unload_main_menu);
        app.update();

        //and ensure that no entities remain
        item_num = app.world.entities().len();
        assert_eq!(0, item_num);
    }
}
