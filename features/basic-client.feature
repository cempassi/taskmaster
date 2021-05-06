Feature: Test Client with basic interaction

  Scenario: Check help command
    Given the client is running
    # When we skip current output
    # And we write help
    When we write help
    Then client is still running
    Then we read the help command output
