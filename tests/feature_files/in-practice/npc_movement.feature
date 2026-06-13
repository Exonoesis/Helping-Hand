Feature: NPC Movements

    ##################################################################################
    #                        Grid = Pixel Coordinate Tests                           #
    ##################################################################################

    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates at placement
        Given the game is loaded with the act 'something.json',
        And a line path 'simple_line',
        And an NPC 'Name' is at tile 0,0,1,
        When the map cutscene is loaded,
        Then after taking 0 steps the NPC 'Name' has pixel coordinates equivalent to tile 0,1,1.
        And the NPC 'Name" has grid coordinates set to tile 0,1,1.

    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates after one step
        Given the game is loaded with the act 'something.json',
        And a line path 'simple_line',
        And an NPC 'Name' is at tile 0,0,1,
        When the map cutscene is loaded,
        Then after taking 1 step the NPC 'Name' has pixel coordinates equivalent to tile 0,2,1.
        And the NPC 'Name" has grid coordinates set to tile 0,2,1.

    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates after two steps
        Given the game is loaded with the act 'something.json',
        And a line path 'simple_line',
        And an NPC 'Name' is at tile 0,0,1,
        When the map cutscene is loaded,
        Then after taking 2 steps the NPC 'Name' has pixel coordinates equivalent to tile 0,3,1.
        And the NPC 'Name" has grid coordinates set to tile 0,3,1.

    ##################################################################################
    #                               Placement Tests                                  #
    ##################################################################################

    Scenario: An NPC can be placed in a specified location
        Given the game is loaded with the act 'something.json',
        And a placement marker 'npc_goes_here' at 0,0,
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then the NPC 'Name' is at tile 0, 0.

    ##################################################################################
    #                               Line Path Tests                                  #
    ##################################################################################

    Scenario: An NPC can move right along a specified complex line path
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 2 steps the NPC 'Name' is at tile 2,0.

    Scenario: An NPC can move down along a specified complex line path
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 3 steps the NPC 'Name' is at tile 2,1.

    Scenario: An NPC can move left along a specified layered complex line
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 4 steps the NPC 'Name' is at tile 1,1.

    Scenario: An NPC can move right along a specified layered complex line
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 7 steps the NPC 'Name' is at tile 4,1.

    Scenario: An NPC can move up along a specified complex line path
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 8 steps the NPC 'Name' is at tile 4,0.

    Scenario: An NPC can move left along a specified complex line path
        Given the game is loaded with the act 'something.json',
        And a line path 'npc_path',
        And an NPC 'Name' is at tile 0,1,
        When the map cutscene is loaded,
        Then after taking 9 steps the NPC 'Name' is at tile 3,0.

    ##################################################################################
    #                             Looping Path Tests                                 #
    ##################################################################################

    Scenario: An NPC can move down along a specified looping path
        Given the game is loaded with the act 'something.json',
        And a looping path 'npc_loop',
        And an NPC 'Name' is at tile 0,0,
        When the map cutscene is loaded,
        Then after taking 3 steps the NPC 'Name' is at tile 1,3.

    Scenario: An NPC can move right along a specified looping path
        Given the game is loaded with the act 'something.json',
        And a looping path 'npc_loop',
        And an NPC 'Name' is at tile 0,0,
        When the map cutscene is loaded,
        Then after taking 6 steps the NPC 'Name' is at tile 4,3.

    Scenario: An NPC can move up along a specified looping path
        Given the game is loaded with the act 'something.json',
        And a looping path 'npc_loop',
        And an NPC 'Name' is at tile 0,0,
        When the map cutscene is loaded,
        Then after taking 9 steps the NPC 'Name' is at tile 4,0.

    Scenario: An NPC can move left along a specified looping path
        Given the game is loaded with the act 'something.json',
        And a looping path 'npc_loop',
        And an NPC 'Name' is at tile 0,0,
        When the map cutscene is loaded,
        Then after taking 12 steps the NPC 'Name' is at tile 1,0.
