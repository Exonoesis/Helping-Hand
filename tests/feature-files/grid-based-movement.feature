Feature: Grid-based Movement
    Scenario: A player moves to the right
        Given a Map,
        And a Player on a Map on the center of Tile A,
        When the Player is requested to move to the right,
        Then the Player should be on the center of Tile B.