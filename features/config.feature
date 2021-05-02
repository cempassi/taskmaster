Feature: testing loading configuration file with taskmaster

  Background: Setting option for taskmaster server
    Given the verbose level as DEBUG

  Scenario Outline: Load valid yaml config file
    Given the config file <File> in YAML
    When server is running
    Then it shouldn't have stopped
    And he has read <N> tasks
    And the tasks are named <Names>

    Examples:
      | File        | N | Names                 |
      | example.yml | 3 | ls, ls homer, ls test |

  Scenario Outline: Load valid toml config file
    Given the config file <File> in TOML
    When server is running
    Then it shouldn't have stopped
    And he has read <N> tasks
    And the task are named <Names>

    Examples:
      | File         | N | Names                 |
      | example.toml | 3 | ls, ls homer, ls test |
