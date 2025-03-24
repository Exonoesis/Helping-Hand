Feature: Map Changing
    Scenario: A player transitions to a different map
        Given a Tiled map called transition_test.tmx,
        And a map size of 3 x 3 tiles,
        And the Player is at 2,2,
        When the player interacts with the tile ahead of them,
        Then the map size should be 16 x 10 tiles,
        And the Player should be at 7,9.

    Scenario: A player doesn't transition to a different map
        Given a Tiled map called no_transition_test.tmx,
        And a map size of 3 x 3 tiles,
        And the Player is at 2,1,
        When the player interacts with the tile ahead of them,
        Then the map size should be 3 x 3 tiles,
        And the Player should be at 2,1.
