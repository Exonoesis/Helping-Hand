Feature: Scenes can be loaded into the game from an Act.
    Scenario: An image cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        Then the title of the current scene loaded is called 'Intro Image 1'.
        And the image at 'acts/images/PI1.png' is displayed on the screen.

    Scenario: The game can transition to the next scene.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        And the game transitions to the next scene,
        Then the title of the current scene loaded is called 'Intro Image 2'.
        And the image at 'acts/images/PI2.png' is displayed on the screen.

    Scenario: Image Cutscenes can fade.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        And a request is made to fade the scene,
        And the fade timer has elapsed,
        Then there is only one image loaded.
        And the image at 'acts/images/PI2.png' is displayed on the screen.
        And the loaded image's opacity is 100%.
