@fixture.setup_mimetypes
@fixture.remove_tmp_files
@fixture.use_client_mock
Feature: testing loading configuration file with taskmaster

  Background: Setting option for taskmaster server
    Given the verbose level as debug
    And the format as "json"
    And the log file as "config.log"

  @fixture.clean_server
  Scenario Outline: Load valid config file
    Given the config file configs/<File>
    When the server is running
    And we ask for tasks
    Then the server is still running
    And the server has read the tasks

    Examples:
      | File         |
      | example.yml  |
      | example.toml |

  @fixture.clean_server
  Scenario: Load minimal config file
    Given the config in "application/yaml"
      """
      test:
        cmd: echo foo
      """
    When the server is running
    And we ask for tasks
    Then the server is still running
    And the server has read the tasks

  @fixture.clean_server
  Scenario: Test deleted task from config file
    Given the config in "application/yaml"
      """
      test:
        cmd: echo foo
      rm:
        cmd: echo bar
      """
    When the server is running
    And we ask for tasks
    Then the server has read the tasks
    When we edit the current config file with
      """
      test:
        cmd: echo foo
      """
    And we ask to reload the config
    And we ask for tasks
    Then the server has read the tasks
