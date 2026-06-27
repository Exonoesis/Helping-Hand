Feature: NPC Movements

    ##################################################################################
    #                        Grid = Pixel Coordinate Tests                           #
    ##################################################################################

    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates at placement
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'Simple Line' is loaded,
        And 0 steps have taken place,
        Then the NPC 'Iye' has pixel coordinates equivalent to tile 0,1,1.
        And the NPC 'Iye" has grid coordinates set to tile 1,0,1.

    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates after one step
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'Simple Line' is loaded,
        And 1 steps have taken place,
        Then the NPC 'Iye' has pixel coordinates equivalent to tile 0,2,1.
        And the NPC 'Iye" has grid coordinates set to tile 2,0,1.

    Scenario: An NPC's location in pixel coordinates is equivalent to grid coordinates after two steps
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'Simple Line' is loaded,
        And 2 steps have taken place,
        Then the NPC 'Iye' has pixel coordinates equivalent to tile 0,3,1.
        And the NPC 'Iye" has grid coordinates set to tile 3,0,1.

    ##################################################################################
    #                               Placement Tests                                  #
    ##################################################################################

    Scenario: An NPC can be placed in a specified location
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Placement' is loaded,
        Then the NPC 'Iye' is at tile 0, 0.

    ##################################################################################
    #                               Line Path Tests                                  #
    ##################################################################################

    Scenario: An NPC can move right along a specified complex line path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Path' is loaded,
        And 2 steps have taken place,
        Then the NPC 'Iye' is at tile 2,0.

    Scenario: An NPC can move down along a specified complex line path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Path' is loaded,
        And 3 steps have taken place,
        Then the NPC 'Iye' is at tile 2,1.

    Scenario: An NPC can move left along a specified layered complex line
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Path' is loaded,
        And 4 steps have taken place,
        Then the NPC 'Iye' is at tile 1,1.

    Scenario: An NPC can move right along a specified layered complex line
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Path' is loaded,
        And 7 steps have taken place,
        Then the NPC 'Iye' is at tile 4,1.

    Scenario: An NPC can move up along a specified complex line path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Path' is loaded,
        And 8 steps have taken place,
        Then the NPC 'Iye' is at tile 4,0.

    Scenario: An NPC can move left along a specified complex line path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Path' is loaded,
        And 9 steps have taken place,
        Then the NPC 'Iye' is at tile 3,0.

    ##################################################################################
    #                             Looping Path Tests                                 #
    ##################################################################################

    Scenario: An NPC can move down along a specified looping path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Loop' is loaded,
        And 3 steps have taken place,
        Then the NPC 'Iye' is at tile 1,3.

    Scenario: An NPC can move right along a specified looping path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Loop' is loaded,
        And 6 steps have taken place,
        Then the NPC 'Iye' is at tile 4,3.

    Scenario: An NPC can move up along a specified looping path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Loop' is loaded,
        And 9 steps have taken place,
        Then the NPC 'Iye' is at tile 4,0.

    Scenario: An NPC can move left along a specified looping path
        Given the game is loaded with the act 'npc_movement_act.json',
        When the map cutscene 'NPC Loop' is loaded,
        And 12 steps have taken place,
        Then the NPC 'Iye' is at tile 1,0.
