Feature: Test Client with basic interaction

  Scenario: Check help command
    Given the client is running
    When we write help
    Then we read the help command output
