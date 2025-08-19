Feature: Acts
    Scenario: A title can be found from a loaded scene.
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then the scene with the title 'Intro Image 1' is scene 1 in the current act.

    Scenario: The current scene is updated
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        And we move to the next scene,
        Then the current scene is 'Intro Image 2'.

    Scenario: Image Cutscenes are loaded
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then the act's scene called 'Intro Image 1' is an Image Cutscene pointing to the image PI1.png.
        And the act's scene called 'Intro Image 2' is an Image Cutscene pointing to the image PI2.png.
        And the act's scene called 'Intro Image 3' is an Image Cutscene pointing to the image PI3.png.

    Scenario: Scenes 1-3 are connected correctly
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then scene 'Intro Image 1' should connect to scene 'Intro Image 2'.
        And scene 'Intro Image 2' should connect to scene 'Intro Image 3'.

    Scenario: Map Cutscenes are loaded.
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then the act's scene called 'Map Cutscene 1' is a Map Cutscene pointing to the map file called breeding_center.tmx.
        And the act's scene called 'Map Cutscene 1' is a Map Cutscene with 3 Commands.

    Scenario: Scenes 3-4 are connected correctly
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then scene 'Intro Image 3' should connect to scene 'Map Cutscene 1'.
