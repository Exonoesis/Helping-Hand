Feature: Acts
    Scenario: A title can be found from a loaded scene.
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then the scene with the title 'Intro Image 1' is scene 1 in the current act.

    Scenario: Image Cutscenes are loaded
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then the act's scene called 'Intro Image 1' is an Image Cutscene pointing to the image PI1.png.
        Then the act's scene called 'Intro Image 2' is an Image Cutscene pointing to the image PI2.png.
        Then the act's scene called 'Intro Image 3' is an Image Cutscene pointing to the image PI3.png.

    Scenario: Scene Connections are made correctly
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then scene 'Intro Image 1' should connect to scene 'Intro Image 2'.
        And scene 'Intro Image 2' should connect to scene 'Intro Image 3'.
