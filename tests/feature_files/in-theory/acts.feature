Feature: Acts
    Scenario: A title can be found from a loaded scene.
        Given an act file called image_cutscene_act.json,
        When the act is read from the act file,
        Then the scene with the title 'Intro Image 1' is scene 1 in the current act.

    Scenario: The current scene is updated
        Given an act file called image_cutscene_act.json,
        When the act is read from the act file,
        And we move to the next scene,
        Then the current scene is 'Intro Image 2'.

    Scenario: Scenes connect correctly
        Given an act file called image_cutscene_act.json,
        When the act is read from the act file,
        Then scene 'Intro Image 1' should connect to scene 'Intro Image 2'.

    Scenario: Image Cutscenes are loaded
        Given an act file called image_cutscene_act.json,
        When the act is read from the act file,
        Then the act's scene called 'Intro Image 1' is an Image Cutscene pointing to the image PI1.png.
        And the act's scene called 'Intro Image 2' is an Image Cutscene pointing to the image PI2.png.

    Scenario: Dialogue in MapCutscenes are loaded
        Given an act file called dialogue_act.json,
        When the act is read from the act file,
        Then in Map Action 1 of 'Map Cutscene 1' Map Instruction 1 says to Read Dialogue Test,
        And Dialogue Test has 3 lines of text,
        And the speaker of line 1 is 'Jay' with text 'This is testing text.',
        And the speaker of line 2 is 'Iye' with text 'This one is really long and probably goes off the screen if we don't have text wrapping or formatting or anything like that. Also it will take some time to print out once we're going one character at a time. Characters as in letters in a word though, not characters like us!',
        And the speaker of line 3 is 'Jay' with text 'Wow, you said a lot!'.
