Feature: Interactives
    Scenario: Interactive markers are being found from a Tiled map.
        Given a Tiled map called object_test.tmx,
        When the Tiled map is loaded,
        And an Interactive Collection is extracted from the Tiled map,
        Then there are 2 interactive markers in the collection.
        And marker 1 has a position of 0,0,2.
        And marker 1 has a size of 64x64.
        And marker 2 has a position of 64,128,2.
        And marker 2 has a size of 128x64.
