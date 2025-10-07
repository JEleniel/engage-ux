# Unit Test Generation Guidelines

You are a skilled, experienced test author. Strictly follow these rules when generating tests:

- Tests should be grouped following the best practices of the language.
- Use _clear_, concise, descriptive test names that specify the Unit Under Test (UUT), what the test scenario is, and what behavior is expected, e.g. `add_overflow_returns_error` where `add` is the UUT, `overflow` the scenarion, and `returns_error` is the expected behavior.
- Mock or stub any external resources; keep testing focused on the unit under test only.
- Use helper methods instead of Setup and Teardown.
- Include regression tests as appropriate.
- Target 90% test coverage, focusing on the more complex units first.
- Do not put logic in tests.

## Approach

Follow the "Arrange, Act, Assert" pattern:

- Arrange your objects, create, and configure them as necessary
- Act on an object
- Assert that something is as expected

## Characteristics of good unit tests

There are several important characteristics that define a good unit test:

- Fast: Unit tests should take little time to run.
- Isolated: Unit tests are standalone, can run in isolation, and have no dependencies on any outside factors, such as a file system or database.
- Deterministic: Running a unit test should be consistent with its results. The test always returns the same result if you don't change anything in between runs.
- Self-Checking: The test should automatically detect if it passed or failed without any human interaction.
- Smiple: A unit test shouldn't take a disproportionately long time to write compared to the code being tested.
