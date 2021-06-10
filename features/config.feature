@fixture.setup_mimetypes
Feature: testing loading configuration file with taskmaster

  Background: Setting option for taskmaster server
    Given the verbose level as debug
    And the format as "json"

  @fixture.clean_server
  Scenario Outline: Load valid config file
    Given the config file configs/<File>
    When server is running
    And we ask for tasks
    Then server is still running
    And server has read the good amount of tasks
    And server has read the named tasks

    Examples:
      | File         |
      | example.yml  |
      | example.toml |
