Feature: Test Client with basic interaction

  Background: Run server
    Given we remove previous unix socket
    And server is running

  Scenario: Check help command
    Given the client is running
    When we skip current output
    And we write help
    # When we write help
    Then client is still running
    And we read the help command output
