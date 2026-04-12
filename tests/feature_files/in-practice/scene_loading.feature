Feature: An Act's Scenes can be traversed in the game.
    Scenario: The game has an initial scene.
        Given the game is capable of handling acts,
        When the act called 'image_cutscene_act.json' is loaded,
        Then the title of the current scene loaded is called 'Intro Image 1'.

    Scenario: The game can transition to the next scene.
        Given the game is capable of handling acts,
        When the act called 'image_cutscene_act.json' is loaded,
        And the game transitions to the next scene,
        Then the title of the current scene loaded is called 'Intro Image 2'.

    ##################################################################################
    #                             Image Cutscene Tests                               #
    ##################################################################################

    Scenario: An image cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'image_cutscene_act.json' is loaded,
        Then the title of the current scene loaded is called 'Intro Image 1'.
        And the image at 'acts/images/PI1.png' is displayed on the screen.

    Scenario: Image cutscenes can fade between one another.
        Given the game is capable of handling acts,
        When the act called 'image_cutscene_act.json' is loaded,
        And the game transitions to the next scene,
        Then there is only one image loaded.
        And the image at 'acts/images/PI2.png' is displayed on the screen.
        And the loaded image's opacity is 100%.

    ##################################################################################
    #                              Map Cutscene Tests                                #
    ##################################################################################

    Scenario: A map cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'map_cutscene_act.json' is loaded,
        Then the title of the current scene loaded is called 'Placement Test'.
        And the map size should be 2 x 2 tiles.

    Scenario: The character and tile is found properly in a placement.
        Given the game is capable of handling acts,
        When the act called 'map_cutscene_act.json' is loaded,
        Then the title of the current scene loaded is called 'Placement Test'.
        And the character 'Jay' will be placed at location 'PlayerStart'.
        And the location 'PlayerStart' is at tile 1, 1.

    Scenario: The character and each tile is found properly on a complex line path.
        Given the game is capable of handling acts,
        When the act called 'map_cutscene_act.json' is loaded,
        And the game transitions to scene 2,
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

    Scenario: The character and each tile is found properly in a looping path.
        Given the game is capable of handling acts,
        When the act called 'map_cutscene_act.json' is loaded,
        And the game transitions to scene 3,
        Then the title of the current scene loaded is called 'Looping Test'.
        And the character 'Jay' will be moved along the looping path 'LoopingPath'.
        #And the looping path 'LoopingPath' has a path length of 12 tiles.
        And tile 1 of looping path 'LoopingPath' is tile 1, 0.
        And tile 2 of looping path 'LoopingPath' is tile 1, 1.
        And tile 3 of looping path 'LoopingPath' is tile 1, 2.
        And tile 4 of looping path 'LoopingPath' is tile 1, 3.
        And tile 5 of looping path 'LoopingPath' is tile 2, 3.
        And tile 6 of looping path 'LoopingPath' is tile 3, 3.
        And tile 7 of looping path 'LoopingPath' is tile 4, 3.
        And tile 8 of looping path 'LoopingPath' is tile 4, 2.
        And tile 9 of looping path 'LoopingPath' is tile 4, 1.
        And tile 10 of looping path 'LoopingPath' is tile 4, 0.
        And tile 11 of looping path 'LoopingPath' is tile 3, 0.
        And tile 12 of looping path 'LoopingPath' is tile 2, 0.
