#[cfg(test)]
mod tests {
    fn setup_settings_menu_build_and_cleanup_checking() -> App {
        let mut app = App::new();

        //We test this as a startup system because we cannot test states directly
        app.add_systems(Startup, spawn_settings_menu);

        app
    }

    #[test]
    fn settings_menu_build_and_cleanup_checking() {
        //No entities should exist at this point
        let mut app = setup_settings_menu_build_and_cleanup_checking();
        let mut item_num = app.world.entities().len();
        assert_eq!(0, item_num);

        //Settings Menu entities should now be loaded
        app.update();
        item_num = app.world.entities().len();
        assert!(item_num > 0);

        //Now we call our unload Settings Menu function...
        app.add_systems(Update, unload_settings_menu);
        app.update();

        //and ensure that no entities remain
        item_num = app.world.entities().len();
        assert_eq!(0, item_num);
    }

    #[test]
    fn get_0_percent_from_text_field() {
        let spinner_text_bundle = TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        );

        let spinner_text_field = spinner_text_bundle.text;

        let expected_percentage = 0.0;
        let actual_percentage = get_percentage_from(spinner_text_field);

        assert_eq!(expected_percentage, actual_percentage);
    }

    #[test]
    fn get_50_percent_from_text_field() {
        let spinner_text_bundle = TextBundle::from_section(
            "50",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        );

        let spinner_text_field = spinner_text_bundle.text;

        let expected_percentage = 0.5;
        let actual_percentage = get_percentage_from(spinner_text_field);

        assert_eq!(expected_percentage, actual_percentage);
    }

    #[test]
    fn get_100_percent_from_text_field() {
        let spinner_text_bundle = TextBundle::from_section(
            "100",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        );

        let spinner_text_field = spinner_text_bundle.text;

        let expected_percentage = 1.0;
        let actual_percentage = get_percentage_from(spinner_text_field);

        assert_eq!(expected_percentage, actual_percentage);
    }

    // TODO: Implement the rest of these methods with TDD
    // for the 'Introduce Tests for Existing Features'
    // Feature Request/GitHub Issue types.
    //
    //#[test]
    //fn music_slider_changes_volume() {
    //    let mut game = HelpingHand::new();

    //    let slider = game.find_slider(SliderType::Music);
    //    let percentage = 50;

    //    game.move_slider_to(slider, percentage);

    //    let slider_percentage = game.get_slider_percentage_of(slider);
    //    let volume_percentage = game.get_volume_of(SoundSource::Music);

    //    assert_eq!(percentage, volume_percentage);
    //    assert_eq!(slider_percentage, volume_percentage);
    //}
}
