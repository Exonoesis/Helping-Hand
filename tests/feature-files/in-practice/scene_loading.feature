Feature: Scenes can be loaded into the game from an Act.
    Scenario: An image cutscene is loaded into the game.
        Given the game is capable of handling acts,
        When the act called 'introductory_act.json' is loaded,
        Then the title of the current scene loaded is called 'Intro Image 1'.
        And the image at 'acts/images/PI1.png' is displayed on the screen.
