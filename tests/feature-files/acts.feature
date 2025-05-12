Feature: Acts
    Scenario: Image Cutscenes are loaded
        Given an act file called introductory_act.json,
        When the act is read from the act file,
        Then the act's scene 1 called Intro Image 1 is an Image Cutscene pointing to the image PI1.png.
        Then the act's scene 2 called Intro Image 2 is an Image Cutscene pointing to the image PI2.png.
        Then the act's scene 3 called Intro Image 3 is an Image Cutscene pointing to the image PI3.png.
