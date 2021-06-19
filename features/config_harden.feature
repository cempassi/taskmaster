@fixture.setup_mimetypes
@fixture.remove_tmp_files
Feature: testing loading configuration with tricky use

  Background: Setting option for taskmaster server
    Given the verbose level as debug
    And the format as "json"
    And the log file as "config_harden.log"

  @fixture.clean_server
  Scenario: Load config with duplicated name
    Given the config in "application/yaml"
      """
      test:
        cmd: echo foo
      test:
        cmd: echo bar
      """
    When the server is running
    And we sleep for 0.2
    Then the server is still running
