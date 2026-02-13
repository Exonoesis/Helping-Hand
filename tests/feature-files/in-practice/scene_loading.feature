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
        Then there is only one image loaded.
        And the image at 'acts/images/PI2.png' is displayed on the screen.
        And the loaded image's opacity is 100%.

    Scenario: A map cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        And the game transitions to scene 4,
        Then the title of the current scene loaded is called 'Map Cutscene 1'.
        And the map size should be 19 x 20 tiles.
        # Confirm character and tile were found properly in the placement.
        And the character 'Jay' will be placed at location 'PlayerStart'.
        And the location 'PlayerStart' is at tile 5, 17.
        # Confirm character and each tile were found properly in the line path.
        And the character 'Iye' will be moved along the line path 'GoToPlayer'.
        And the line path 'GoToPlayer' has a path length of 5 tiles.
        And tile 1 of line path 'GoToPlayer' is tile 1, 16.
        And tile 2 of line path 'GoToPlayer' is tile 2, 16.
        And tile 3 of line path 'GoToPlayer' is tile 3, 16.
        And tile 4 of line path 'GoToPlayer' is tile 4, 16.
        And tile 5 of line path 'GoToPlayer' is tile 5, 16.
        # Confirm character and each tile were found properly in the looping path.
        And the character 'Siblings' will be moved along the looping path 'RunInCircles'.
        And the looping path 'RunInCircles' has a path length of 12 tiles.
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

    Scenario: A line path can have multiple parts.
    Given the game is capable of handling acts,
    When the testing act called 'testing_act.json' is loaded,
    Then the title of the current scene loaded is called 'Multiline Test'.
    And the character 'Jay' will be moved along the line path 'Combo'.
    And the line path 'Combo' has a path length of 10 tiles.
    And tile 1 of line path 'Combo' is tile 0, 0.
    And tile 2 of line path 'Combo' is tile 1, 0.
    And tile 3 of line path 'Combo' is tile 2, 0.
    And tile 4 of line path 'Combo' is tile 2, 1.
    And tile 5 of line path 'Combo' is tile 1, 1.
    And tile 6 of line path 'Combo' is tile 2, 1.
    And tile 7 of line path 'Combo' is tile 3, 1.
    And tile 8 of line path 'Combo' is tile 4, 1.
    And tile 9 of line path 'Combo' is tile 4, 0.
    And tile 10 of line path 'Combo' is tile 3, 0.
