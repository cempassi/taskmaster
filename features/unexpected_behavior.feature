Feature: Test client on unexpected behavior

    @wip
    @fixture.use_server_mock
    Scenario: Test server close socket before client quit
        When the client is running
        And we close the socket
        Then the client shouldn't have paniced
