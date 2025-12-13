Feature: An Act's Scenes can be traversed in the game.
    Scenario: The game has an initial scene.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        Then the title of the current scene loaded is called 'Intro Image 1'.

    Scenario: The game can transition to the next scene.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        And the game transitions to the next scene,
        Then the title of the current scene loaded is called 'Intro Image 2'.

    Scenario: An image cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        Then the title of the current scene loaded is called 'Intro Image 1'.
        And the image at 'acts/images/PI1.png' is displayed on the screen.

    Scenario: Image cutscenes can fade between one another.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        And the game transitions to the next scene,
        And the fade timer has elapsed,
        Then there is only one image loaded.
        And the image at 'acts/images/PI2.png' is displayed on the screen.
        And the loaded image's opacity is 100%.

    Scenario: A map cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        # TODO: This is 'and the game transitions to the next scene', but called n times.
        And the game transitions to scene 4,
        # TODO: This step comes form 'map_changing.feature'. Steal that from 'map_changing.rs'
        Then the map size should be 19 x 20 tiles.
        And there is a location called 'PlayerStart' at tile 5, 17.
        And the character 'Jay' is at tile 5, 17.
        # Confirm each tile was found properly in the line path.
        And there is a line path called 'GoToPlayer' at tile 1, 16 with a path length of 5 tiles.
        And the line path 'GoToPlayer' contains the tile 1, 16.
        And the line path 'GoToPlayer' contains the tile 2, 16.
        And the line path 'GoToPlayer' contains the tile 3, 16.
        And the line path 'GoToPlayer' contains the tile 4, 16.
        And the line path 'GoToPlayer' contains the tile 5, 16.
        # Confirm each tile was found properly in the looping path.
        And there is a looping path called 'RunInCircles' at tile 3, 11 with a path length of 13 tiles.
        And the looping path 'RunInCircles' contains the tile 3, 11.
        And the looping path 'RunInCircles' contains the tile 4, 11.
        And the looping path 'RunInCircles' contains the tile 5, 11.
        And the looping path 'RunInCircles' contains the tile 6, 11.
        And the looping path 'RunInCircles' contains the tile 6, 12.
        And the looping path 'RunInCircles' contains the tile 6, 13.
        And the looping path 'RunInCircles' contains the tile 6, 14.
        And the looping path 'RunInCircles' contains the tile 5, 14.
        And the looping path 'RunInCircles' contains the tile 4, 14.
        And the looping path 'RunInCircles' contains the tile 3, 14.
        And the looping path 'RunInCircles' contains the tile 3, 13.
        And the looping path 'RunInCircles' contains the tile 3, 12.
        And the looping path 'RunInCircles' contains the tile 3, 11.
