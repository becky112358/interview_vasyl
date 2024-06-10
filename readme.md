# Micropelt Interview Tasks

* Please spend around 1 hour working on one or both of the following tasks.
* It's preferrable to have one fully completed task than to have two partially-completed tasks.
* Please code your answer in Rust.

## Task

We have an event scheduler which has a time of day and days of the week on which the event reccurs. Implement a function which, given a Schedule, returns the next date and time at which the event will occur.

You can use as much or as little of this Cargo package as you like.

## Task

We have textboxes into which users type numbers. The textboxes call a function to ensure that users only enter valid numbers. The function returns `true` for valid input and `false` for invalid input. Write this function, to ensure that the user only enters valid numbers:

* A number in the range 0.0 to 80.0, ending in .0 or .5
    + Examples of valid inputs:
        - `2`
        - `4.` (because the user types character-by-character)
        - `8.5`
    + Examples of invalid inputs:
        - `-1`
        - `9.3`
        - `80.5`
* A number in the range 0.0 to 40.0, ending in .0, .25, .5, or .75
    + Examples of valid inputs:
        - `4.`
        - `8.75`
        - `9`
    + Examples of invalid inputs:
        - `-1`
        - `9.3`
        - `93`
