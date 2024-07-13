Feature: Load Tilemap from Ldtk.
    Scenario: All tiles are loaded in a grid.
        Given a LDtk map called test_map.ldtk,
        When the LDtk map is loaded,
        Then there are 4x4 (16) tiles loaded in a grid.