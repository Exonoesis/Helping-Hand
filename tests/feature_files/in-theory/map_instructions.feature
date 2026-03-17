Feature: Get map instruction information from a Tiled map
    Scenario: Character and Placement Location Coordinates are both found.
        # ASSUMPTION: We got a placement instruction from an Arcweave file.
        Given a Tiled map called 'path_place.tmx',
        Given the placement instruction of Jay at PlayerStart,
        When we extract a Map Coordinator from the Tiled Map,
        # A Map Coordinator will take in a MapInstruction::Place(Character, Location)
        When we give the instruction to the Map Coordinator,
        # KEY: Different types of results can be returned. This can be PlacementResult
        Then the placement result should have the character at coordinates 1, 1.
        And the placement result should have the location at coordinates 2, 2.
