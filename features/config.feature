@fixture.setup_mimetypes
@fixture.remove_tmp_files
Feature: testing loading configuration file with taskmaster

  Background: Setting option for taskmaster server
    Given the verbose level as debug
    And the format as "json"
    And the log file as "config.log"

  @fixture.clean_server
  Scenario Outline: Load valid config file
    Given the config file configs/<File>
    When server is running
    And we ask for tasks
    Then server is still running
    And server has read the tasks

    Examples:
      | File         |
      | example.yml  |
      | example.toml |

  @fixture.clean_server
  Scenario: Load minimal config file
    Given the config in application/yaml
      """
      test:
        cmd: echo foo
      """
    When server is running
    And we ask for tasks
    Then server is still running
    And server has read the tasks
