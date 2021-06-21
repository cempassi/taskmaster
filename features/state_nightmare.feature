@fixture.setup_mimetypes
@fixture.remove_tmp_files
@fixture.use_client_mock
Feature: Test task's state nightmare

  Background: Setting option for taskmaster server
    Given the verbose level as debug
    And the format as "json"
    And the log file as "state_nightmare.log"

  @fixture.clean_server
  Scenario: Test autostart on reloading task
    Given the config in "application/yaml"
      """
      test:
        cmd: echo foo
        autostart: false
      """
    When the server is running
    And we ask the status of "test"
    Then the status of "test" is "Inactive"
    When we edit the current config file with
      """
      test:
        cmd: echo foo
        autostart: true
      """
    And we ask to reload the config
    And we sleep for 0.2
    And we ask the status of "test"
    Then the status of "test" is one of
      | status   |
      | Finished |
      | Active   |
