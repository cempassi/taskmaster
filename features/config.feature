Feature: testing loading configuration file with taskmaster

  Scenario Outline: Load valid yaml config file
    Given the config file <File> in YAML
    When taskmaster is running
    Then it shouldn't have stopped
    And he has read <N> tasks
    And the tasks are named <Names>

    Examples:
      | File    | N | Names                 |
      | example | 3 | ls, ls homer, ls test |

  Scenario Outline: Load valid toml config file
    Given the config file <File> in TOML
    When taskmaster is running
    Then it shouldn't have stopped
    And he has read <N> tasks
    And the task are named <Names>

    Examples:
      | File    | N | Names                 |
      | example | 3 | ls, ls homer, ls test |
