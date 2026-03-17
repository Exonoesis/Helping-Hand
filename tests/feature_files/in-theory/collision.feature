Feature: Collision
    Scenario: Collision tiles should be invisible.
        Given a Tiled map called collision_test.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        Then there are 4 collision tiles in the rendered map.
        And there are 3 layers in the rendered map.
        And rendered tile 1,0,2 is labeled as a collision tile.
        And rendered tile 1,0,2 is invisible.
        And rendered tile 0,1,2 is labeled as a collision tile.
        And rendered tile 0,1,2 is invisible.
        And rendered tile 2,1,2 is labeled as a collision tile.
        And rendered tile 2,1,2 is invisible.
        And rendered tile 1,2,2 is labeled as a collision tile.
        And rendered tile 1,2,2 is invisible.

    Scenario: Collision tiles are being found.
        Given a Tiled map called collision_test.tmx,
        When the Tiled map is loaded,
        And the Tiled map has been converted to a rendered map,
        And the collision tiles are collected,
        Then tile 1,0,2 is a collision tile in the collection.
        And tile 0,1,2 is a collision tile in the collection.
        And tile 2,1,2 is a collision tile in the collection.
        And tile 1,2,2 is a collision tile in the collection.
