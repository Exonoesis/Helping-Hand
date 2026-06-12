Feature: NPC Movements
    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates
        Given the game is loaded with the act 'something.json',
        And a line path 'simple_line',
        And an NPC 'Name' is at tile 0,0,1,
        When the map cutscene is loaded,
        Then after taking 0 steps the NPC 'Name' has pixel and grid coordinates equivalent to tile 0,1,1.
        Then the next step the NPC 'Name' has pixel and grid coordinates equivalent to tile 0,2,1.
        Then the next step the NPC 'Name' has pixel and grid coordinates equivalent to tile 0,3,1.

    Scenario: An NPC can be placed in a specified location
        Given the game is loaded with the act 'something.json',
        And a placement marker 'npc_goes_here' at 0,0,
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then the NPC 'Name' is at tile 0, 0.

    Scenario: An NPC can move along a specified line path
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 0 steps the NPC 'Name' is at tile 0,0.
        And the next step the NPC 'Name' is at tile 1,0.
        And the next step the NPC 'Name' is at tile 2,0.
        And the next step the NPC 'Name' is at tile 2,1.
        And the next step the NPC 'Name' is at tile 1,1.
        And the next step the NPC 'Name' is at tile 2,1.
        And the next step the NPC 'Name' is at tile 3,1.
        And the next step the NPC 'Name' is at tile 4,1.
        And the next step the NPC 'Name' is at tile 4,0.
        And the next step the NPC 'Name' is at tile 3,0.

    Scenario: An NPC can move along a specified looping path
        Given the game is loaded with the act 'something.json',
        And a looping path 'npc_loop',
        And an NPC 'Name' is at tile 0,0,
        When the map cutscene is loaded,
        Then after taking 0 steps the NPC 'Name' is at tile 1,0.
        And the next step the NPC 'Name' is at tile 1,1.
        And the next step the NPC 'Name' is at tile 1,2.
        And the next step the NPC 'Name' is at tile 1,3.
        And the next step the NPC 'Name' is at tile 2,3.
        And the next step the NPC 'Name' is at tile 3,3.
        And the next step the NPC 'Name' is at tile 4,3.
        And the next step the NPC 'Name' is at tile 4,2.
        And the next step the NPC 'Name' is at tile 4,1.
        And the next step the NPC 'Name' is at tile 4,0.
        And the next step the NPC 'Name' is at tile 3,0.
        And the next step the NPC 'Name' is at tile 2,0.
        And the next step the NPC 'Name' is at tile 1,0.
