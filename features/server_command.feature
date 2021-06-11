# USING: Gherkin v6
@wip
@fixture.setup_mimetypes
@fixture.remove_tmp_files
Feature: Test server command on basic config

    Test various server command

    Background: Setting option for taskmaster server
        Given the verbose level as debug
        And the format as "json"
        And the log file as "server_command.log"
        Given the config in "application/yaml"
            """
            test:
                cmd: echo foo
                stdout: /tmp/echo-test.out
            """

    @fixture.clean_server
    Scenario: Test list command
        When the server is running
        And we ask for tasks
        Then the server has read the tasks

    @fixture.clean_server
    Scenario: Test start command
        When the server is running
        And we ask to start "test"
        And we ask the status of "test"
        Then the status of "test" is "active"

    @fixture.clean_server
    Scenario: Test stop command
        When the server is running
        And we ask to start "test"
        And we ask to stop "test"
        And we ask the status of "test"
        Then the status of "test" is "stopped"

    @fixture.clean_server
    Scenario: Test info command
        When the server is running
        And we ask the info about "test"
        Then the server sent the info about "test"

    @fixture.clean_server
    Scenario: Test reload command
        When the server is running
        And we add the following to the current config file
            """
            test1:
                cmd: echo bar
                stdout: /tmp/echo-test1.out
            """
        And we ask to reload the config
        Then the server has read the tasks

    @fixture.clean_server
    Scenario: Test status command
        When the server is running
        And we ask the status of "test"
        Then the status of "test" is "inactive"

    @fixture.clean_server
    Scenario: Test restart command


    @fixture.clean_server
    Scenario: Test quit command
        When the server is running
        And we ask to stop the server
        Then the server is stopped
