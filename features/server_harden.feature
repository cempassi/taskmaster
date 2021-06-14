@fixture.setup_mimetypes
@fixture.remove_tmp_files
@fixture.use_client_mock
Feature: Test server on malicious use

    Background: Setting option for taskmaster server
        Given the verbose level as debug
        And the format as "json"
        And the log file as "server_hardening.log"
        Given the config in "application/yaml"
            """
            test:
                cmd: echo foo
                stdout: /tmp/echo-test.out
            """

    @fixture.clean_server
    Scenario: Test start on unknown task name
        When the server is running
        And we ask to start "foo"
        And we sleep for 0.2
        Then the server is still running

    @fixture.clean_server
    Scenario: Test stop on unknown task name
        When the server is running
        And we ask to start "foo"
        And we sleep for 0.2
        Then the server is still running

    @fixture.clean_server
    Scenario: Test info on unknown task name
        When the server is running
        And we ask the info about "foo"
        And we sleep for 0.2
        Then the server is still running

    @fixture.clean_server
    Scenario: Test restart on unknown task name
        When the server is running
        And we ask to restart "foo"
        And we sleep for 0.2
        Then the server is still running

    @fixture.clean_server
    Scenario: Test status on unknown task name
        When the server is running
        And we ask the status of "foo"
        And we sleep for 0.2
        Then the server is still running
