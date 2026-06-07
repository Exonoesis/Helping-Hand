Feature: NPC Movements
    Scenario: An NPC can be placed in a specified location
        Given the game is loaded with the act 'something.json',
        When the map cutscene is loaded,
        Then the NPC 'Name' is at tile 0, 0.
