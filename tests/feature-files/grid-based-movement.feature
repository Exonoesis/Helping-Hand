Feature: Grid-based Movement
    Scenario: A player moves to the right
        Given a Tiled map called player.tmx,
        And the player is at 0,1,1,
        When the Tiled map is loaded,
        And the Player is requested to move to the right,
        Then the Player's pixel coordinates are equivalent to tile 1,1,1.