Feature: testing loading configuration file with taskmaster

  Background: Setting option for taskmaster server
    Given the verbose level as debug
    And we remove previous unix socket

  @fixture.clean_server
  Scenario Outline: Load valid yaml config file
    Given the config file configs/<File> in YAML
    When server is running
    Then server is still running
    And server has read <N> tasks
    And the tasks are named <Names>

    Examples:
      | File        | N | Names                 |
      | example.yml | 3 | ls, ls homer, ls test |

  @fixture.clean_server
  Scenario Outline: Load valid toml config file
    Given the config file configs/<File> in TOML
    When server is running
    Then server is still running
    And server has read <N> tasks
    And the tasks are named <Names>

    Examples:
      | File         | N | Names                 |
      | example.toml | 3 | ls, ls homer, ls test |
