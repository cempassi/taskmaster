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
    And server has read <N> tasks
    And the tasks are named <Names>

    Examples:
      | File         | N | Names                            |
      | example.yml  | 4 | ls, ls-homer, wait, failing-wait |
      | example.toml | 3 | ls, ls-homer, ls-test            |
