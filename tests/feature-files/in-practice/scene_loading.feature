Feature: Scenes can be loaded into the game from an Act.
    Scenario: An image cutscene is loaded into the game.
        Given an act file called 'some_act.json',
        When the act is loaded,
        Then the title of the current scene loaded is called 'some image cutscene'.
        And there is an image loaded pointing to 'some/image.png'.
