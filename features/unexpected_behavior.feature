Feature: Test client on unexpected behavior

    Background: Config client
        Given the verbose level as debug
        And the log file as "unexpected_client.log"

    @fixture.use_server_mock
    Scenario: Test server close socket before client quit
        When the client is running
        And we sleep for 0.02
        Then the client is still running
        When we stop the server mock
        When the client send the command "list"
        And we sleep for 0.02
        Then the client shouldn't have paniced
